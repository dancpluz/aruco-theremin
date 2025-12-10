mod aruco;
mod camera;
mod config;
mod theremin;
mod ui;

use aruco::ArucoProcessor;
use opencv::{
    core::Mat,
    highgui::{WINDOW_AUTOSIZE, imshow, named_window, wait_key},
    prelude::MatTraitConst,
};
use std::error::Error;
use theremin::ThereminController;
use ui::{draw_markers, draw_position_info, draw_theremin_info};

fn main() -> Result<(), Box<dyn Error>> {
    println!("===== ArUco + Theremin =====");
    println!("Controles:");
    println!("  ESC     - Sair");
    println!("  ESPAÇO  - Ativar/Desativar som");
    println!("============================");

    let mut theremin_controller = ThereminController::new()?;
    println!("[START] Theremin inicializado. Som ativo.");

    let (mut cam, is_camera) = camera::initialize_capture()?;

    let mut aruco_processor = match ArucoProcessor::new() {
        Ok(processor) => {
            println!("[START] Processador ArUco inicializado");
            Some(processor)
        }
        Err(e) => {
            println!("[ERROR] Erro ao inicializar ArUco: {}", e);
            println!("[INFO] Continuando apenas com visualização de vídeo...");
            None
        }
    };

    named_window("Video", WINDOW_AUTOSIZE)?;
    println!("[START] Iniciando captura de vídeo...");
    println!("============================");

    let mut frame_counter = 0;
    let mut last_position = (0.0, 0.0); // (x, y)

    loop {
        frame_counter += 1;

        let mut frame = Mat::default();
        if !camera::read_frame(&mut cam, &mut frame, is_camera)? {
            println!("[INFO] Fim do vídeo/câmera");
            break;
        }

        let frame_width = frame.cols();
        let frame_height = frame.rows();

        if let Some(processor) = &mut aruco_processor {
            match processor.detect_markers(&frame) {
                Ok(markers) => {
                    if let Err(e) = draw_markers(&mut frame, &markers) {
                        eprintln!("[ERROR] Erro ao desenhar marcadores: {}", e);
                    }

                    let marker_position =
                        processor.calculate_marker0_position(frame_width, frame_height, &markers);

                    if let Err(e) = draw_position_info(&mut frame, &marker_position) {
                        eprintln!("[ERROR] Erro ao desenhar informações: {}", e);
                    }

                    // marcador detectado
                    if marker_position.detected {
                        last_position = (marker_position.x, marker_position.y);
                        theremin_controller
                            .update_from_position(marker_position.x, marker_position.y);
                    } else {
                        // usar a última posição
                        theremin_controller.update_from_position(last_position.0, last_position.1);
                    }

                    draw_theremin_info(&mut frame, &theremin_controller)?;

                    if frame_counter % 30 == 0 {
                        if marker_position.detected {
                            println!(
                                "[INFO] Frame {}: (x: {:.3}, y: {:.3}) | Frequência: {:.1} Hz, Amplitude: {:.2}",
                                frame_counter,
                                marker_position.x,
                                marker_position.y,
                                theremin_controller.get_frequency(),
                                theremin_controller.get_amplitude()
                            );
                        } else {
                            println!("[INFO] Frame {}: Marcador não detectado", frame_counter);
                        }
                    }
                }
                Err(e) => {
                    if !e.to_string().contains("empty") && frame_counter % 60 == 0 {
                        eprintln!("[ERROR] Erro na detecção: {}", e);
                    }
                }
            }
        }

        imshow("Video", &frame)?;

        let key = wait_key(30)?;

        match key {
            27 => {
                // ESC
                println!("[INFO] Esc pressionado. Saindo...");
                theremin_controller.stop();
                break;
            }
            32 => {
                // ESPAÇO
                theremin_controller.toggle_sound();
            }
            _ => {}
        }
    }

    println!("============================");
    println!("[INFO] Liberando recursos...");

    if let Err(e) = camera::release_capture(&mut cam) {
        eprintln!("[ERROR] Erro ao liberar câmera: {}", e);
    }

    println!("[INFO] Programa finalizado com sucesso.");
    Ok(())
}
