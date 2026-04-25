@echo off
setlocal EnableDelayedExpansion
cd /d "%~dp0"
echo === TEST BOOT WinPE pur (aucune modification) ===
echo.

set "PROJ_ROOT=%~dp0.."
set "OUT_DIR=%PROJ_ROOT%\release"

:: ── Recherche ADK (x86 puis x64) ─────────────────────────────────────────────
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

set "WINPE_ADK=!ADK_PATH!\Windows Preinstallation Environment"
set "OSCDIMG=!ADK_PATH!\Deployment Tools\x86\Oscdimg\oscdimg.exe"
set "WORK_DIR=%TEMP%\winpe_test_work"
set "ISO_OUT=!OUT_DIR!\TEST_WinPE_pur.iso"

if not exist "!OUT_DIR!" mkdir "!OUT_DIR!"
if exist "!WORK_DIR!" rmdir /s /q "!WORK_DIR!"

call "!ADK_PATH!\Deployment Tools\DandISetEnv.bat" >nul 2>&1
cd /d "!PROJ_ROOT!"

echo --- copype...
call "!WINPE_ADK!\copype.cmd" amd64 "!WORK_DIR!"

echo --- Generation ISO via MakeWinPEMedia.cmd...
set "ISO_TEMP=%TEMP%\winpe_test.iso"
if exist "!ISO_TEMP!" del /f /q "!ISO_TEMP!"

call "!WINPE_ADK!\MakeWinPEMedia.cmd" /ISO "!WORK_DIR!" "!ISO_TEMP!"

rmdir /s /q "!WORK_DIR!"
if exist "!ISO_OUT!" del /f /q "!ISO_OUT!"
move /y "!ISO_TEMP!" "!ISO_OUT!" >nul 2>&1

echo.
for %%F in ("!ISO_OUT!") do echo ISO : %%~fF ^(%%~zF octets^)
echo.
echo Monte cet ISO dans la VM et teste le boot.
echo Si ca boote = le WIM modifie est le probleme.
echo Si ca ne boote pas = probleme de configuration VM.
echo.
pause
