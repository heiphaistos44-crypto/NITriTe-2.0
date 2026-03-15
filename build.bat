@echo off
setlocal EnableDelayedExpansion
title Nitrite Build v26.44.0
echo ============================================================
echo   Nitrite 2.0 - Build Production v26.44.0
echo   (UAC: requireAdministrator)
echo ============================================================
echo.
cd /d "%~dp0"

set VERSION=26.44.0
set EXE_SRC=src-tauri\target\release\nitrite.exe
set OUT_DIR=release
set MAKENSIS=%LOCALAPPDATA%\tauri\NSIS\makensis.exe
set NSIS_INCLUDE=%LOCALAPPDATA%\tauri\NSIS\Include

:: ============================================================
:: MENU
:: ============================================================
echo Choisissez le mode de packaging :
echo.
echo   [1] EXE Portable seul (~15 MB)
echo       release\Nitrite_v%VERSION%_portable.exe
echo       (application uniquement, sans logiciel/drivers/scripts)
echo.
echo   [2] Dossier portable complet (~2.5 GB)
echo       release\Nitrite_v%VERSION%\
echo       Contient : Nitrite.exe + logiciel + Drivers + Script Windows
echo.
echo   [3] SFX tout-en-un (~1.5 GB compresse)
echo       release\Nitrite_v%VERSION%_full.exe
echo       Un seul .exe qui extrait tout et lance Nitrite
echo.
set /p CHOICE="Votre choix [1/2/3] : "
echo.

if "%CHOICE%"=="1" goto MODE_EXE_SEUL
if "%CHOICE%"=="2" goto MODE_DOSSIER
if "%CHOICE%"=="3" goto MODE_SFX
echo Choix invalide. Relancez le script.
pause & exit /b 1

:: ============================================================
:: ETAPES COMMUNES
:: ============================================================
:BUILD_COMMON
echo --- Arret des instances en cours...
taskkill /F /IM nitrite.exe >nul 2>&1
timeout /t 1 /nobreak >nul

echo --- Verification des dependances npm...
if not exist "node_modules" (
    call npm install
    if %ERRORLEVEL% neq 0 ( echo [ERREUR] npm install. & pause & exit /b 1 )
) else (
    echo      node_modules present.
)

echo --- Verification TypeScript (729 apps portables + catalogue complet)...
call npx tsc --noEmit
if %ERRORLEVEL% neq 0 ( echo [ERREUR] Erreurs TypeScript. & pause & exit /b 1 )

echo --- Preparation dossier release...
if exist "%OUT_DIR%" rmdir /s /q "%OUT_DIR%"
mkdir "%OUT_DIR%"

echo --- Build Tauri Release (Vue + Rust + manifests)...
call npx tauri build --no-bundle
if %ERRORLEVEL% neq 0 ( echo [ERREUR] Build Tauri echoue. & pause & exit /b 1 )
if not exist "%EXE_SRC%" ( echo [ERREUR] EXE non genere. & pause & exit /b 1 )

goto %BUILD_RETURN%

:: ============================================================
:: MODE 1 - EXE portable seul
:: ============================================================
:MODE_EXE_SEUL
set BUILD_RETURN=AFTER_EXE_SEUL
goto BUILD_COMMON
:AFTER_EXE_SEUL
set DEST=%OUT_DIR%\Nitrite_v%VERSION%_portable.exe
copy /Y "%EXE_SRC%" "%DEST%" >nul
echo.
echo ============================================================
echo   [1] Build termine !
echo   Fichier : %DEST%
for %%F in ("%DEST%") do echo   Taille  : %%~zF octets
echo ============================================================
echo.
pause & exit /b 0

:: ============================================================
:: MODE 2 - Dossier portable complet
:: ============================================================
:MODE_DOSSIER
set BUILD_RETURN=AFTER_DOSSIER
goto BUILD_COMMON
:AFTER_DOSSIER
set DEST_DIR=%OUT_DIR%\Nitrite_v%VERSION%
mkdir "%DEST_DIR%"
echo --- Copie de l'executable...
copy /Y "%EXE_SRC%" "%DEST_DIR%\Nitrite.exe" >nul
echo --- Copie de logiciel\ (peut prendre du temps)...
xcopy /E /I /Q /Y "logiciel" "%DEST_DIR%\logiciel" >nul
echo --- Copie de Drivers\...
xcopy /E /I /Q /Y "Drivers" "%DEST_DIR%\Drivers" >nul
echo --- Copie de Script Windows\...
xcopy /E /I /Q /Y "Script Windows" "%DEST_DIR%\Script Windows" >nul
echo.
echo ============================================================
echo   [2] Build termine !
echo   Dossier : %DEST_DIR%\
echo     Nitrite.exe
echo     logiciel\
echo     Drivers\
echo     Script Windows\
echo ============================================================
echo.
pause & exit /b 0

:: ============================================================
:: MODE 3 - SFX tout-en-un via 7-Zip SFX
:: ============================================================
:MODE_SFX
:: Recherche 7-Zip (64-bit requis pour >2GB)
set SEVENZIP=
if exist "C:\Program Files\7-Zip\7z.exe"       set SEVENZIP=C:\Program Files\7-Zip\7z.exe
if exist "C:\Program Files (x86)\7-Zip\7z.exe" set SEVENZIP=C:\Program Files (x86)\7-Zip\7z.exe
if not defined SEVENZIP (
    echo [ERREUR] 7-Zip introuvable.
    echo Installez 7-Zip 64-bit depuis https://www.7-zip.org/
    pause & exit /b 1
)
for %%D in ("%SEVENZIP%") do set SEVENZIP_DIR=%%~dpD

:: Recherche du stub SFX (7zSD.sfx = GUI avec choix dossier, 7zS.sfx = console, 7z.sfx = GUI simple)
set SFX_STUB=
if exist "%SEVENZIP_DIR%7zSD.sfx" set SFX_STUB=%SEVENZIP_DIR%7zSD.sfx
if exist "%SEVENZIP_DIR%7zS.sfx"  set SFX_STUB=%SEVENZIP_DIR%7zS.sfx
if exist "%SEVENZIP_DIR%7z.sfx"   set SFX_STUB=%SEVENZIP_DIR%7z.sfx
if exist "%~dp0tools\7zSD.sfx"    set SFX_STUB=%~dp0tools\7zSD.sfx
if exist "%~dp0tools\7zS.sfx"     set SFX_STUB=%~dp0tools\7zS.sfx
if exist "%~dp0tools\7z.sfx"      set SFX_STUB=%~dp0tools\7z.sfx
if defined SFX_STUB goto :SFX_STUB_FOUND

:: Telechargement automatique via subroutine (hors bloc if, evite problemes expansion)
echo --- 7zSD.sfx introuvable. Telechargement du package 7-Zip Extra...
"%SEVENZIP%" i > "%TEMP%\_7zver.txt" 2>nul
set SZVER=
for /f "tokens=2" %%V in ('type "%TEMP%\_7zver.txt" ^| findstr /B /C:"7-Zip "') do (
    if not defined SZVER set SZVER=%%V
)
del /Q "%TEMP%\_7zver.txt" >nul 2>&1
if not defined SZVER (
    echo [ERREUR] Impossible de determiner la version de 7-Zip.
    pause & exit /b 1
)
:: Convertit "24.09" -> "2409"
set SZVER_URL=!SZVER:.=!
echo     Version detectee : !SZVER! -- URL: 7z!SZVER_URL!-extra.7z
if not exist "%~dp0tools" mkdir "%~dp0tools"
set EXTRA_DL=%~dp0tools\_extra.7z
set EXTRA_OUT=%~dp0tools
call :SFX_DOWNLOAD
if exist "%~dp0tools\7zSD.sfx" set SFX_STUB=%~dp0tools\7zSD.sfx
if exist "%~dp0tools\7zS.sfx"  set SFX_STUB=%~dp0tools\7zS.sfx
if exist "%~dp0tools\7z.sfx"   set SFX_STUB=%~dp0tools\7z.sfx
if defined SFX_STUB (
    echo     SFX stub telecharge dans tools\
    goto :SFX_STUB_FOUND
)
echo [ERREUR] Echec du telechargement automatique.
echo.
echo Solution manuelle :
echo   1. Allez sur https://www.7-zip.org/download.html
echo   2. Telechargez "7-Zip Extra" ^(en bas de la page^)
echo   3. Extrayez 7zSD.sfx dans le dossier "tools\" du projet
pause & exit /b 1

:SFX_DOWNLOAD
:: TLS 1.2 obligatoire + download visible pour diagnostiquer les erreurs reseau
echo     Telechargement en cours...
powershell -NoProfile -Command "[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12; Invoke-WebRequest 'https://www.7-zip.org/a/7z%SZVER_URL%-extra.7z' -OutFile '%EXTRA_DL%' -UseBasicParsing"
if exist "%EXTRA_DL%" goto :SFX_EXTRACT
:: Fallback version stable 23.01
echo     Fallback sur la version stable 23.01...
powershell -NoProfile -Command "[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12; Invoke-WebRequest 'https://www.7-zip.org/a/7z2301-extra.7z' -OutFile '%EXTRA_DL%' -UseBasicParsing"
if exist "%EXTRA_DL%" goto :SFX_EXTRACT
exit /b 0
:SFX_EXTRACT
:: Extraction recursive (le fichier peut etre dans un sous-dossier de l'archive)
"%SEVENZIP%" e "%EXTRA_DL%" 7zSD.sfx -o"%EXTRA_OUT%" -y -r
del /Q "%EXTRA_DL%" >nul 2>&1
exit /b 0

:SFX_STUB_FOUND

set BUILD_RETURN=AFTER_SFX
goto BUILD_COMMON
:AFTER_SFX

set TEMP_SFX=%OUT_DIR%\_sfx_temp
set ARCHIVE=%OUT_DIR%\_nitrite.7z
set CFG_FILE=%OUT_DIR%\_sfx_config.txt
set SFX_OUT=%OUT_DIR%\Nitrite_v%VERSION%_full.exe

:: Preparation du dossier temporaire
echo --- Preparation du contenu SFX...
if exist "%TEMP_SFX%" rmdir /s /q "%TEMP_SFX%"
mkdir "%TEMP_SFX%"
copy /Y "%EXE_SRC%" "%TEMP_SFX%\Nitrite.exe" >nul
echo --- Copie logiciel\ ...
xcopy /E /I /Q /Y "logiciel" "%TEMP_SFX%\logiciel" >nul
echo --- Copie Drivers\ ...
xcopy /E /I /Q /Y "Drivers" "%TEMP_SFX%\Drivers" >nul
echo --- Copie Script Windows\ ...
xcopy /E /I /Q /Y "Script Windows" "%TEMP_SFX%\Script Windows" >nul

:: Fichier de configuration SFX 7-Zip
(
echo ;!@Install@!UTF-8!
echo Title="Nitrite v%VERSION%"
echo BeginPrompt="Voulez-vous extraire Nitrite v%VERSION% ?"
echo RunProgram="Nitrite.exe"
echo ;!@InstallEnd@!
) > "%CFG_FILE%"

:: Compression 7-Zip (LZMA2, multi-thread, niveau 5 = equilibre vitesse/taille)
echo --- Compression 7-Zip LZMA2 (peut prendre 10-20 min selon le CPU)...
echo     Source : ~2.4 GB non compresse
"%SEVENZIP%" a -t7z -m0=lzma2 -mx=5 -mmt=on -ms=on "%ARCHIVE%" "%TEMP_SFX%\*" -y
if %ERRORLEVEL% neq 0 (
    echo [ERREUR] Compression 7-Zip echouee.
    rmdir /s /q "%TEMP_SFX%" >nul 2>&1
    del /Q "%CFG_FILE%" >nul 2>&1
    pause & exit /b 1
)

:: Assemblage SFX : stub + config + archive
echo --- Assemblage SFX...
copy /b "%SFX_STUB%" + "%CFG_FILE%" + "%ARCHIVE%" "%SFX_OUT%" >nul
if %ERRORLEVEL% neq 0 (
    echo [ERREUR] Assemblage SFX echoue.
    rmdir /s /q "%TEMP_SFX%" >nul 2>&1
    del /Q "%ARCHIVE%" >nul 2>&1
    del /Q "%CFG_FILE%" >nul 2>&1
    pause & exit /b 1
)

:: Nettoyage
rmdir /s /q "%TEMP_SFX%" >nul 2>&1
del /Q "%ARCHIVE%" >nul 2>&1
del /Q "%CFG_FILE%" >nul 2>&1

if not exist "%SFX_OUT%" (
    echo [ERREUR] SFX non genere.
    pause & exit /b 1
)

echo.
echo ============================================================
echo   [3] Build termine !
echo   SFX : %SFX_OUT%
for %%F in ("%SFX_OUT%") do echo   Taille : %%~zF octets
echo.
echo   Au lancement :
echo     1. Cliquez sur "Oui" pour extraire
echo     2. Extraction automatique de tout le contenu
echo     3. Nitrite se lance automatiquement
echo ============================================================
echo.
pause & exit /b 0
