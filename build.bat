@echo off
title Nitrite Build v26.17.0
echo ============================================================
echo   Nitrite 1.0 - Build Production v26.17.0
echo ============================================================
echo.
cd /d "%~dp0"

echo [0/5] Arret des instances en cours...
taskkill /F /IM nitrite.exe >nul 2>&1
taskkill /F /IM nitrite-tauri.exe >nul 2>&1
timeout /t 1 /nobreak >nul

echo [1/5] Verification des dependances npm...
if not exist "node_modules" (
    call npm install
    if %ERRORLEVEL% neq 0 (
        echo [ERREUR] npm install a echoue.
        pause
        exit /b 1
    )
) else (
    echo      node_modules deja present, installation ignoree.
)

echo [2/5] Verification TypeScript...
call npx tsc --noEmit
if %ERRORLEVEL% neq 0 (
    echo [ERREUR] Erreurs TypeScript detectees. Build annule.
    pause
    exit /b 1
)

echo [3/5] Build Tauri (frontend + backend)...
call npx tauri build
if %ERRORLEVEL% neq 0 (
    echo [ERREUR] Build Tauri a echoue.
    pause
    exit /b 1
)

echo.
echo ============================================================
echo   Build termine avec succes !
echo.
echo   Executable  : src-tauri\target\release\nitrite.exe
echo   Installeur  : src-tauri\target\release\bundle\nsis\
echo   MSI         : src-tauri\target\release\bundle\msi\
echo ============================================================
echo.
pause
