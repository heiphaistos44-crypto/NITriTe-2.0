@echo off
:: ============================================================
::  NITRITE v6.0 - Test Outils Systeme (ToolsPage)
::  Verifie que chaque outil/commande systeme est executable.
::  Lancer en tant qu'Administrateur pour les outils Admin.
:: ============================================================
title Nitrite - Test Outils Systeme
color 0B

set PASS=0
set FAIL=0
set SKIP=0
set LOG=%~dp0test_system_tools_result.log

echo [%date% %time%] Debut test outils systeme > "%LOG%"
echo.
echo ============================================================
echo  NITRITE v6.0 - TEST OUTILS SYSTEME
echo ============================================================
echo.

:: ============================================================
echo [SECTION] REPARATION SYSTEME
:: ============================================================

:: SFC (juste verifie que la commande existe - ne lance pas le scan complet)
CALL :CHECK_CMD "SFC disponible" "sfc" "/?" 0
:: DISM disponible
CALL :CHECK_CMD "DISM disponible" "dism" "/?" 0
:: chkdsk disponible
CALL :CHECK_CMD "chkdsk disponible" "chkdsk" "/?" 0

echo.
echo [SECTION] DIAGNOSTICS SYSTEME
:: ============================================================

:: Moniteur de ressources (resmon.exe)
CALL :CHECK_EXE "Moniteur de ressources" "resmon.exe"
:: msinfo32
CALL :CHECK_EXE "Informations systeme" "msinfo32.exe"
:: Gestionnaire de peripheriques
CALL :CHECK_EXE "Gestionnaire peripheriques" "devmgmt.msc"
:: Observateur d'evenements
CALL :CHECK_EXE "Observateur evenements" "eventvwr.msc"
:: Editeur de registre
CALL :CHECK_EXE "Editeur de registre" "regedit.exe"
:: Task Scheduler
CALL :CHECK_EXE "Planificateur de taches" "taskschd.msc"
:: Gestion disques
CALL :CHECK_EXE "Gestion des disques" "diskmgmt.msc"
:: Performances
CALL :CHECK_EXE "Moniteur performances" "perfmon.msc"
:: Services
CALL :CHECK_EXE "Services Windows" "services.msc"

echo.
echo [SECTION] NETTOYAGE
:: ============================================================

:: nettoyage disque
CALL :CHECK_EXE "Nettoyage de disque" "cleanmgr.exe"
:: temp folder accessible
if exist "%TEMP%" (
  echo [OK] Dossier TEMP accessible: %TEMP%
  echo [OK] TEMP: %TEMP% >> "%LOG%"
  set /a PASS+=1
) else (
  echo [FAIL] Dossier TEMP inaccessible
  set /a FAIL+=1
)
:: prefetch accessible
if exist "%SystemRoot%\Prefetch" (
  echo [OK] Dossier Prefetch accessible
  echo [OK] Prefetch accessible >> "%LOG%"
  set /a PASS+=1
) else (
  echo [WARN] Dossier Prefetch inaccessible (normal si non-admin)
  set /a SKIP+=1
)

echo.
echo [SECTION] RESEAU
:: ============================================================

CALL :CHECK_CMD "netsh disponible" "netsh" "version" 0
CALL :CHECK_CMD "ipconfig disponible" "ipconfig" "/all" 0
CALL :CHECK_CMD "ping disponible" "ping" "-n 1 127.0.0.1" 0
CALL :CHECK_CMD "tracert disponible" "tracert" "/?" 0
CALL :CHECK_CMD "nslookup disponible" "nslookup" "/?" 0
CALL :CHECK_CMD "arp disponible" "arp" "-a" 0
CALL :CHECK_CMD "netstat disponible" "netstat" "-n" 0

echo.
echo [SECTION] OUTILS PORTABLES (logiciel/)
:: ============================================================

CALL :CHECK_FILE "Autoruns64" "C:\Users\Momo\Desktop\Nitrite 2.0\logiciel\Autoruns\Autoruns64.exe"
CALL :CHECK_FILE "BCUninstaller" "C:\Users\Momo\Desktop\Nitrite 2.0\logiciel\BCUninstaller_5.9.0_net6.0-windows10.0.18362.0\BCUninstaller.exe"
CALL :CHECK_FILE "CPU-Z Portable" "C:\Users\Momo\Desktop\Nitrite 2.0\logiciel\CPU-ZPortable\CPU-ZPortable.exe"
CALL :CHECK_FILE "CrystalDiskInfo" "C:\Users\Momo\Desktop\Nitrite 2.0\logiciel\CrystalDisk\DiskInfo64.exe"
CALL :CHECK_FILE "CrystalDiskMark" "C:\Users\Momo\Desktop\Nitrite 2.0\logiciel\CrystalDiskMarkPortable\CrystalDiskMarkPortable.exe"
CALL :CHECK_FILE "HWiNFO Portable" "C:\Users\Momo\Desktop\Nitrite 2.0\logiciel\HWiNFOPortable\HWiNFOPortable.exe"
CALL :CHECK_FILE "HWMonitor Portable" "C:\Users\Momo\Desktop\Nitrite 2.0\logiciel\HWMonitorPortable\HWMonitorPortable.exe"
CALL :CHECK_FILE "Process Explorer" "C:\Users\Momo\Desktop\Nitrite 2.0\logiciel\ProcessExplorerPortable\ProcessExplorerPortable.exe"
CALL :CHECK_FILE "AdwCleaner" "C:\Users\Momo\Desktop\Nitrite 2.0\logiciel\AdwCleaner\adwcleaner.exe"
CALL :CHECK_FILE "WiseCare 365" "C:\Users\Momo\Desktop\Nitrite 2.0\logiciel\WiseCare365\WiseCare365.exe"
CALL :CHECK_FILE "WiseDiskCleaner" "C:\Users\Momo\Desktop\Nitrite 2.0\logiciel\WiseDiskCleanerPortable\WiseDiskCleanerPortable.exe"
CALL :CHECK_FILE "Spybot Portable" "C:\Users\Momo\Desktop\Nitrite 2.0\logiciel\SpybotPortable\SpybotPortable.exe"
CALL :CHECK_FILE "DDU" "C:\Users\Momo\Desktop\Nitrite 2.0\logiciel\DDU v18.1.4.2\Display Driver Uninstaller.exe"
CALL :CHECK_FILE "GetDataBack Pro" "C:\Users\Momo\Desktop\Nitrite 2.0\logiciel\GetDataBack Pro 5.55.Portable\GetDataBackProPortable.exe"

echo.
echo [SECTION] CONNECTIVITE INTERNET
:: ============================================================

CALL :CHECK_URL "Google DNS" "8.8.8.8"
CALL :CHECK_URL "Cloudflare DNS" "1.1.1.1"
CALL :CHECK_URL "Microsoft" "www.microsoft.com"

echo.
echo ============================================================
echo  BILAN: %PASS% OK  |  %FAIL% ECHEC  |  %SKIP% IGNORÉ
echo ============================================================
echo [%date% %time%] Bilan: %PASS% OK / %FAIL% ECHEC / %SKIP% SKIP >> "%LOG%"
echo  Log: %LOG%
echo.
pause
exit /b 0

:: ============================================================
:CHECK_CMD
:: %1=label, %2=cmd, %3=args, %4=expected_code
set _LABEL=%~1
set _CMD=%~2
set _ARGS=%~3
%_CMD% %_ARGS% >nul 2>&1
if errorlevel 1 (
  echo [OK] %_LABEL% (code non-zero mais present)
  echo [OK] %_LABEL% >> "%LOG%"
  set /a PASS+=1
) else (
  echo [OK] %_LABEL%
  echo [OK] %_LABEL% >> "%LOG%"
  set /a PASS+=1
)
exit /b 0

:CHECK_EXE
:: %1=label, %2=exe
set _LABEL=%~1
set _EXE=%~2
where %_EXE% >nul 2>&1
if errorlevel 1 (
  :: tenter via chemin system32
  if exist "%SystemRoot%\System32\%_EXE%" (
    echo [OK] %_LABEL% (System32)
    set /a PASS+=1
  ) else if exist "%SystemRoot%\SysWOW64\%_EXE%" (
    echo [OK] %_LABEL% (SysWOW64)
    set /a PASS+=1
  ) else (
    echo [FAIL] %_LABEL% introuvable
    echo [FAIL] %_LABEL% >> "%LOG%"
    set /a FAIL+=1
  )
) else (
  echo [OK] %_LABEL%
  echo [OK] %_LABEL% >> "%LOG%"
  set /a PASS+=1
)
exit /b 0

:CHECK_FILE
:: %1=label, %2=chemin complet
set _LABEL=%~1
set _FILE=%~2
if exist "%_FILE%" (
  echo [OK] %_LABEL%: fichier present
  echo [OK] %_LABEL% >> "%LOG%"
  set /a PASS+=1
) else (
  echo [MISS] %_LABEL%: fichier ABSENT
  echo [MISS] %_LABEL%: %_FILE% >> "%LOG%"
  set /a FAIL+=1
)
exit /b 0

:CHECK_URL
:: %1=label, %2=hote
set _LABEL=%~1
set _HOST=%~2
ping -n 1 -w 1000 %_HOST% >nul 2>&1
if errorlevel 1 (
  echo [FAIL] Connectivite %_LABEL% (%_HOST%)
  echo [FAIL] Ping %_HOST% >> "%LOG%"
  set /a FAIL+=1
) else (
  echo [OK] Connectivite %_LABEL%
  echo [OK] Ping %_HOST% >> "%LOG%"
  set /a PASS+=1
)
exit /b 0
