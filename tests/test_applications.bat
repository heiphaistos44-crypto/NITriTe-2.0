@echo off
:: ============================================================
::  NITRITE v6.0 - Test Applications (winget availability)
::  Verifie que chaque application est disponible via winget.
::  Ne procede PAS a l'installation reelle.
::  Lancer en tant qu'Administrateur pour de meilleurs resultats.
:: ============================================================
title Nitrite - Test Applications (Winget Check)
color 0A

set PASS=0
set FAIL=0
set LOG=%~dp0test_applications_result.log

echo [%date% %time%] Debut test Applications > "%LOG%"
echo.
echo ============================================================
echo  NITRITE - TEST DISPONIBILITE APPLICATIONS (WinGet)
echo ============================================================
echo.

:: --- Verifier winget ---
winget --version >nul 2>&1
if errorlevel 1 (
  echo [ERREUR] WinGet n'est pas installe ou non accessible.
  echo [ERREUR] WinGet non disponible >> "%LOG%"
  pause & exit /b 1
)
echo [OK] WinGet detecte.
echo.

:: --- Fonction de test (winget show) ---
:: Format: CALL :CHECK "Nom App" "WinGet.ID"

CALL :CHECK "Google Chrome"                 "Google.Chrome"
CALL :CHECK "Mozilla Firefox"              "Mozilla.Firefox"
CALL :CHECK "Brave"                        "Brave.Brave"
CALL :CHECK "Microsoft Edge"               "Microsoft.Edge"
CALL :CHECK "Opera GX"                     "Opera.OperaGX"
CALL :CHECK "LibreOffice"                  "TheDocumentFoundation.LibreOffice"
CALL :CHECK "Notepad++"                    "Notepad++.Notepad++"
CALL :CHECK "Obsidian"                     "Obsidian.Obsidian"
CALL :CHECK "Notion"                       "Notion.Notion"
CALL :CHECK "Visual Studio Code"           "Microsoft.VisualStudioCode"
CALL :CHECK "Git"                          "Git.Git"
CALL :CHECK "Node.js LTS"                  "OpenJS.NodeJS.LTS"
CALL :CHECK "Python 3.12"                  "Python.Python.3.12"
CALL :CHECK "Docker Desktop"               "Docker.DockerDesktop"
CALL :CHECK "Postman"                      "Postman.Postman"
CALL :CHECK "VLC"                          "VideoLAN.VLC"
CALL :CHECK "OBS Studio"                   "OBSProject.OBSStudio"
CALL :CHECK "GIMP"                         "GIMP.GIMP"
CALL :CHECK "Audacity"                     "Audacity.Audacity"
CALL :CHECK "Spotify"                      "Spotify.Spotify"
CALL :CHECK "HandBrake"                    "HandBrake.HandBrake"
CALL :CHECK "Discord"                      "Discord.Discord"
CALL :CHECK "Zoom"                         "Zoom.Zoom"
CALL :CHECK "Slack"                        "SlackTechnologies.Slack"
CALL :CHECK "Microsoft Teams"              "Microsoft.Teams"
CALL :CHECK "Telegram"                     "Telegram.TelegramDesktop"
CALL :CHECK "Malwarebytes"                 "Malwarebytes.Malwarebytes"
CALL :CHECK "Bitwarden"                    "Bitwarden.Bitwarden"
CALL :CHECK "KeePass"                      "DominikReichl.KeePass"
CALL :CHECK "ProtonVPN"                    "ProtonTechnologies.ProtonVPN"
CALL :CHECK "VeraCrypt"                    "IDRIX.VeraCrypt"
CALL :CHECK "GlassWire"                    "GlassWire.GlassWire"
CALL :CHECK "7-Zip"                        "7zip.7zip"
CALL :CHECK "CPU-Z"                        "CPUID.CPU-Z"
CALL :CHECK "HWiNFO64"                     "REALiX.HWiNFO"
CALL :CHECK "CrystalDiskInfo"              "CrystalDewWorld.CrystalDiskInfo"
CALL :CHECK "Autoruns"                     "Microsoft.Sysinternals.Autoruns"
CALL :CHECK "Process Explorer"             "Microsoft.Sysinternals.ProcessExplorer"
CALL :CHECK "WinDirStat"                   "WinDirStat.WinDirStat"
CALL :CHECK "Everything"                   "voidtools.Everything"
CALL :CHECK "TreeSize Free"                "JAMSoftware.TreeSize.Free"
CALL :CHECK "PowerToys"                    "Microsoft.PowerToys"
CALL :CHECK "Speccy"                       "Piriform.Speccy"
CALL :CHECK "GPU-Z"                        "TechPowerUp.GPU-Z"
CALL :CHECK "Rufus"                        "Rufus.Rufus"
CALL :CHECK "Ventoy"                       "Ventoy.Ventoy"
CALL :CHECK "ShutUp10++"                   "OO-Software.ShutUp10"
CALL :CHECK "WinSCP"                       "WinSCP.WinSCP"
CALL :CHECK "PuTTY"                        "PuTTY.PuTTY"
CALL :CHECK "Advanced IP Scanner"          "Famatech.AdvancedIPScanner"
CALL :CHECK "Wireshark"                    "WiresharkFoundation.Wireshark"
CALL :CHECK "FileZilla"                    "TimKosse.FileZilla.Client"
CALL :CHECK "qBittorrent"                  "qBittorrent.qBittorrent"
CALL :CHECK "Free Download Manager"        "SoftdeluxeGroup.FreeDownloadManager"
CALL :CHECK "JDownloader 2"                "AppWork.JDownloader"
CALL :CHECK "mRemoteNG"                    "mRemoteNG.mRemoteNG"
CALL :CHECK "Cyberduck"                    "iterate.Cyberduck"
CALL :CHECK "Nmap"                         "Nmap.Nmap"
CALL :CHECK "Angry IP Scanner"             "AngryIPScanner.AngryIPScanner"
CALL :CHECK "Steam"                        "Valve.Steam"
CALL :CHECK "Epic Games Launcher"          "EpicGames.EpicGamesLauncher"
CALL :CHECK "GOG Galaxy"                   "GOG.Galaxy"
CALL :CHECK "Battle.net"                   "Blizzard.BattleNet"
CALL :CHECK "Ubisoft Connect"              "Ubisoft.Connect"
CALL :CHECK "EA App"                       "ElectronicArts.EADesktop"
CALL :CHECK "Xbox App"                     "Microsoft.GamingApp"
CALL :CHECK "Heroic Games Launcher"        "HeroicGamesLauncher.HeroicGamesLauncher"
CALL :CHECK "Playnite"                     "Playnite.Playnite"
CALL :CHECK "MSI Afterburner"              "Guru3D.Afterburner"
CALL :CHECK "GeForce Experience"           "Nvidia.GeForceExperience"
CALL :CHECK "Parsec"                       "Parsec.Parsec"
CALL :CHECK "Moonlight"                    "MoonlightGameStreamingProject.Moonlight"
CALL :CHECK "RetroArch"                    "Libretro.RetroArch"
CALL :CHECK "Dolphin Emulator"             "DolphinEmu.Dolphin"

:: --- Bilan ---
echo.
echo ============================================================
echo  BILAN: %PASS% OK  |  %FAIL% ECHEC
echo ============================================================
echo [%date% %time%] Bilan: %PASS% OK / %FAIL% ECHEC >> "%LOG%"
if %FAIL% GTR 0 (
  echo  ATTENTION: Certaines apps sont introuvables dans winget.
  echo  Elles peuvent avoir change d'ID ou necessiter winget upgrade.
)
echo  Log complet: %LOG%
echo.
pause
exit /b 0

:: ============================================================
:CHECK
:: %1 = nom affiche, %2 = winget ID
set APP_NAME=%~1
set WINGET_ID=%~2
<nul set /p "=[TEST] %-40s" "%APP_NAME%"
winget show --id %WINGET_ID% >nul 2>&1
if errorlevel 1 (
  echo [ECHEC]
  echo [ECHEC] %APP_NAME% (%WINGET_ID%) >> "%LOG%"
  set /a FAIL+=1
) else (
  echo [OK]
  echo [OK]    %APP_NAME% (%WINGET_ID%) >> "%LOG%"
  set /a PASS+=1
)
exit /b 0
