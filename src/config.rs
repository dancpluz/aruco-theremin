/// Caminhos de vídeo para fallback (quando câmera não estiver disponível)
pub const VIDEO_PATHS: &[&str] = &[
    "hand_video.mp4",
    "videos/hand_video.mp4",
    "test_video.mp4",
    "video.mp4",
    "assets/video.mp4",
];

/// Caminhos de áudio para tentar
pub const AUDIO_PATHS: &[&str] = &[
    "audio.mp3",
    "audio.wav",
    "sound.mp3",
    "assets/audio.mp3",
    "assets/sound.mp3",
];

/// Pitch range constants
pub const MIN_PITCH: f32 = 0.25;  // Two octaves lower
pub const MAX_PITCH: f32 = 4.0;   // Two octaves higher
pub const DEFAULT_PITCH: f32 = 1.0; // Normal pitch

/// Helper para encontrar primeiro arquivo de áudio disponível
pub fn find_audio_file() -> Option<String> {
    for path in AUDIO_PATHS {
        if std::path::Path::new(path).exists() {
            return Some(path.to_string());
        }
    }
    None
}