use rodio::{Decoder, OutputStream, Sink, OutputStreamBuilder};
use rodio::source::Source;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

/// Reprodutor de áudio com pitch ajustável
pub struct AudioPlayer {
    stream: Arc<Mutex<OutputStream>>,
    main_sink: Arc<Mutex<Option<Sink>>>,
    current_pitch: Arc<Mutex<f32>>,
}

impl AudioPlayer {
    /// Cria um novo AudioPlayer
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let stream = OutputStreamBuilder::open_default_stream()?;
        
        Ok(AudioPlayer {
            stream: Arc::new(Mutex::new(stream)),
            main_sink: Arc::new(Mutex::new(None)),
            current_pitch: Arc::new(Mutex::new(1.0)),
        })
    }

    /// Carrega e toca um arquivo de áudio com pitch atual (em loop)
    pub fn play_file(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(file_path)?;
        let decoder = Decoder::try_from(BufReader::new(file))?;

        let pitch = *self.current_pitch.lock().unwrap();
        let pitched = decoder.speed(pitch);

        let stream = self.stream.lock().unwrap();
        let sink = Sink::connect_new(&stream.mixer());
        
        // Liberar lock antes de modificar o sink
        drop(stream);
        
        sink.append(pitched.repeat_infinite());
        
        // Substituir sink anterior
        let mut main_sink = self.main_sink.lock().unwrap();
        if let Some(old_sink) = main_sink.take() {
            old_sink.stop();
        }
        *main_sink = Some(sink);
        
        Ok(())
    }

    /// Define o pitch (1.0 = normal)
    pub fn set_pitch(&self, pitch: f32) {
        let clamped = pitch.clamp(0.25, 4.0);
        *self.current_pitch.lock().unwrap() = clamped;
        println!("🎵 Pitch alterado para {:.2}", clamped);
    }

    /// Obtém o pitch atual
    pub fn get_pitch(&self) -> f32 {
        *self.current_pitch.lock().unwrap()
    }

    /// Para toda reprodução de áudio
    pub fn stop(&self) {
        if let Some(s) = self.main_sink.lock().unwrap().as_mut() {
            s.stop();
        }
    }
}