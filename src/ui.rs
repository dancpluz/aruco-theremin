use crate::{
    config::{COLOR_GREEN, COLOR_RED, COLOR_WHITE},
    theremin::ThereminController,
};
use opencv::{
    core::{Point, Point2f},
    imgproc::{FONT_HERSHEY_SIMPLEX, LINE_AA, get_text_size, line, put_text},
    prelude::MatTraitConst,
};

pub struct DetectedMarker {
    pub id: i32,
    pub corners: Vec<Point2f>,
    pub center: Point2f,
}

impl DetectedMarker {
    pub fn new(id: i32, corners: Vec<Point2f>) -> Self {
        let center = if corners.len() >= 4 {
            let mut sum_x = 0.0;
            let mut sum_y = 0.0;
            for corner in &corners {
                sum_x += corner.x;
                sum_y += corner.y;
            }
            Point2f::new(sum_x / 4.0, sum_y / 4.0)
        } else {
            Point2f::new(0.0, 0.0)
        };

        DetectedMarker {
            id,
            corners,
            center,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NormalizedPosition {
    pub x: f32,
    pub y: f32,
    pub detected: bool,
}

impl NormalizedPosition {
    pub fn new(x: f32, y: f32, detected: bool) -> Self {
        NormalizedPosition { x, y, detected }
    }
}

pub fn draw_theremin_info(
    frame: &mut opencv::core::Mat,
    controller: &ThereminController,
) -> Result<(), Box<dyn std::error::Error>> {
    let frame_height = frame.rows();
    let frame_width = frame.cols();

    let sound_status = if controller.is_enabled() { "ON" } else { "OFF" };
    let sound_color = if controller.is_enabled() {
        COLOR_GREEN
    } else {
        COLOR_RED
    };

    let sound_text = format!("Som: {} (Espaco: Liga/Desliga)", sound_status);
    let text_size = get_text_size(&sound_text, FONT_HERSHEY_SIMPLEX, 0.7, 2, &mut 0)?;

    let sound_text_x = frame_width - text_size.width - 10;
    put_text(
        frame,
        &sound_text,
        Point::new(sound_text_x, 30),
        FONT_HERSHEY_SIMPLEX,
        0.7,
        sound_color,
        2,
        LINE_AA,
        false,
    )?;

    let freq_text = format!("Freq: {:.1} Hz", controller.get_frequency());
    let amp_text = format!("Amp: {:.2}", controller.get_amplitude());

    let bottom_y = frame_height - 20;

    put_text(
        frame,
        &freq_text,
        Point::new(10, bottom_y),
        FONT_HERSHEY_SIMPLEX,
        0.7,
        COLOR_WHITE,
        2,
        LINE_AA,
        false,
    )?;

    let amp_text_size = get_text_size(&amp_text, FONT_HERSHEY_SIMPLEX, 0.7, 2, &mut 0)?;

    put_text(
        frame,
        &amp_text,
        Point::new(frame_width - amp_text_size.width - 10, bottom_y),
        FONT_HERSHEY_SIMPLEX,
        0.7,
        COLOR_WHITE,
        2,
        LINE_AA,
        false,
    )?;

    Ok(())
}

pub fn draw_markers(
    frame: &mut opencv::core::Mat,
    markers: &[DetectedMarker],
) -> Result<(), Box<dyn std::error::Error>> {
    for marker in markers {
        let corners = &marker.corners;
        if corners.len() != 4 {
            continue;
        }

        for i in 0..4 {
            let start_point = Point::new(corners[i].x as i32, corners[i].y as i32);
            let end_point =
                Point::new(corners[(i + 1) % 4].x as i32, corners[(i + 1) % 4].y as i32);

            line(frame, start_point, end_point, COLOR_GREEN, 2, LINE_AA, 0)?;
        }

        if marker.id == 0 {
            draw_marker_center_cross(frame, marker.center)?;
        }
    }

    Ok(())
}

pub fn draw_marker_center_cross(
    frame: &mut opencv::core::Mat,
    center: Point2f,
) -> Result<(), Box<dyn std::error::Error>> {
    let center_x = center.x as i32;
    let center_y = center.y as i32;
    let cross_size = 8;

    line(
        frame,
        Point::new(center_x - cross_size, center_y),
        Point::new(center_x + cross_size, center_y),
        COLOR_RED,
        2,
        LINE_AA,
        0,
    )?;

    line(
        frame,
        Point::new(center_x, center_y - cross_size),
        Point::new(center_x, center_y + cross_size),
        COLOR_RED,
        2,
        LINE_AA,
        0,
    )?;

    Ok(())
}

pub fn draw_position_info(
    frame: &mut opencv::core::Mat,
    position: &NormalizedPosition,
) -> Result<(), Box<dyn std::error::Error>> {
    let color = if position.detected {
        COLOR_GREEN
    } else {
        COLOR_RED
    };

    let main_text = if position.detected {
        format!("({:.3}, {:.3})", position.x, position.y)
    } else {
        String::from("Nao detectado")
    };

    put_text(
        frame,
        &main_text,
        Point::new(10, 30),
        FONT_HERSHEY_SIMPLEX,
        0.7,
        color,
        2,
        LINE_AA,
        false,
    )?;

    Ok(())
}
