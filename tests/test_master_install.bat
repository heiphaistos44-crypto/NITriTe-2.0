@echo off
:: ============================================================
::  NITRITE v6.0 - Test Master Install (installation silencieuse)
::  Installe REELLEMENT les applications selectionnees via winget.
::  ATTENTION : ce script installe les logiciels !
::  Lancer en tant qu'Administrateur.
:: ============================================================
title Nitrite - Master Install (Installation silencieuse)
color 0E

echo ============================================================
echo  NITRITE v6.0 - MASTER INSTALL - Installation Silencieuse
echo ============================================================
echo.
echo  ATTENTION : Ce script va installer les logiciels suivants
echo  de maniere silencieuse via WinGet.
echo.
echo  Logiciels qui seront installes :
echo    Navigateurs  : Chrome, Firefox, Brave
echo    Bureautique  : LibreOffice, Notepad++, Obsidian
echo    Dev          : VS Code, Git, Node.js LTS, Python 3.12
echo    Multimedia   : VLC, OBS Studio, Audacity, HandBrake
echo    Communication: Discord, Telegram
echo    Securite     : Bitwarden, KeePass, VeraCrypt
echo    Systeme      : 7-Zip, PowerToys, Everything, CPU-Z
echo    Reseau       : WinSCP, PuTTY, FileZilla
echo    Gaming       : Steam
echo.
set /p CONFIRM="Confirmer l'installation ? (O/N) : "
if /i not "%CONFIRM%"=="O" (
  echo Installation annulee.
  pause & exit /b 0
)

set PASS=0
set FAIL=0
set LOG=%~dp0test_master_install_result.log
echo [%date% %time%] Debut Master Install > "%LOG%"

:: --- Verifier winget ---
winget --version >nul 2>&1
if errorlevel 1 (
  echo [ERREUR] WinGet non disponible.
  pause & exit /b 1
)

echo.
echo --- NAVIGATEURS ---
CALL :INSTALL "Google Chrome"           "Google.Chrome"
CALL :INSTALL "Mozilla Firefox"         "Mozilla.Firefox"
CALL :INSTALL "Brave"                   "Brave.Brave"

echo.
echo --- BUREAUTIQUE ---
CALL :INSTALL "LibreOffice"             "TheDocumentFoundation.LibreOffice"
CALL :INSTALL "Notepad++"               "Notepad++.Notepad++"
CALL :INSTALL "Obsidian"                "Obsidian.Obsidian"

echo.
echo --- DEVELOPPEMENT ---
CALL :INSTALL "Visual Studio Code"      "Microsoft.VisualStudioCode"
CALL :INSTALL "Git"                     "Git.Git"
CALL :INSTALL "Node.js LTS"             "OpenJS.NodeJS.LTS"
CALL :INSTALL "Python 3.12"             "Python.Python.3.12"

echo.
echo --- MULTIMEDIA ---
CALL :INSTALL "VLC"                     "VideoLAN.VLC"
CALL :INSTALL "OBS Studio"              "OBSProject.OBSStudio"
CALL :INSTALL "Audacity"                "Audacity.Audacity"
CALL :INSTALL "HandBrake"               "HandBrake.HandBrake"

echo.
echo --- COMMUNICATION ---
CALL :INSTALL "Discord"                 "Discord.Discord"
CALL :INSTALL "Telegram"                "Telegram.TelegramDesktop"

echo.
echo --- SECURITE ---
CALL :INSTALL "Bitwarden"               "Bitwarden.Bitwarden"
CALL :INSTALL "KeePass"                 "DominikReichl.KeePass"
CALL :INSTALL "VeraCrypt"               "IDRIX.VeraCrypt"

echo.
echo --- SYSTEME ---
CALL :INSTALL "7-Zip"                   "7zip.7zip"
CALL :INSTALL "PowerToys"               "Microsoft.PowerToys"
CALL :INSTALL "Everything"              "voidtools.Everything"
CALL :INSTALL "CPU-Z"                   "CPUID.CPU-Z"
CALL :INSTALL "CrystalDiskInfo"         "CrystalDewWorld.CrystalDiskInfo"
CALL :INSTALL "GPU-Z"                   "TechPowerUp.GPU-Z"
CALL :INSTALL "Rufus"                   "Rufus.Rufus"

echo.
echo --- RESEAU ---
CALL :INSTALL "WinSCP"                  "WinSCP.WinSCP"
CALL :INSTALL "PuTTY"                   "PuTTY.PuTTY"
CALL :INSTALL "FileZilla"               "TimKosse.FileZilla.Client"

echo.
echo --- GAMING ---
CALL :INSTALL "Steam"                   "Valve.Steam"

:: --- Bilan ---
echo.
echo ============================================================
echo  BILAN: %PASS% installes  |  %FAIL% echecs
echo ============================================================
echo [%date% %time%] Bilan: %PASS% OK / %FAIL% ECHEC >> "%LOG%"
echo  Log: %LOG%
echo.
pause
exit /b 0

:: ============================================================
:INSTALL
set APP_NAME=%~1
set WINGET_ID=%~2
echo [....] Installation de %APP_NAME% (%WINGET_ID%)...
winget install --id %WINGET_ID% --silent --accept-package-agreements --accept-source-agreements
if errorlevel 1 (
  echo [FAIL] %APP_NAME% - Echec ou deja installe
  echo [FAIL] %APP_NAME% (%WINGET_ID%) >> "%LOG%"
  set /a FAIL+=1
) else (
  echo [OK]   %APP_NAME% - Installe avec succes
  echo [OK]   %APP_NAME% (%WINGET_ID%) >> "%LOG%"
  set /a PASS+=1
)
echo.
exit /b 0
