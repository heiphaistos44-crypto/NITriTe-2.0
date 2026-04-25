@echo off
:: ============================================================
::  NITRITE v6.0 - Test Apps Portables (accessibilite URLs)
::  Verifie que les pages de telechargement des apps portables
::  sont accessibles (HTTP 200). Necessite une connexion internet.
::  Utilise PowerShell Invoke-WebRequest.
:: ============================================================
title Nitrite - Test Portable Apps URLs
color 09

set PASS=0
set FAIL=0
set LOG=%~dp0test_portable_apps_result.log

echo [%date% %time%] Debut test Apps Portables > "%LOG%"
echo.
echo ============================================================
echo  NITRITE v6.0 - TEST ACCESSIBILITE APPS PORTABLES
echo ============================================================
echo  Verification de l'accessibilite des sites de telechargement.
echo  Connexion internet requise.
echo.

:: Verifier PowerShell
powershell -Command "exit 0" >nul 2>&1
if errorlevel 1 (
  echo [ERREUR] PowerShell non disponible.
  pause & exit /b 1
)

:: ============================================================
echo [SECTION] SYSTEME
:: ============================================================
CALL :CHECK "CPU-Z"               "https://www.cpuid.com/softwares/cpu-z.html"
CALL :CHECK "GPU-Z"               "https://www.techpowerup.com/gpuz/"
CALL :CHECK "HWMonitor"           "https://www.cpuid.com/softwares/hwmonitor.html"
CALL :CHECK "HWiNFO64"            "https://www.hwinfo.com/download/"
CALL :CHECK "CrystalDiskInfo"     "https://crystalmark.info/en/software/crystaldiskinfo/"
CALL :CHECK "CrystalDiskMark"     "https://crystalmark.info/en/software/crystaldiskmark/"
CALL :CHECK "Process Explorer"    "https://learn.microsoft.com/sysinternals/downloads/process-explorer"
CALL :CHECK "Autoruns"            "https://learn.microsoft.com/sysinternals/downloads/autoruns"
CALL :CHECK "Rufus"               "https://rufus.ie/"
CALL :CHECK "Ventoy"              "https://www.ventoy.net/en/download.html"
CALL :CHECK "DDU"                 "https://www.guru3d.com/download/display-driver-uninstaller-download/"
CALL :CHECK "NVCleanstall"        "https://www.techpowerup.com/nvcleanstall/"

echo.
echo [SECTION] RESEAU
:: ============================================================
CALL :CHECK "Wireshark"           "https://www.wireshark.org/download.html"
CALL :CHECK "Nmap"                "https://nmap.org/download.html"
CALL :CHECK "PuTTY"               "https://portableapps.com/apps/internet/putty_portable"
CALL :CHECK "WinSCP"              "https://portableapps.com/apps/internet/winscp_portable"
CALL :CHECK "FileZilla"           "https://portableapps.com/apps/internet/filezilla_portable"
CALL :CHECK "mRemoteNG"           "https://mremoteng.org/download"
CALL :CHECK "TCPView"             "https://learn.microsoft.com/sysinternals/downloads/tcpview"
CALL :CHECK "Angry IP Scanner"    "https://angryip.org/download/"

echo.
echo [SECTION] DEVELOPPEMENT
:: ============================================================
CALL :CHECK "VS Code"             "https://code.visualstudio.com/Download"
CALL :CHECK "Notepad++"           "https://portableapps.com/apps/development/notepadplusplus_portable"
CALL :CHECK "Sublime Text"        "https://www.sublimetext.com/download_thanks?target=win-x64"
CALL :CHECK "Git for Windows"     "https://git-scm.com/download/win"
CALL :CHECK "GitHub Desktop"      "https://desktop.github.com/"
CALL :CHECK "Postman"             "https://www.postman.com/downloads/"
CALL :CHECK "DBeaver"             "https://dbeaver.io/download/"
CALL :CHECK "HeidiSQL"            "https://portableapps.com/apps/development/heidisql_portable"
CALL :CHECK "Node.js"             "https://nodejs.org/en/download/"

echo.
echo [SECTION] UTILITAIRES
:: ============================================================
CALL :CHECK "7-Zip"               "https://portableapps.com/apps/utilities/7-zip_portable"
CALL :CHECK "PeaZip"              "https://portableapps.com/apps/utilities/peazip_portable"
CALL :CHECK "KeePass"             "https://portableapps.com/apps/security/keepass_portable"
CALL :CHECK "Greenshot"           "https://portableapps.com/apps/graphics_pictures/greenshot_portable"
CALL :CHECK "ShareX"              "https://getsharex.com/"
CALL :CHECK "Recuva"              "https://portableapps.com/apps/utilities/recuva_portable"
CALL :CHECK "TestDisk"            "https://www.cgsecurity.org/wiki/TestDisk_Download"
CALL :CHECK "Bulk Rename Utility" "https://www.bulkrenameutility.co.uk/Download.php"
CALL :CHECK "HashMyFiles"         "https://www.nirsoft.net/utils/hash_my_files.html"

echo.
echo [SECTION] MULTIMEDIA
:: ============================================================
CALL :CHECK "VLC"                 "https://portableapps.com/apps/music_video/vlc_portable"
CALL :CHECK "MPC-HC"              "https://github.com/clsid2/mpc-hc/releases"
CALL :CHECK "foobar2000"          "https://portableapps.com/apps/music_video/foobar2000_portable"
CALL :CHECK "Audacity"            "https://portableapps.com/apps/music_video/audacity_portable"
CALL :CHECK "HandBrake"           "https://portableapps.com/apps/music_video/handbrake_portable"
CALL :CHECK "OBS Studio"          "https://obsproject.com/fr/download"
CALL :CHECK "yt-dlp"              "https://github.com/yt-dlp/yt-dlp/releases"
CALL :CHECK "FFmpeg"              "https://ffmpeg.org/download.html"
CALL :CHECK "MediaInfo"           "https://mediaarea.net/fr/MediaInfo/Download/Windows"
CALL :CHECK "Mp3tag"              "https://portableapps.com/apps/music_video/mp3tag_portable"

echo.
echo [SECTION] BUREAUTIQUE
:: ============================================================
CALL :CHECK "LibreOffice"         "https://portableapps.com/apps/office/libreoffice_portable"
CALL :CHECK "SumatraPDF"          "https://portableapps.com/apps/office/sumatra_pdf_portable"
CALL :CHECK "Joplin"              "https://joplinapp.org/download/"
CALL :CHECK "Obsidian"            "https://obsidian.md/download"
CALL :CHECK "CherryTree"          "https://portableapps.com/apps/office/cherrytree_portable"
CALL :CHECK "Calibre"             "https://portableapps.com/apps/office/calibre_portable"

echo.
echo [SECTION] NETTOYAGE
:: ============================================================
CALL :CHECK "BleachBit"           "https://portableapps.com/apps/utilities/bleachbit_portable"
CALL :CHECK "CCleaner"            "https://portableapps.com/apps/utilities/ccleaner_portable"
CALL :CHECK "Wise Care 365"       "https://portableapps.com/apps/utilities/wise_care_365_portable"
CALL :CHECK "PrivaZer"            "https://privazer.com/fr/download.php"
CALL :CHECK "Revo Uninstaller"    "https://portableapps.com/apps/utilities/revo_uninstaller_portable"
CALL :CHECK "AdwCleaner"          "https://www.malwarebytes.com/adwcleaner"

echo.
echo [SECTION] SECURITE
:: ============================================================
CALL :CHECK "Malwarebytes"        "https://www.malwarebytes.com/mwb-download/thankyou"
CALL :CHECK "Kaspersky KVRT"      "https://www.kaspersky.fr/downloads/free-virus-removal-tool"
CALL :CHECK "VeraCrypt"           "https://www.veracrypt.fr/en/Downloads.html"
CALL :CHECK "KeePassXC"           "https://keepassxc.org/download/"
CALL :CHECK "Bitwarden"           "https://bitwarden.com/download/"
CALL :CHECK "Process Hacker"      "https://processhacker.sourceforge.io/downloads.php"

echo.
echo [SECTION] GRAPHISME
:: ============================================================
CALL :CHECK "GIMP"                "https://portableapps.com/apps/graphics_pictures/gimp_portable"
CALL :CHECK "Inkscape"            "https://portableapps.com/apps/graphics_pictures/inkscape_portable"
CALL :CHECK "IrfanView"           "https://portableapps.com/apps/graphics_pictures/irfanview_portable"
CALL :CHECK "Krita"               "https://krita.org/fr/telechargement/"
CALL :CHECK "Paint.NET"           "https://www.getpaint.net/download.html"
CALL :CHECK "Blender"             "https://www.blender.org/download/"
CALL :CHECK "Figma Desktop"       "https://www.figma.com/downloads/"
CALL :CHECK "XnView MP"           "https://www.xnview.com/fr/xnviewmp/"

echo.
echo [SECTION] RECUPERATION
:: ============================================================
CALL :CHECK "MiniTool Partition"  "https://www.partitionwizard.com/free-partition-manager.html"
CALL :CHECK "GetDataBack"         "https://www.runtime.org/data-recovery-software.htm"
CALL :CHECK "HDDScan"             "https://hddscan.com/"
CALL :CHECK "Victoria"            "https://hdd.by/victoria/"
CALL :CHECK "Hiren's BootCD"      "https://www.hirensbootcd.org/download/"

:: --- Bilan ---
echo.
echo ============================================================
echo  BILAN: %PASS% OK  |  %FAIL% inaccessible(s)
echo ============================================================
echo [%date% %time%] Bilan: %PASS% OK / %FAIL% FAIL >> "%LOG%"
if %FAIL% GTR 0 (
  echo  Les URLs en echec sont peut-etre temporairement indisponibles
  echo  ou ont change. Consultez le log pour les details.
)
echo  Log: %LOG%
echo.
pause
exit /b 0

:: ============================================================
:CHECK
:: %1=label, %2=URL
set _LABEL=%~1
set _URL=%~2
<nul set /p "=[TEST] %-32s  %_URL%  ... " "%_LABEL%"
powershell -NoProfile -Command ^
  "try { $r=(Invoke-WebRequest -Uri '%_URL%' -UseBasicParsing -TimeoutSec 8 -Method Head -ErrorAction Stop).StatusCode; if($r -lt 400){exit 0}else{exit 1} } catch { exit 1 }" >nul 2>&1
if errorlevel 1 (
  echo [FAIL]
  echo [FAIL] %_LABEL%: %_URL% >> "%LOG%"
  set /a FAIL+=1
) else (
  echo [OK]
  echo [OK]   %_LABEL%: %_URL% >> "%LOG%"
  set /a PASS+=1
)
exit /b 0
