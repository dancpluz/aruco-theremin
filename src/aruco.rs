use opencv::{
    core::{Point2f, Vector},
    objdetect::{ArucoDetector, DetectorParameters, RefineParameters, get_predefined_dictionary},
    prelude::ArucoDetectorTraitConst,
};
use std::error::Error;

use crate::{
    config::{DEFAULT_MIN_MARKER_SIZE, DICTIONARY_TYPE},
    ui::{DetectedMarker, NormalizedPosition},
};

pub struct ArucoProcessor {
    detector: ArucoDetector,
    min_marker_size: f32,
    last_position: NormalizedPosition,
}

impl ArucoProcessor {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let dictionary = get_predefined_dictionary(DICTIONARY_TYPE)?;
        let parameters = DetectorParameters::default()?;
        let refine_params = RefineParameters {
            min_rep_distance: 10.0,
            error_correction_rate: 3.0,
            check_all_orders: true,
        };

        let detector = ArucoDetector::new(&dictionary, &parameters, refine_params)?;

        Ok(ArucoProcessor {
            detector,
            min_marker_size: DEFAULT_MIN_MARKER_SIZE,
            last_position: NormalizedPosition::new(0.0, 0.0, false),
        })
    }

    pub fn detect_markers(
        &self,
        frame: &opencv::core::Mat,
    ) -> Result<Vec<DetectedMarker>, Box<dyn Error>> {
        let mut corners = Vector::<Vector<Point2f>>::new();
        let mut ids = Vector::<i32>::new();
        let mut rejected = Vector::<Vector<Point2f>>::new();

        self.detector
            .detect_markers(frame, &mut corners, &mut ids, &mut rejected)?;

        let mut markers = Vec::new();
        for (i, id) in ids.iter().enumerate() {
            if let Ok(corner_vec) = corners.get(i) {
                let corners_vec: Vec<Point2f> = corner_vec.iter().collect();
                let marker = DetectedMarker::new(id, corners_vec);

                if self.is_marker_valid(&marker) {
                    markers.push(marker);
                }
            }
        }

        Ok(markers)
    }

    fn is_marker_valid(&self, marker: &DetectedMarker) -> bool {
        // calcular perímetro
        let corners = &marker.corners;
        if corners.len() != 4 {
            return false;
        }

        let mut perimeter = 0.0;
        for i in 0..4 {
            let j = (i + 1) % 4;
            let dx = corners[i].x - corners[j].x;
            let dy = corners[i].y - corners[j].y;
            perimeter += (dx * dx + dy * dy).sqrt();
        }

        if perimeter < self.min_marker_size {
            return false;
        }

        // verificar se é aproximadamente quadrado
        let mut side_lengths = Vec::new();
        for i in 0..4 {
            let j = (i + 1) % 4;
            let dx = corners[i].x - corners[j].x;
            let dy = corners[i].y - corners[j].y;
            side_lengths.push((dx * dx + dy * dy).sqrt());
        }

        let avg_length: f32 = side_lengths.iter().sum::<f32>() / 4.0;
        let max_variation = side_lengths
            .iter()
            .map(|&l| (l - avg_length).abs() / avg_length)
            .fold(0f32, |a, b| a.max(b));

        max_variation < 0.3
    }

    pub fn calculate_marker0_position(
        &mut self,
        frame_width: i32,
        frame_height: i32,
        markers: &[DetectedMarker],
    ) -> NormalizedPosition {
        for marker in markers {
            if marker.id == 0 {
                let center = marker.center;

                // normalizar posição para [-1, 1]
                let x_normalized = ((center.x * 2.0) / frame_width as f32) - 1.0;
                let y_normalized = ((center.y * 2.0) / frame_height as f32) - 1.0;

                let position = NormalizedPosition::new(x_normalized, y_normalized, true);
                self.last_position = position;
                return position;
            }
        }

        let position = NormalizedPosition::new(0.0, 0.0, false);
        self.last_position = position;
        position
    }
}
