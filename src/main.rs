use opencv::{
    prelude::*,
    videoio,
    highgui,
    imgproc
};
/*
fn cam() -> opencv::Result<videoio::VideoCapture> {
    // 1. Inicializar a captura da webcam
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)
        .expect("Não foi possível abrir a câmera");

    let opened = videoio::VideoCapture::is_opened(&cam)
        .expect("Falha ao verificar se a câmera estava aberta");

    if !opened {
        panic!("Câmera não pode ser aberta");
    }

    // 2. Criar uma janela para exibir
    highgui::named_window("Webcam", highgui::WINDOW_NORMAL)?;

    // 3. Criar um Mat (Matriz) para armazenar cada frame
    let mut frame = Mat::default();

    // 4. Iniciar o loop de captura
    loop {
        // Tenta ler um frame da câmera
        let read_success = cam.read(&mut frame)?;
        
        if !read_success {
            println!("Não foi possível ler o frame da câmera");
            break; // Sai do loop se a câmera for desconectada
        }

        // 5. Exibir o frame na janela
        // Verifica se o frame não está vazio
        if frame.size()?.width > 0 {
            highgui::imshow("Webcam", &frame)?;
        }

        // 6. Esperar por uma tecla (por 1ms)
        // Se 'q' (ou ESC) for pressionado, quebra o loop
        let key = highgui::wait_key(1)?;
        if key == 'q' as i32 || key == 27 { // 27 é o código ASCII para ESC
            break;
        }
    }

    Ok(())
}
*/

fn main() -> opencv::Result<()> {
    // 1. Inicializar a captura do ARQUIVO DE VÍDEO
    let mut cam = videoio::VideoCapture::from_file("video.mp4", videoio::CAP_ANY)
        .expect("Não foi possível abrir o arquivo de vídeo"); 

    let opened = videoio::VideoCapture::is_opened(&cam)
        .expect("Falha ao verificar se o vídeo estava aberto");

    if !opened {
        panic!("Vídeo não pode ser aberto");
    }

    // 2. Criar uma janela para exibir
    highgui::named_window("Video", highgui::WINDOW_NORMAL)?;

    // 3. Criar um Mat (Matriz) para armazenar cada frame
    let mut frame = Mat::default();
    let mut gray_frame = Mat::default();

    // 4. Iniciar o loop de captura (com replay automático ao fim)
    loop {
        // Tenta ler um frame do vídeo
        let read_success = cam.read(&mut frame)?;

        // Se não conseguiu ler (fim do vídeo), volta ao início do vídeo
        if !read_success {
            println!("Fim do vídeo. Reiniciando...");
            // Rebobina o vídeo para o frame 0
            cam.set(videoio::CAP_PROP_POS_FRAMES, 0.0)?;
            continue;
        }

        // 5. Converter para gris e exibir o frame na janela
        imgproc::cvt_color(&frame, &mut gray_frame, imgproc::COLOR_BGR2GRAY, 0)?;
        highgui::imshow("Video", &gray_frame)?;

        // 6. Esperar por uma tecla (por 30ms)
        let key = highgui::wait_key(30)?;
        if key == 'q' as i32 || key == 27 { // 'q' ou ESC
            break;
        }
    }

    Ok(())
}
