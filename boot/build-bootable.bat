@echo off
setlocal EnableDelayedExpansion
title Nitrite - Build ISO WinPE
echo ============================================================
echo   Nitrite 2.0 - Build ISO WinPE Bootable
echo   Necessite : Windows ADK + WinPE Add-on installes
echo ============================================================
echo.

set "BOOT_DIR=%~dp0"
set "PROJ_ROOT=%~dp0.."
for /f "delims=" %%V in ('powershell -NoProfile -Command "(Get-Content \"%~dp0..\package.json\" -Raw | ConvertFrom-Json).version"') do set "VERSION=%%V"
if not defined VERSION set "VERSION=unknown"
set "NITRITE_EXE=%PROJ_ROOT%\src-tauri\target\release\nitrite.exe"
set "WEBVIEW2_SRC=%BOOT_DIR%webview2-fixed-runtime"
set "WORK_DIR=%TEMP%\nitrite_winpe_work"
set "MOUNT_DIR=%TEMP%\nitrite_winpe_mount"
set "ISO_OUT=%PROJ_ROOT%\release\Nitrite_v!VERSION!_WinPE.iso"
set "SYS32=C:\Windows\System32"

if not exist "%PROJ_ROOT%\release" mkdir "%PROJ_ROOT%\release"

:: ── ADK ──────────────────────────────────────────────────────────────────────
set "ADK_PATH="
if exist "C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit" (
    set "ADK_PATH=C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit"
)
if exist "C:\Program Files\Windows Kits\10\Assessment and Deployment Kit" (
    set "ADK_PATH=C:\Program Files\Windows Kits\10\Assessment and Deployment Kit"
)
if not defined ADK_PATH (
    echo [ERREUR] Windows ADK introuvable.
    pause & exit /b 1
)

set "WINPE_ADK=%ADK_PATH%\Windows Preinstallation Environment"
set "PKG_BASE=%WINPE_ADK%\amd64\WinPE_OCs"
set "OSCDIMG=%ADK_PATH%\Deployment Tools\x86\Oscdimg\oscdimg.exe"

:: DISM systeme (x64) pour eviter les problemes de compatibilite avec le WIM amd64
set "DISM_EXE=%SYS32%\dism.exe"

if not exist "%WINPE_ADK%" (
    echo [ERREUR] WinPE Add-on manquant. Installez "Windows PE Add-on for the Windows ADK"
    pause & exit /b 1
)
if not exist "%OSCDIMG%" (
    echo [ERREUR] oscdimg.exe introuvable.
    pause & exit /b 1
)
if not exist "%NITRITE_EXE%" (
    echo [ERREUR] nitrite.exe introuvable : %NITRITE_EXE%
    echo Compilez d'abord avec build.bat
    pause & exit /b 1
)

:: ── Nettoyage ────────────────────────────────────────────────────────────────
echo --- Nettoyage...
if exist "%WORK_DIR%" rmdir /s /q "%WORK_DIR%"
if exist "%MOUNT_DIR%" rmdir /s /q "%MOUNT_DIR%"

:: ── Init ADK + copype ────────────────────────────────────────────────────────
call "%ADK_PATH%\Deployment Tools\DandISetEnv.bat" >nul 2>&1
cd /d "%PROJ_ROOT%"
echo --- Creation workspace WinPE...
call "%WINPE_ADK%\copype.cmd" amd64 "%WORK_DIR%"
if %ERRORLEVEL% neq 0 (
    echo [ERREUR] copype.cmd a echoue.
    pause & exit /b 1
)

:: ── Montage WIM ──────────────────────────────────────────────────────────────
echo --- Montage WIM...
mkdir "%MOUNT_DIR%"
"%DISM_EXE%" /Mount-Image /ImageFile:"%WORK_DIR%\media\sources\boot.wim" /Index:1 /MountDir:"%MOUNT_DIR%"
if %ERRORLEVEL% neq 0 (
    echo [ERREUR] Montage WIM echoue.
    rmdir /s /q "%MOUNT_DIR%"
    pause & exit /b 1
)

:: ── Packages WinPE (sans NetFX - trop lourd et inutile) ─────────────────────
echo --- Ajout packages WinPE...
for %%P in (WinPE-WMI WinPE-Scripting WinPE-PowerShell) do (
    if exist "%PKG_BASE%\%%P.cab" (
        "%DISM_EXE%" /Image:"%MOUNT_DIR%" /Add-Package /PackagePath:"%PKG_BASE%\%%P.cab" >nul 2>&1
        echo     %%P ajout.
    )
)

:: ── Drivers ──────────────────────────────────────────────────────────────────
if exist "%PROJ_ROOT%\Drivers" (
    echo --- Injection drivers...
    "%DISM_EXE%" /Image:"%MOUNT_DIR%" /Add-Driver /Driver:"%PROJ_ROOT%\Drivers" /Recurse /ForceUnsigned >nul 2>&1
    echo     Drivers injectes.
)

:: ── Structure profil utilisateur ─────────────────────────────────────────────
echo --- Profil utilisateur...
mkdir "%MOUNT_DIR%\Users\Default\AppData\Local" 2>nul
mkdir "%MOUNT_DIR%\Users\Default\AppData\Roaming" 2>nul
mkdir "%MOUNT_DIR%\Users\Default\Desktop" 2>nul
mkdir "%MOUNT_DIR%\Users\Public" 2>nul
mkdir "%MOUNT_DIR%\ProgramData" 2>nul
mkdir "%MOUNT_DIR%\Temp" 2>nul

:: ── Nitrite.exe ───────────────────────────────────────────────────────────────
echo --- Copie Nitrite...
mkdir "%MOUNT_DIR%\Nitrite" 2>nul
copy /Y "%NITRITE_EXE%" "%MOUNT_DIR%\Nitrite\nitrite.exe" >nul

:: ── VC++ Runtime (dans Nitrite\ pour isolation) ───────────────────────────────
echo --- VC++ Runtime...
for %%D in (
    VCRUNTIME140.dll VCRUNTIME140_1.dll MSVCP140.dll ucrtbase.dll
    api-ms-win-crt-runtime-l1-1-0.dll api-ms-win-crt-stdio-l1-1-0.dll
    api-ms-win-crt-string-l1-1-0.dll api-ms-win-crt-heap-l1-1-0.dll
    api-ms-win-crt-math-l1-1-0.dll api-ms-win-crt-locale-l1-1-0.dll
    api-ms-win-crt-convert-l1-1-0.dll dwmapi.dll pdh.dll
    dbghelp.dll powrprof.dll uxtheme.dll
) do (
    if exist "%SYS32%\%%D" copy /Y "%SYS32%\%%D" "%MOUNT_DIR%\Nitrite\%%D" >nul 2>&1
)
echo     VC++ Runtime copie.

:: ── Explorer Shell (DLLs SUPPLEMENTAIRES uniquement) ─────────────────────────
:: IMPORTANT : ne pas ecraser shell32.dll, shlwapi.dll, shcore.dll, comctl32.dll,
::             comdlg32.dll, xmllite.dll — ces fichiers sont deja dans WinPE
::             et les versions Win11 ont des dependances absentes dans WinPE.
echo --- Explorer Shell (DLLs supplementaires)...
copy /Y "C:\Windows\explorer.exe" "%MOUNT_DIR%\Windows\explorer.exe" >nul 2>&1
for %%D in (
    ExplorerFrame.dll
    windows.storage.dll
    twinapi.appcore.dll
    twinapi.dll
    twinui.dll
    twinui.pcshell.dll
    dui70.dll
    AppResolver.dll
    propsys.dll
    StructuredQuery.dll
    dsreg.dll
    Winlangdb.dll
    iertutil.dll
    urlmon.dll
    stobject.dll
    SLC.dll
    NInput.dll
    CoreMessaging.dll
    SystemEventsBrokerClient.dll
    Windows.Storage.ApplicationData.dll
    wlanapi.dll
    wintypes.dll
    cscapi.dll
    ntshrui.dll
    actxprxy.dll
    es.dll
    InputSwitch.dll
) do (
    if exist "%SYS32%\%%D" copy /Y "%SYS32%\%%D" "%MOUNT_DIR%\Windows\System32\%%D" >nul 2>&1
)
echo     Explorer Shell copie.

:: Raccourci bureau Nitrite (pointe vers X:\ runtime)
powershell -NoProfile -Command "$s=(New-Object -COM WScript.Shell).CreateShortcut('%MOUNT_DIR%\Users\Default\Desktop\Nitrite.lnk');$s.TargetPath='X:\Nitrite\nitrite.exe';$s.WorkingDirectory='X:\Nitrite';$s.Save()" >nul 2>&1
echo     Raccourci Nitrite.lnk cree.

:: ── startnet.cmd + winpeshl.ini ───────────────────────────────────────────────
echo --- Scripts boot...
copy /Y "%BOOT_DIR%startnet.cmd" "%MOUNT_DIR%\Windows\System32\startnet.cmd" >nul
:: winpeshl.ini intentionnellement absent = WinPE utilise startnet.cmd automatiquement
del /f /q "%MOUNT_DIR%\Windows\System32\winpeshl.ini" >nul 2>&1

:: ── Registre SYSTEM (variables d'environnement) ───────────────────────────────
echo --- Registre...
reg load HKLM\WINPE_SYSTEM "%MOUNT_DIR%\Windows\System32\config\SYSTEM" >nul 2>&1
reg add "HKLM\WINPE_SYSTEM\ControlSet001\Control\Session Manager\Environment" /v "LOCALAPPDATA" /t REG_SZ /d "X:\Users\Default\AppData\Local" /f >nul 2>&1
reg add "HKLM\WINPE_SYSTEM\ControlSet001\Control\Session Manager\Environment" /v "APPDATA"      /t REG_SZ /d "X:\Users\Default\AppData\Roaming" /f >nul 2>&1
reg add "HKLM\WINPE_SYSTEM\ControlSet001\Control\Session Manager\Environment" /v "USERPROFILE"  /t REG_SZ /d "X:\Users\Default" /f >nul 2>&1
reg add "HKLM\WINPE_SYSTEM\ControlSet001\Control\Session Manager\Environment" /v "TEMP"         /t REG_SZ /d "X:\Temp" /f >nul 2>&1
reg add "HKLM\WINPE_SYSTEM\ControlSet001\Control\Session Manager\Environment" /v "TMP"          /t REG_SZ /d "X:\Temp" /f >nul 2>&1
reg unload HKLM\WINPE_SYSTEM >nul 2>&1
echo     Registre configure.

:: ── Demontage WIM ─────────────────────────────────────────────────────────────
echo --- Demontage WIM...
"%DISM_EXE%" /Unmount-Image /MountDir:"%MOUNT_DIR%" /Commit
if %ERRORLEVEL% neq 0 (
    echo [ERREUR] Demontage echoue.
    "%DISM_EXE%" /Unmount-Image /MountDir:"%MOUNT_DIR%" /Discard >nul 2>&1
    pause & exit /b 1
)
rmdir /s /q "%MOUNT_DIR%"

:: ── WebView2 dans media ISO (hors WIM) ───────────────────────────────────────
if exist "%WEBVIEW2_SRC%" (
    echo --- Copie WebView2 dans ISO - hors WIM, 625MB...
    mkdir "%WORK_DIR%\media\WebView2" 2>nul
    robocopy "%WEBVIEW2_SRC%" "%WORK_DIR%\media\WebView2" /E /NFL /NDL /NJH /NJS /NC /NS /NP >nul 2>&1
    echo     WebView2 copie.
) else (
    echo [WARN] WebView2 absent, Nitrite ne pourra pas afficher l'UI.
)

:: ── Generation ISO via MakeWinPEMedia.cmd (outil officiel Microsoft) ───────────
echo --- Generation ISO bootable...
set "ISO_TEMP=%TEMP%\Nitrite_WinPE_temp.iso"
if exist "%ISO_TEMP%" del /f /q "%ISO_TEMP%"

call "%WINPE_ADK%\MakeWinPEMedia.cmd" /ISO "%WORK_DIR%" "%ISO_TEMP%"
if %ERRORLEVEL% neq 0 (
    echo [ERREUR] MakeWinPEMedia a echoue.
    rmdir /s /q "%WORK_DIR%"
    pause & exit /b 1
)

:: ── Finalisation ──────────────────────────────────────────────────────────────
echo --- Nettoyage temporaires...
rmdir /s /q "%WORK_DIR%"

if exist "%ISO_OUT%" del /f /q "%ISO_OUT%"
move /y "%ISO_TEMP%" "%ISO_OUT%" >nul 2>&1
if not exist "%ISO_OUT%" (
    echo [WARN] Impossible de deplacer l'ISO, disponible dans : %ISO_TEMP%
    set "ISO_OUT=%ISO_TEMP%"
)

echo.
echo ============================================================
echo   ISO genere avec succes !
echo.
for %%F in ("%ISO_OUT%") do echo   Fichier : %%~fF
for %%F in ("%ISO_OUT%") do echo   Taille  : %%~zF octets
echo ============================================================
echo.
pause & exit /b 0
