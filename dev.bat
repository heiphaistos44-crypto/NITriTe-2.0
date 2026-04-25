@echo off
setlocal EnableDelayedExpansion
cd /d "%~dp0"

:: ── Version dynamique depuis package.json ────────────────────────────────────
for /f "delims=" %%V in ('powershell -NoProfile -Command "(Get-Content package.json -Raw | ConvertFrom-Json).version"') do set "VERSION=%%V"
if not defined VERSION set "VERSION=unknown"

title Nitrite 2.0 - Dev Mode v!VERSION!

:: ── Auto-elevation UAC ───────────────────────────────────────────────────────
net session >nul 2>&1
if !ERRORLEVEL! neq 0 (
    echo --- Elevation UAC necessaire. Relancement en administrateur...
    powershell -NoProfile -Command "Start-Process -FilePath '%~f0' -WorkingDirectory '%~dp0' -Verb RunAs"
    exit /b
)

echo ============================================================
echo   Nitrite 2.0 - Mode Developpement v!VERSION!  [Administrateur]
echo ============================================================
echo.

:: ── Log ──────────────────────────────────────────────────────────────────────
if not exist ".logs" mkdir ".logs"
echo [%DATE% %TIME%] [INFO] tauri dev lance v!VERSION! >> ".logs\dev.log"

:: Kill instances en cours pour liberer les fichiers
taskkill /F /IM nitrite.exe >nul 2>&1

:: Verifier node_modules
if not exist "node_modules" (
    echo --- Installation des dependances npm...
    call npm install
    if !ERRORLEVEL! neq 0 (
        echo [ERREUR] npm install echoue.
        pause & exit /b 1
    )
) else (
    echo --- node_modules OK
)

echo --- Lancement : npm run tauri dev
echo.
call npm run tauri dev

if !ERRORLEVEL! neq 0 (
    echo.
    echo [ERREUR] tauri dev a quitte avec une erreur.
    echo [%DATE% %TIME%] [ERROR] tauri dev exit code !ERRORLEVEL! >> ".logs\dev.log"
    pause
)
