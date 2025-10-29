use opencv::{
    prelude::*,
    videoio,
    highgui,
    imgproc,
    core::{Size, Point, Scalar, Vector, no_array},
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
    let mut cam = videoio::VideoCapture::from_file("video2.mp4", videoio::CAP_ANY)
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
    let mut blurred_frame = Mat::default();
    let mut edges_frame = Mat::default();
    let mut dilated_frame = Mat::default();
    let mut contours = Vector::<Vector<Point>>::new();

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

        if frame.size()?.width > 0 {
            // Passo 1: Escala de Cinza
            imgproc::cvt_color(
                &frame, 
                &mut gray_frame, 
                imgproc::COLOR_BGR2GRAY, 
                0
            )?;

            // Passo 2: Desfoque Gaussiano
            imgproc::gaussian_blur(
                &gray_frame,
                &mut blurred_frame,
                Size::new(5, 5),
                0.0, 
                0.0, 
                0
            )?;

            imgproc::canny(
                &blurred_frame,
                &mut edges_frame,
                150.0,
                200.0,
                3,
                false
            )?;

            imgproc::dilate(
                &edges_frame,
                &mut dilated_frame, // <-- 2. USADO AQUI
                &no_array(), // Usa um kernel padrão
                Point::new(-1, -1), // Posição da âncora (padrão)
                1, // Número de iterações
                0, // Tipo de borda
                Scalar::default() // Valor da borda
            )?;

            imgproc::find_contours(
                &mut dilated_frame,
                &mut contours,
                imgproc::RETR_EXTERNAL,
                imgproc::CHAIN_APPROX_SIMPLE,
                Point::new(0, 0)
            )?;

            imgproc::draw_contours(
                &mut frame,                                 // Imagem onde vamos desenhar
                &contours,                                  // A lista de contornos
                -1,                                         // Índice (-1 = desenhar todos)
                Scalar::new(0.0, 255.0, 0.0, 0.0), // Cor (Verde)
                2,                                          // Espessura
                imgproc::LINE_8,                            // Tipo de linha
                &no_array(),                                // Hierarquia (não precisamos agora)
                0,                                          // Nível máximo
                Point::new(0, 0)                            // Offset
            )?;

            // Exibe o resultado das bordas
            highgui::imshow("Video", &dilated_frame)?;
        }

        // 6. Esperar por uma tecla (por 30ms)
        let key = highgui::wait_key(30)?;
        if key == 'q' as i32 || key == 27 { // 'q' ou ESC
            break;
        }
    }

    Ok(())
}
