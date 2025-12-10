use opencv::{core::Scalar, objdetect::PredefinedDictionaryType};

/// fallback video paths
pub const VIDEO_PATHS: &[&str] = &[
    "video.mp4",
    "videos/aruco_video.mp4",
    "test_video.mp4",
    "assets/aruco_video.mp4",
    "aruco_video.mp4"
];

pub const DEFAULT_MIN_MARKER_SIZE: f32 = 30.0;

pub const DICTIONARY_TYPE: PredefinedDictionaryType = PredefinedDictionaryType::DICT_ARUCO_ORIGINAL;

pub const COLOR_GREEN: Scalar = Scalar::new(0.0, 255.0, 0.0, 0.0);
pub const COLOR_RED: Scalar = Scalar::new(0.0, 0.0, 255.0, 0.0);
pub const COLOR_WHITE: Scalar = Scalar::new(255.0, 255.0, 255.0, 0.0);
