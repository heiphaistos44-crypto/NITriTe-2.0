import type { KBCategory } from "../knowledgeBase";

export const kbMultimedia: KBCategory[] = [
  {
    id: "audio-video-encodage",
    label: "Audio & Vidéo — Encodage",
    icon: "Film",
    items: [
      {
        title: "FFmpeg — commandes essentielles",
        solution: [
          "FFmpeg : outil en ligne de commande pour convertir, couper, fusionner audio/vidéo",
          "Télécharger depuis ffmpeg.org (builds statiques pour Windows)",
          "Ajouter au PATH pour l'utiliser depuis n'importe quel terminal",
          "Formats d'entrée supportés : quasi tous (MKV, MP4, AVI, MOV, WMV, FLV, WebM...)",
          "Encodeurs vidéo : H.264 (libx264), H.265/HEVC (libx265), AV1 (libaom-av1, libsvtav1)",
          "Encodeurs audio : AAC, MP3 (libmp3lame), Opus, FLAC",
          "CRF (Constant Rate Factor) : qualité fixe (18-28 pour H.264, plus bas = meilleure qualité)",
        ],
        code: `# Convertir MP4 en MKV (copie codec)
ffmpeg -i input.mp4 -c copy output.mkv

# Convertir en H.265/HEVC (50% plus petit qu'H.264)
ffmpeg -i input.mp4 -c:v libx265 -crf 22 -c:a aac -b:a 128k output.mp4

# Compresser la vidéo (H.264 haute qualité)
ffmpeg -i input.mp4 -c:v libx264 -crf 20 -preset slow -c:a aac -b:a 128k output.mp4

# Extraire l'audio
ffmpeg -i video.mp4 -vn -c:a libmp3lame -q:a 2 audio.mp3
ffmpeg -i video.mp4 -vn -c:a flac audio.flac

# Couper une vidéo (sans réencodage)
ffmpeg -i input.mp4 -ss 00:01:30 -to 00:05:00 -c copy output.mp4

# Redimensionner
ffmpeg -i input.mp4 -vf "scale=1920:1080" -c:v libx264 -crf 20 output.mp4
ffmpeg -i input.mp4 -vf "scale=1280:-2" output.mp4  # Hauteur automatique

# Fusionner des fichiers
ffmpeg -f concat -safe 0 -i liste.txt -c copy output.mp4
# liste.txt contient :
# file 'part1.mp4'
# file 'part2.mp4'

# Extraire des frames
ffmpeg -i video.mp4 -vf fps=1 frames/frame%04d.png  # 1 image/seconde

# Convertir une série d'images en vidéo
ffmpeg -r 30 -i frames/frame%04d.png -c:v libx264 -pix_fmt yuv420p output.mp4

# Ajouter des sous-titres
ffmpeg -i video.mp4 -i subtitles.srt -c copy -c:s mov_text output.mp4

# Comprimer pour WhatsApp/réseaux sociaux
ffmpeg -i input.mp4 -vf "scale=1280:-2" -c:v libx264 -crf 25 -maxrate 2M -bufsize 4M -c:a aac -b:a 96k output.mp4`,
        note: "CRF 18 ≈ visuellement sans perte pour H.264. CRF 22-24 bon compromis taille/qualité. preset slow = meilleure compression au même CRF.",
      },
      {
        title: "Handbrake — encodage vidéo simplifié",
        solution: [
          "Handbrake : interface graphique pour FFmpeg, idéal pour les conversions en lot",
          "Télécharger depuis handbrake.fr (gratuit, open source)",
          "Presets : Web > YouTube HQ (1080p), Devices > Android/iPhone, Production Ready",
          "Format de sortie recommandé : MKV (plus universel) ou MP4 (meilleure compatibilité mobile)",
          "Codec vidéo H.265 NVENC : encode avec le GPU Nvidia (beaucoup plus rapide, qualité légèrement inférieure)",
          "Qualité RF : 20-24 pour H.264, 24-28 pour H.265 (même résultat visuel)",
          "Audio : AAC 160kbps pour la plupart des usages, AC3 passthrough pour le home cinema",
          "Convertir une collection entière : File > Open Source > Ajouter un dossier",
        ],
        command: "HandBrakeCLI -i input.mp4 -o output.mp4 --preset=\"H.265 MKV 1080p30\"",
        note: "HandBrakeCLI permet le traitement par lot en script. --preset-import-file custom.json pour utiliser vos propres presets.",
      },
      {
        title: "OBS Studio — enregistrement et streaming",
        solution: [
          "OBS Studio : outil gratuit de streaming et enregistrement (obsproject.com)",
          "Scènes : collections de sources (fenêtres, webcam, images, texte)",
          "Sources : Capture d'écran/jeu, Périphérique de capture vidéo (webcam), Source audio, Capture de fenêtre",
          "Encodeur vidéo : NVENC H.265 (Nvidia GPU) ou x264 (CPU plus compatible)",
          "Pour l'enregistrement : qualité CQP 20 (GPU) ou CRF 20 (CPU), container MKV",
          "Pour le streaming Twitch : 6000 kbps max, encodeur NVENC H.264, B-frames 2",
          "Audio : 160kbps AAC, sample rate 48kHz",
          "Résolution : 1920x1080 @ 60fps pour le gaming, 30fps suffit pour les tutoriels",
        ],
        code: `# Configuration OBS recommandée streaming
Paramètres > Sortie > Mode Avancé
  Encodeur: NVIDIA NVENC H.264
  Débit: 6000 Kbps
  Intervalle keyframe: 2s
  Preset: P5 (Lent)
  Profil: high
  B-frames: 2

# Pour l'enregistrement haute qualité
  Encodeur: NVIDIA NVENC HEVC (H.265)
  Mode qualité: CQP
  Valeur CQ: 20
  Container: MKV (plus sûr que MP4 si crash)

# Raccourcis OBS utiles
Start/Stop Recording: Ctrl+Alt+R (configurable)
Start/Stop Streaming: Ctrl+Alt+S
Mute Mic: Ctrl+Alt+M
Switch Scene: Ctrl+1-9

# Convertir MKV OBS → MP4 après enregistrement
ffmpeg -i Recording.mkv -c copy Recording.mp4`,
        note: "Toujours enregistrer en MKV — si OBS crashe, le fichier MKV reste lisible contrairement au MP4 non finalisé.",
      },
      {
        title: "Formats audio — guide complet",
        solution: [
          "FLAC : lossless (sans perte), 50% plus petit que WAV — idéal pour la musique master",
          "MP3 : lossy (avec perte) 128-320kbps — universel, compatible partout",
          "AAC : meilleur que MP3 à même bitrate — utilisé par iTunes/Apple, YouTube, streaming",
          "Opus : codec moderne, meilleur qu'AAC à faible bitrate (32-64kbps) — Discord, YouTube interne",
          "WAV/AIFF : non compressé, énorme fichier — studio uniquement",
          "OGG Vorbis : open source, qualité similaire à AAC — Steam, Spotify interne",
          "Pour la musique : distribuer en FLAC (masters), MP3 320kbps (partage), AAC 256kbps (streaming)",
          "Convertir avec FFmpeg, Audacity, ou fre:ac (gratuit, GUI)",
        ],
        code: `# Convertir avec FFmpeg
# WAV/FLAC vers MP3 320kbps
ffmpeg -i input.flac -c:a libmp3lame -q:a 0 output.mp3
# -q:a 0 = qualité maximale variable (320kbps)

# Batch : convertir tous les FLAC d'un dossier en MP3
Get-ChildItem *.flac | ForEach-Object {
  $output = $_.BaseName + ".mp3"
  ffmpeg -i $_.FullName -c:a libmp3lame -q:a 0 $output
}

# FLAC vers AAC 256kbps (pour iTunes/Apple)
ffmpeg -i input.flac -c:a aac -b:a 256k output.m4a

# Normaliser le volume audio
ffmpeg -i input.mp3 -af loudnorm=I=-16:LRA=11:TP=-1.5 output.mp3

# Extraire piste audio d'une vidéo en FLAC
ffmpeg -i video.mkv -map 0:a:0 -c:a flac audio.flac`,
      },
      {
        title: "VLC — fonctionnalités avancées",
        solution: [
          "VLC est bien plus qu'un lecteur vidéo : transcodage, streaming réseau, enregistrement",
          "Lire des flux réseau : Média > Ouvrir un flux réseau (RTSP, HTTP, YouTube URL)",
          "Enregistrer un stream : Média > Convertir/Enregistrer > Réseau > URL",
          "Changer la vitesse : [ et ] (ralentir/accélérer), touches 1-4 (vitesse fixe)",
          "Égaliseur audio : Outils > Effets > Égaliseur",
          "Synchroniser audio/vidéo : G/H ajustent le délai audio par 50ms",
          "Captures d'écran vidéo : Maj+S (capture PNG dans le dossier vidéo)",
          "Mode plein écran sans titre : View > Minimal Interface (Ctrl+H)",
          "Convertir des fichiers : Média > Convertir/Enregistrer",
        ],
        command: "vlc input.mp4 --sout \"#transcode{vcodec=h264,acodec=mp3}:file{dst=output.mp4}\" vlc://quit",
      },
    ],
  },
  {
    id: "photo-graphisme",
    label: "Photo & Graphisme",
    icon: "Image",
    items: [
      {
        title: "ImageMagick — traitement d'images en lot",
        solution: [
          "ImageMagick : outil CLI puissant pour traiter des images (redimensionner, convertir, watermark...)",
          "Télécharger depuis imagemagick.org (Windows : installer avec la case PATH cochée)",
          "convert : ancienne commande (v6), magick : nouvelle commande (v7)",
          "Traitement par lot via des scripts PowerShell",
          "Prend en charge 200+ formats : JPG, PNG, WEBP, AVIF, HEIC, SVG, PDF, PSD...",
        ],
        code: `# Resize images
magick input.jpg -resize 1920x1080 output.jpg
magick input.jpg -resize 1920x1080^ -gravity center -extent 1920x1080 output.jpg  # Crop

# Convertir en WebP (format web moderne)
magick input.jpg -quality 80 output.webp

# Batch resize — tous les JPG d'un dossier
Get-ChildItem *.jpg | ForEach-Object {
  magick $_.Name -resize 1280x720 "resized/$($_.Name)"
}

# Créer un watermark
magick input.jpg -gravity SouthEast -fill "rgba(255,255,255,0.5)" ^
  -pointsize 24 -annotate +10+10 "© MonNom 2025" output.jpg

# Optimiser JPEG sans perte visible
magick input.jpg -strip -quality 85 -sampling-factor 4:2:0 output.jpg

# Créer une miniature carrée (thumbnail)
magick input.jpg -thumbnail 300x300^ -gravity center -extent 300x300 thumb.jpg

# Convertir PDF en images
magick -density 150 document.pdf page_%04d.png

# Assembler des images en PDF
magick *.png document.pdf

# Retirer l'arrière-plan (basique)
magick input.png -fuzz 10% -transparent white output.png`,
        note: "Pour la suppression d'arrière-plan complexe : utiliser Remove.bg (en ligne) ou REMBG (outil Python/Rust local).",
      },
      {
        title: "GIMP — raccourcis et fonctionnalités pro",
        solution: [
          "GIMP : alternative gratuite à Photoshop (gimp.org)",
          "Raccourcis essentiels : S (Scale), R (Rectangle), F (Bucket fill), G (Gradient), T (Text)",
          "Script-Fu (Scheme) ou Python-Fu : automatiser les tâches répétitives",
          "Exporter (pas Enregistrer) pour JPEG/PNG — Enregistrer conserve le format XCF natif",
          "Activer Single-Window Mode : Fenêtres > Mode fenêtre unique",
          "Niveaux et Courbes : Couleurs > Niveaux (Ctrl+L) / Courbes (Ctrl+M)",
          "Content-Aware Fill (depuis GIMP 2.10) : Filtres > Améliorer > Smart Remove Selection",
          "Plugins utiles : G'MIC (effets avancés), BIMP (traitement par lot)",
        ],
        code: `# Script-Fu batch — redimensionner toutes les images
; Dans GIMP : Filtres > Script-Fu > Console
(let* ((filelist (cadr (file-glob "/chemin/*.jpg" 1))))
  (for-each (lambda (filename)
    (let* ((image (car (gimp-file-load RUN-NONINTERACTIVE filename filename)))
           (drawable (car (gimp-image-get-active-drawable image))))
      (gimp-image-scale-full image 1920 1080 INTERPOLATION-LINEAR)
      (file-jpeg-save RUN-NONINTERACTIVE image drawable
                     (string-append "/output/" (basename filename))
                     (basename filename) 0.85 0 0 0 "" 0 1 0 2 0)
      (gimp-image-delete image)))
  filelist))`,
      },
      {
        title: "Optimisation web — images et formats modernes",
        solution: [
          "WebP : 25-35% plus petit que JPEG/PNG à qualité équivalente — supporté par tous les navigateurs modernes",
          "AVIF : 50% plus petit que JPEG, mais encodage lent — idéal pour les images statiques",
          "SVG : format vectoriel, infiniment scalable, idéal pour les logos et icônes",
          "Squoosh (squoosh.app) : comparaison visuelle de formats dans le navigateur",
          "Sharp (Node.js) : bibliothèque pour optimiser les images en masse",
          "Règle de base : WEBP pour les photos, SVG pour les illustrations, PNG pour les screenshots",
          "srcset HTML : servir différentes tailles selon la résolution de l'écran",
        ],
        code: `# Convertir en WebP avec FFmpeg
ffmpeg -i input.jpg -c:v libwebp -quality 80 output.webp

# Sharp (Node.js) — optimisation en masse
npm install sharp
node -e "
const sharp = require('sharp');
const fs = require('fs');
const files = fs.readdirSync('./images').filter(f => f.match(/\.(jpg|png)$/));
files.forEach(file => {
  sharp('./images/' + file)
    .resize(1920, null, { withoutEnlargement: true })
    .webp({ quality: 80 })
    .toFile('./webp/' + file.replace(/\.(jpg|png)$/, '.webp'));
});
"

# HTML — image responsive avec WebP + fallback
<picture>
  <source srcset="image.avif" type="image/avif">
  <source srcset="image.webp" type="image/webp">
  <img src="image.jpg" alt="Description" loading="lazy" width="800" height="600">
</picture>`,
      },
    ],
  },
  {
    id: "streaming-broadcast",
    label: "Streaming & Broadcast",
    icon: "Radio",
    items: [
      {
        title: "Configuration Twitch / YouTube Gaming",
        solution: [
          "Twitch : débit recommandé 4500-6000 kbps (H.264 CBR), 1080p 60fps",
          "YouTube Gaming : débit 4500-9000 kbps selon la résolution",
          "Encodeur NVENC H.264 (Nvidia) : encode avec le GPU, moins de charge CPU",
          "Encodeur x264 (CPU) : meilleure qualité à même bitrate, mais charge CPU élevée",
          "Latence faible : Streaming > Paramètres avancés > Encodeur preset Ultra Low Latency",
          "Tester sa connexion : Twitch Inspector (inspector.twitch.tv) avant de streamer",
          "Audio : 160kbps AAC, micro en entrée séparée dans OBS pour contrôle volume",
          "Scenes suggérées : Main (jeu + webcam), BRB (écran pause), Starting Soon, Ending",
        ],
        code: `# OBS — paramètres streaming Twitch optimaux
Paramètres > Stream:
  Service: Twitch
  Serveur: Automatique (détecte le meilleur)
  Clé de stream: depuis dashboard.twitch.tv

Paramètres > Sortie > Mode Simple:
  Débit vidéo: 5000 kbps
  Encodeur: NVIDIA NVENC H.264 (sinon x264)
  Qualité de l'encodeur: High

Paramètres > Audio:
  Taux d'échantillonnage: 48 kHz
  Canaux: Stéréo

Paramètres > Vidéo:
  Résolution de base: 1920x1080
  Résolution de sortie: 1920x1080 (ou 1280x720)
  Valeur FPS: 60

# Commande ffmpeg pour streamer un fichier
ffmpeg -re -i video.mp4 \\
  -c:v libx264 -preset veryfast -b:v 5000k -maxrate 5000k \\
  -c:a aac -b:a 160k \\
  -f flv rtmp://live.twitch.tv/app/VOTRE_CLE_STREAM`,
      },
    ],
  },
];
