mod audio;
mod camera;
mod commands;
mod config;

use audio::AudioPlayer;
use commands::{KeyCommand, key_to_command, get_pitch_factor, get_pitch_adjustment_message};
use opencv::core::Mat;
use opencv::highgui::{WINDOW_AUTOSIZE, imshow, named_window, wait_key};
use std::error::Error;

/// Ajusta o pitch e reinicia o áudio se estiver tocando
fn adjust_pitch_with_restart(
    audio_player: &AudioPlayer,
    new_pitch: f32,
    is_audio_playing: bool,
    audio_file: &str,
) -> Result<bool, Box<dyn Error>> {
    audio_player.set_pitch(new_pitch);
    
    if is_audio_playing {
        audio_player.stop();
        audio_player.play_file(audio_file)?;
    }
    
    Ok(is_audio_playing)
}

/// Processa comando de ajuste de pitch
fn process_pitch_adjustment(
    key: i32,
    audio_player: &AudioPlayer,
    is_audio_playing: bool,
    audio_file: &str,
) -> Result<Option<bool>, Box<dyn Error>> {
    if let Some(factor) = get_pitch_factor(key) {
        let new_pitch = commands::clamp_pitch(
            audio_player.get_pitch() * factor,
            config::MIN_PITCH,
            config::MAX_PITCH
        );
        
        let current_pitch = audio_player.get_pitch();
        if (new_pitch - current_pitch).abs() > 0.001 {
            println!("{} para: {:.2}", get_pitch_adjustment_message(factor), new_pitch);
            let playing = adjust_pitch_with_restart(audio_player, new_pitch, is_audio_playing, audio_file)?;
            return Ok(Some(playing));
        }
    }
    
    Ok(None)
}

/// Alterna entre reproduzir e parar o áudio
fn toggle_audio_playback(
    audio_player: &AudioPlayer,
    is_audio_playing: bool,
    audio_file: &str,
) -> Result<bool, Box<dyn Error>> {
    if is_audio_playing {
        audio_player.stop();
        println!("⏹️  Áudio parado");
        Ok(false)
    } else {
        audio_player.play_file(audio_file)?;
        println!("▶️  Áudio iniciado (pitch: {:.2})", audio_player.get_pitch());
        Ok(true)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("🎬 === Projeto Visão + Gestos + Áudio ===");
    println!("🎮 Controles:");
    println!("  ESC     - Sair");
    println!("  ESPAÇO  - Iniciar/Parar áudio");
    println!("  + / -   - Ajuste fino de pitch");
    println!("  R       - Resetar pitch para normal");
    println!("  P       - Efeito sonoro rápido");
    println!();

    // Initialize audio system
    let audio_player = AudioPlayer::new()?;
    println!("🔊 Sistema de áudio inicializado com sucesso");

    // Find audio file
    let audio_file = match config::find_audio_file() {
        Some(path) => {
            println!("🎵 Arquivo de áudio encontrado: {}", path);
            path
        }
        None => {
            println!("⚠️  Nenhum arquivo de áudio encontrado");
            println!("📁 Adicione um arquivo audio.mp3 na pasta do projeto");
            "audio.mp3".to_string()
        }
    };

    // Initialize camera or video
    let (mut cam, is_camera) = camera::initialize_capture()?;

    // Create window
    named_window("Video", WINDOW_AUTOSIZE)?;

    println!("🎥 Iniciando captura de vídeo...");
    println!("🎵 Pitch atual: {:.2}", audio_player.get_pitch());
    println!();

    let mut is_audio_playing = false;

    // Main loop
    loop {
        // Capture frame
        let mut frame = Mat::default();
        if !camera::read_frame(&mut cam, &mut frame, is_camera)? {
            println!("📹 Fim do vídeo/câmera");
            break;
        }

        // Show frame
        imshow("Video", &frame)?;

        // Check for key presses
        let key = wait_key(30)?;
        let command = key_to_command(key);

        match command {
            KeyCommand::Exit => {
                println!("\n✅ Programa encerrado!");
                audio_player.stop();
                break;
            }
            KeyCommand::ToggleAudio => {
                is_audio_playing = toggle_audio_playback(&audio_player, is_audio_playing, &audio_file)?;
            }
            KeyCommand::ResetPitch => {
                audio_player.set_pitch(config::DEFAULT_PITCH);
                println!("🔄 Pitch resetado para 1.0");
                
                if is_audio_playing {
                    audio_player.stop();
                    audio_player.play_file(&audio_file)?;
                    println!("🔁 Áudio reiniciado com pitch normal");
                }
            }
            KeyCommand::AdjustPitch(_) => {
                if let Ok(Some(playing)) = process_pitch_adjustment(
                    key, &audio_player, is_audio_playing, &audio_file
                ) {
                    is_audio_playing = playing;
                }
            }
            KeyCommand::None => {} // No action for other keys
        }
    }

    println!("🎉 Até logo!");
    Ok(())
}