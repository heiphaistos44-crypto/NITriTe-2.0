# ============================================================
# TEST DÉSINSTALLATION SILENCIEUSE — À lancer en tant qu'Admin
# Ce script simule exactement ce que Nitrite fait pour VLC
# ============================================================

# 1. Trouver l'UninstallString de VLC
$paths = @(
    'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\*',
    'HKLM:\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\*',
    'HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\*'
)

$vlc = Get-ItemProperty $paths -ErrorAction SilentlyContinue |
    Where-Object { $_.DisplayName -like '*VLC*' } |
    Select-Object -First 1

if (-not $vlc) {
    Write-Host "[ERREUR] VLC non trouvé dans le registre." -ForegroundColor Red
    exit
}

Write-Host "[INFO] Trouvé : $($vlc.DisplayName)" -ForegroundColor Cyan
Write-Host "[INFO] UninstallString : $($vlc.UninstallString)" -ForegroundColor Cyan

# 2. Extraire le chemin de l'exe
$s = $vlc.UninstallString.Trim()
if ($s -match '^"([^"]+)"') { $exe = $matches[1] }
else { $exe = ($s -split '\s+')[0].Trim('"') }

Write-Host "[INFO] Exe : $exe" -ForegroundColor Cyan

# 3. Détecter le type d'installeur
$vi   = (Get-Item -LiteralPath $exe -ErrorAction SilentlyContinue).VersionInfo
$meta = "$($vi.FileDescription) $($vi.ProductName) $($vi.CompanyName)"
Write-Host "[INFO] Métadonnées : $meta" -ForegroundColor Yellow

if ($meta -match 'Inno|Jordan Russell') {
    $type = 'Inno'
    $args = @('/VERYSILENT', '/SUPPRESSMSGBOXES', '/NORESTART', '/SP-')
} else {
    $type = 'NSIS ou inconnu'
    $args = @('/S')
}

Write-Host "[INFO] Type détecté : $type" -ForegroundColor Yellow
Write-Host "[INFO] Arguments : $args" -ForegroundColor Yellow
Write-Host ""
Write-Host "[ATTENTION] Si tu continues, VLC sera désinstallé silencieusement !" -ForegroundColor Red
$confirm = Read-Host "Taper OUI pour continuer"
if ($confirm -ne 'OUI') { Write-Host "Annulé."; exit }

# 4. Lancer la désinstallation silencieuse
Write-Host "[INFO] Lancement avec -Wait..." -ForegroundColor Cyan
$proc = Start-Process -FilePath $exe -ArgumentList $args -Wait -PassThru
Write-Host "[INFO] Processus terminé. Code : $($proc.ExitCode)" -ForegroundColor Green

# 5. Vérifier
Start-Sleep -Milliseconds 2500
$still = Get-ItemProperty $paths -ErrorAction SilentlyContinue |
    Where-Object { $_.DisplayName -like '*VLC*' }
if ($still) {
    Write-Host "[FAIL] VLC toujours présent dans le registre." -ForegroundColor Red
} else {
    Write-Host "[OK] VLC supprimé du registre avec succès." -ForegroundColor Green
}
