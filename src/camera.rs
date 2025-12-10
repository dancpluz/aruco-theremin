use opencv::{
    core::Mat,
    prelude::{MatTraitConst, VideoCaptureTrait, VideoCaptureTraitConst},
    videoio::{CAP_ANY, CAP_PROP_POS_FRAMES, VideoCapture},
};
use std::error::Error;

use crate::config::VIDEO_PATHS;

pub fn initialize_capture() -> Result<(VideoCapture, bool), Box<dyn Error>> {
    println!("[INFO] Tentando abrir a câmera...");
    let mut cam = VideoCapture::new(0, CAP_ANY)?;
    
    if cam.is_opened()? {
        println!("[START] Câmera iniciada com sucesso");
        return Ok((cam, true));
    }
    
    // Fallback para vídeo
    println!("[INFO] Câmera não encontrada. Carregando vídeo de exemplo...");
    
    for video_path in VIDEO_PATHS {
        println!("[INFO] Tentando abrir: {}", video_path);
        cam = VideoCapture::from_file(video_path, CAP_ANY)?;
        if cam.is_opened()? {
            println!("[START] Vídeo carregado: {}", video_path);
            return Ok((cam, false));
        }
    }
    
    println!("[ERROR] Não foi possível abrir câmera nem vídeo!");
    println!("[TIP] Coloque um arquivo de vídeo (ex: video.mp4) na pasta do projeto.");
    println!("[TIP] Ou conecte uma webcam e verifique as permissões.");
    
    Err("Nenhuma fonte de vídeo disponível".into())
}

pub fn read_frame(
    cam: &mut VideoCapture,
    frame: &mut Mat,
    is_camera: bool,
) -> Result<bool, Box<dyn Error>> {
    cam.read(frame)?;
    
    if frame.empty() {
        if !is_camera {
            // Reiniciar vídeo
            println!("[INFO] Fim do vídeo. Reiniciando...");
            cam.set(CAP_PROP_POS_FRAMES, 0.0)?;
            cam.read(frame)?;
            
            if frame.empty() {
                println!("[ERROR] Não foi possível reiniciar o vídeo.");
                return Ok(false);
            }
        }
    }
    
    Ok(true)
}

pub fn release_capture(cam: &mut VideoCapture) -> Result<(), Box<dyn Error>> {
    cam.release()?;
    println!("[INFO] Recurso da câmera liberado");
    Ok(())
}