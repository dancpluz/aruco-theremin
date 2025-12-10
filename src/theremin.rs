use rodio::{OutputStream, OutputStreamBuilder, Sink, Source};
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::time::Duration;

struct ThereminState {
    amplitude: f32,
    frequency: f32,
    enabled: bool,
}

pub struct ThereminSource {
    state: Arc<Mutex<ThereminState>>,
    sample_rate: u32,
    phase: f32,
}

impl ThereminSource {
    pub fn new(amplitude: f32, frequency: f32, sample_rate: u32) -> Self {
        let state = Arc::new(Mutex::new(ThereminState {
            amplitude,
            frequency,
            enabled: true,
        }));
        Self {
            state,
            sample_rate,
            phase: 0.0,
        }
    }

    pub fn update_parameters(&self, amplitude: f32, frequency: f32) {
        let mut state = self.state.lock().unwrap();
        state.amplitude = amplitude;
        state.frequency = frequency;
    }

    pub fn set_enabled(&self, enabled: bool) {
        let mut state = self.state.lock().unwrap();
        state.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        let state = self.state.lock().unwrap();
        state.enabled
    }

    pub fn get_amplitude(&self) -> f32 {
        let state = self.state.lock().unwrap();
        state.amplitude
    }

    pub fn get_frequency(&self) -> f32 {
        let state = self.state.lock().unwrap();
        state.frequency
    }

    fn generate_sample(&mut self) -> f32 {
        let (amplitude, frequency, enabled) = {
            let state = self.state.lock().unwrap();
            (state.amplitude, state.frequency, state.enabled)
        };

        if !enabled {
            return 0.0;
        }

        self.phase += 2.0 * PI * frequency / self.sample_rate as f32;
        if self.phase > 2.0 * PI {
            self.phase -= 2.0 * PI;
        }

        let sample = self.phase.sin() * amplitude;
        sample.clamp(-0.8, 0.8)
    }
}

impl Iterator for ThereminSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.generate_sample())
    }
}

impl Source for ThereminSource {
    fn channels(&self) -> u16 {
        2 // Estéreo
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

pub struct ThereminController {
    _stream: OutputStream,
    sink: Sink,
    source: ThereminSource,
    last_amplitude: f32,
    last_frequency: f32,
}

impl ThereminController {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let stream_handle = OutputStreamBuilder::open_default_stream()?;
        let sink = Sink::connect_new(&stream_handle.mixer());

        let source = ThereminSource::new(0.5, 440.0, 44100);
        sink.append(source.clone());
        sink.set_volume(0.7);

        Ok(ThereminController {
            _stream: stream_handle,
            sink,
            source,
            last_amplitude: 0.5,
            last_frequency: 440.0,
        })
    }

    pub fn update_from_position(&mut self, x: f32, y: f32) {
        let (frequency, amplitude) = Self::map_position_to_audio(x, y);

        // últimos valores mesmo se o som estiver desativado
        self.last_amplitude = amplitude;
        self.last_frequency = frequency;

        // atualiza o som apenas se estiver ativado
        if self.is_enabled() {
            self.source.update_parameters(amplitude, frequency);
        }
    }

    fn map_position_to_audio(x: f32, y: f32) -> (f32, f32) {
        let amplitude = match x {
            x if (-1.0 <= x) && (x < -0.80) => 0.10,
            x if (-0.80 <= x) && (x < -0.60) => 0.20,
            x if (-0.60 <= x) && (x < -0.40) => 0.30,
            x if (-0.40 <= x) && (x < -0.20) => 0.40,
            x if (-0.20 <= x) && (x < 0.0) => 0.50,
            x if (0.0 <= x) && (x < 0.20) => 0.60,
            x if (0.20 <= x) && (x < 0.40) => 0.70,
            x if (0.40 <= x) && (x < 0.60) => 0.80,
            x if (0.60 <= x) && (x < 0.80) => 0.90,
            x if (0.80 <= x) && (x <= 1.0) => 1.00,
            _ => 0.5,
        };

        let frequency = match y {
            y if (-1.0 <= y) && (y < -0.80) => 130.81,  // C3
            y if (-0.80 <= y) && (y < -0.60) => 146.83, // D3
            y if (-0.60 <= y) && (y < -0.40) => 164.81, // E3
            y if (-0.40 <= y) && (y < -0.20) => 196.00, // G3
            y if (-0.20 <= y) && (y < 0.0) => 220.00,   // A3
            y if (0.0 <= y) && (y < 0.20) => 261.63,    // C4
            y if (0.20 <= y) && (y < 0.40) => 293.66,   // D4
            y if (0.40 <= y) && (y < 0.60) => 329.63,   // E4
            y if (0.60 <= y) && (y < 0.80) => 392.00,   // G4
            y if (0.80 <= y) && (y <= 1.0) => 440.00,   // A4
            _ => 440.0,
        };

        (frequency, amplitude)
    }

    pub fn toggle_sound(&mut self) {
        let enabled = !self.is_enabled();
        self.source.set_enabled(enabled);

        if enabled {
            self.source
                .update_parameters(self.last_amplitude, self.last_frequency);
            println!("[ON] Som ativado");
        } else {
            println!("[OFF] Som desativado");
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.source.is_enabled()
    }

    pub fn get_frequency(&self) -> f32 {
        self.source.get_frequency()
    }

    pub fn get_amplitude(&self) -> f32 {
        self.source.get_amplitude()
    }

    pub fn stop(&mut self) {
        self.sink.stop();
    }
}

impl Clone for ThereminSource {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            sample_rate: self.sample_rate,
            phase: 0.0, // fase não é compartilhada
        }
    }
}
