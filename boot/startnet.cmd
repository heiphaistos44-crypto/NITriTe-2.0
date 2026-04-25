@echo off
setlocal EnableDelayedExpansion

:: ── Init WinPE (réseau, PnP, drivers) ────────────────────────────────────────
wpeinit

:: ── Log de démarrage ─────────────────────────────────────────────────────────
set "LOG=X:\nitrite_boot.log"
echo [%date% %time%] BOOT START > "%LOG%"

:: ── Variables d'environnement de base ────────────────────────────────────────
set "LOCALAPPDATA=X:\Users\Default\AppData\Local"
set "APPDATA=X:\Users\Default\AppData\Roaming"
set "USERPROFILE=X:\Users\Default"
set "TEMP=X:\Temp"
set "TMP=X:\Temp"
set "PROGRAMDATA=X:\ProgramData"
set "PUBLIC=X:\Users\Public"

:: ── Créer les dossiers de profil ─────────────────────────────────────────────
mkdir "X:\ProgramData" 2>nul
mkdir "X:\Users\Public" 2>nul
mkdir "X:\Users\Default\Desktop" 2>nul
mkdir "X:\Users\Default\AppData\Local\Microsoft\Windows" 2>nul
mkdir "X:\Users\Default\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup" 2>nul

:: ── Attendre que les drives soient initialises par wpeinit ───────────────────
ping -n 5 127.0.0.1 >nul

:: ── Détecter WebView2 (cherche msedgewebview2.exe directement) ───────────────
set "WEBVIEW2_BROWSER_EXECUTABLE_FOLDER=X:\Nitrite\WebView2"
set "ISO_DRIVE="
set "VENTOY_DRIVE="

for %%D in (B C D E F G H I J K L M N O P Q R S T U V W X) do (
    if exist "%%D:\WebView2\msedgewebview2.exe" (
        set "ISO_DRIVE=%%D:"
        set "WEBVIEW2_BROWSER_EXECUTABLE_FOLDER=%%D:\WebView2"
    )
)

if defined ISO_DRIVE (
    echo [INFO] WebView2 detecte sur !ISO_DRIVE! >> "%LOG%"
    echo [INFO] WebView2 sur !ISO_DRIVE!\WebView2

    :: Simuler installation WebView2 user-level
    :: WebView2Loader.dll cherche : %LOCALAPPDATA%\Microsoft\EdgeWebView\Application\<version>\
    md "X:\Users\Default\AppData\Local\Microsoft\EdgeWebView\Application" 2>nul
    mklink /J "X:\Users\Default\AppData\Local\Microsoft\EdgeWebView\Application\146.0.3856.72" "!WEBVIEW2_BROWSER_EXECUTABLE_FOLDER!" >nul 2>&1

    if exist "X:\Users\Default\AppData\Local\Microsoft\EdgeWebView\Application\146.0.3856.72\msedgewebview2.exe" (
        echo [OK] Installation WebView2 simulee dans LOCALAPPDATA >> "%LOG%"
    ) else (
        echo [WARN] Simulation WebView2 echec >> "%LOG%"
    )
) else (
    echo [WARN] WebView2 non detecte sur les drives >> "%LOG%"
    echo [WARN] WebView2 non detecte
)

echo [INFO] WEBVIEW2_PATH=!WEBVIEW2_BROWSER_EXECUTABLE_FOLDER! >> "%LOG%"

:: Lister les drives pour debug
echo [DEBUG] Drives disponibles : >> "%LOG%"
for %%D in (B C D E F G H I J K L M N O P Q R S T U V W X) do (
    if exist "%%D:\" echo   %%D: existe >> "%LOG%"
)

:: ── Détecter Ventoy ──────────────────────────────────────────────────────────
for %%D in (B C D E F G H I J K L M N O P Q R S T U V W) do (
    if exist "%%D:\ventoy" set "VENTOY_DRIVE=%%D:"
)

if defined VENTOY_DRIVE (
    echo [INFO] Ventoy sur !VENTOY_DRIVE! >> "%LOG%"
    if exist "!VENTOY_DRIVE!\Apps" (
        powershell -NoProfile -Command "$s=(New-Object -COM WScript.Shell).CreateShortcut('X:\Users\Default\Desktop\Applications.lnk');$s.TargetPath='!VENTOY_DRIVE!\Apps';$s.Description='Applications portables';$s.Save()" 2>nul
        echo [INFO] Raccourci Applications cree >> "%LOG%"
    )
)

:: ── Raccourci bureau Nitrite ──────────────────────────────────────────────────
powershell -NoProfile -Command "$s=(New-Object -COM WScript.Shell).CreateShortcut('X:\Users\Default\Desktop\Nitrite.lnk');$s.TargetPath='X:\Nitrite\nitrite.exe';$s.WorkingDirectory='X:\Nitrite';$s.Description='Nitrite - Diagnostic Windows';$s.Save()" 2>nul

:: ── Lancer Explorer comme shell ───────────────────────────────────────────────
echo [INFO] Lancement Explorer... >> "%LOG%"
start /b explorer.exe
ping -n 6 127.0.0.1 >nul

:: ── Enregistrer WebView2 Fixed Runtime via politique registre ────────────────
:: WebView2 SDK lit HKLM\...\BrowserExecutableFolder en PRIORITE ABSOLUE
:: (avant env var, avant parametre API) → fonctionne même avec Tauri/wry
reg add "HKLM\Software\Policies\Microsoft\Edge\WebView2\BrowserExecutableFolder" /v "*" /t REG_SZ /d "!WEBVIEW2_BROWSER_EXECUTABLE_FOLDER!" /f >nul 2>&1
echo [INFO] Registre WebView2 configure : !WEBVIEW2_BROWSER_EXECUTABLE_FOLDER! >> "%LOG%"

:: Verifier que msedgewebview2.exe est accessible
if exist "!WEBVIEW2_BROWSER_EXECUTABLE_FOLDER!\msedgewebview2.exe" (
    echo [OK] msedgewebview2.exe trouve >> "%LOG%"
) else (
    echo [ERREUR] msedgewebview2.exe INTROUVABLE dans !WEBVIEW2_BROWSER_EXECUTABLE_FOLDER! >> "%LOG%"
)

:: ── Lancer Nitrite ────────────────────────────────────────────────────────────
echo [INFO] Lancement Nitrite... >> "%LOG%"
start "" "X:\Nitrite\nitrite.exe"

echo [INFO] BOOT END >> "%LOG%"
