@echo off
title NiTriTe Build v26.1.0
echo ========================================
echo   NiTriTe - Build Production v26.1.0
echo ========================================
echo.

cd /d "%~dp0"

echo [1/3] Verification des dependances npm...
call npm install --silent
if %ERRORLEVEL% neq 0 (
    echo [ERREUR] npm install a echoue.
    pause
    exit /b 1
)

echo [2/3] Build Tauri (frontend + backend)...
call npx tauri build
if %ERRORLEVEL% neq 0 (
    echo [ERREUR] Build Tauri a echoue.
    pause
    exit /b 1
)

echo.
echo ========================================
echo   Build termine avec succes !
echo   Executable: src-tauri\target\release\nitrite.exe
echo ========================================
echo.
pause
