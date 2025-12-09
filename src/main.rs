mod camera;
mod config;

use opencv::core::Mat;
use opencv::highgui::{WINDOW_NORMAL, imshow, named_window, wait_key};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Captura de Vídeo Simples ===");
    println!("Pressione ESC para sair");
    println!();

    // Inicializar câmera ou vídeo
    let (mut cam, is_camera) = camera::initialize_capture()?;

    // Criar janela
    named_window("Video", WINDOW_NORMAL)?;

    println!("▶️  Iniciando captura...");
    println!();

    // Loop principal
    loop {
        // Capturar frame
        let mut frame = Mat::default();
        if !camera::read_frame(&mut cam, &mut frame, is_camera)? {
            break;
        }

        // Mostrar frame
        imshow("Video", &frame)?;

        // Verificar tecla ESC (código 27)
        let key = wait_key(30)?;
        if key == 27 {
            println!("\n✅ Encerrando...");
            break;
        }
    }

    println!("Programa finalizado!");
    Ok(())
}
