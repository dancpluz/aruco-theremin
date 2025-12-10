use rodio::{OutputStream, Source, Sink};
use std::f32::consts::PI;
use std::time::Duration;
use std::io::{stdout, Write, Read};
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::thread;

fn normalizar_valores_pentatonica(ponto: (f32,f32)) -> (f32, f32){
    let  frequencia: f32;
    let  amplitude: f32;
    let x = ponto.0;
    let y = ponto.1;

    amplitude = match x {
        x if (-1.0 <= x) && (x < -0.80) => 0.10,  // 10%
        x if (-0.80 <= x) && (x < -0.60) => 0.20, // 20%
        x if (-0.60 <= x) && (x < -0.40) => 0.30, // 30%
        x if (-0.40 <= x) && (x < -0.20) => 0.40, // 40%
        x if (-0.20 <= x) && (x < 0.0) => 0.50,   // 50%
        x if (0.0 <= x) && (x < 0.20) => 0.60,    // 60%
        x if (0.20 <= x) && (x < 0.40) => 0.70,   // 70%
        x if (0.40 <= x) && (x < 0.60) => 0.80,   // 80%
        x if (0.60 <= x) && (x < 0.80) => 0.90,   // 90%
        x if (0.80 <= x) && (x <= 1.0) => 1.00,   // 100%
        _ => 0.5,
    };

    frequencia = match y {
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

    (frequencia, amplitude)
}

// Gera o som do theremin com amplitude (x) e frequência (y)
fn gerar_som_teremin(amplitude: f32, frequencia: f32, sample_rate: u32) -> ThereminSource {
    ThereminSource::new(amplitude, frequencia, sample_rate)
}

// Estado compartilhado para permitir controle em tempo real
struct EstadoAtualDoTheremin {
    amplitude: f32,
    frequency: f32,
}

struct ThereminSource {
    state: Arc<Mutex<EstadoAtualDoTheremin>>,
    sample_rate: u32,
    phase: f32,
}

impl ThereminSource {
    fn new(amplitude: f32, frequency: f32, sample_rate: u32) -> Self {
        let state = Arc::new(Mutex::new(EstadoAtualDoTheremin {
            amplitude,
            frequency,
        }));
        Self {
            state,
            sample_rate,
            phase: 0.0,
        }
    }

    fn generate_sample(&mut self) -> f32 {
        // Obtém os valores atuais do estado (permite mudanças em tempo real)
        let (amplitude, frequency) = {
            let state = self.state.lock().unwrap();
            (state.amplitude, state.frequency)
        };

        // Atualiza a fase do oscilador
        self.phase += 2.0 * PI * frequency / self.sample_rate as f32;
        if self.phase > 2.0 * PI {
            self.phase -= 2.0 * PI;
        }

        // Gera uma onda senoidal simples
        let sample = self.phase.sin() * amplitude;

        // Limita para evitar cliques
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
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        2 // Estéreo
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

// Lê os valores x e y do arquivo
fn ler_xy_do_arquivo() -> Option<(f32, f32)> {
    match File::open("theremin_input.txt") {
        Ok(mut file) => {
            let mut conteudo = String::new();
            if file.read_to_string(&mut conteudo).is_ok() {
                let partes: Vec<&str> = conteudo.trim().split_whitespace().collect();
                if partes.len() >= 2 {
                    if let (Ok(x), Ok(y)) = (partes[0].parse::<f32>(), partes[1].parse::<f32>()) {
                        return Some((x.clamp(-1.0, 1.0), y.clamp(-1.0, 1.0)));
                    }
                }
            }
        }
        Err(_) => {
            // Se o arquivo não existe, cria com valores padrão
            criar_arquivo_teste();
        }
    }
    None
}

// Cria um arquivo de teste com valores iniciais
fn criar_arquivo_teste() {
    if let Ok(mut file) = File::create("theremin_input.txt") {
        use std::io::Write;
        let _ = writeln!(file, "0.0 0.0");
    }
}

fn main() {
    // Inicializa o sistema de áudio
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Cria a fonte de áudio usando a função gerar_som_teremin
    let source = gerar_som_teremin(0.5, 440.0, 44100);

    // Extrai o estado compartilhado para poder modificar depois
    let state = source.state.clone();

    sink.append(source);
    sink.set_volume(0.7);

    // Limpa a tela
    print!("{}[2J", 27 as char);
    println!("=== Theremin com Controle X/Y Externo ===");
    println!("O theremin lê os valores X e Y do arquivo 'theremin_input.txt'");
    println!("Formato: dois valores float separados por espaço (ex: 0.5 -0.3)");
    println!("Valores devem estar entre -1.0 e 1.0");
    println!("Pressione Ctrl+C para sair");
    println!();

    // Garante que o arquivo de entrada exista
    if !std::path::Path::new("theremin_input.txt").exists() {
        criar_arquivo_teste();
    }

    // Variáveis para controle
    let mut ultimo_x: f32 = 0.0;
    let mut ultimo_y: f32 = 0.0;

    // Loop principal - lê valores do arquivo
    loop {
        // Tenta ler os valores do arquivo
        if let Some((x, y)) = ler_xy_do_arquivo() {
            // Só atualiza se os valores mudaram
            if (x - ultimo_x).abs() > 0.001 || (y - ultimo_y).abs() > 0.001 {
                ultimo_x = x;
                ultimo_y = y;

                // Usa a função normalizar para converter x,y em frequência e amplitude
                let (frequencia, amplitude) = normalizar_valores_pentatonica((x, y));

                // Atualiza o estado do theremin com os valores normalizados
                {
                    let mut estado = state.lock().unwrap();
                    estado.frequency = frequencia;
                    estado.amplitude = amplitude;
                }

                // Exibe informações atuais
                print!("{}[4;0H", 27 as char); // Move cursor para linha 4
                println!("Posição X: {:.2}, Y: {:.2} | Frequência: {:.1} Hz, Amplitude: {:.2}",
                         x, y, frequencia, amplitude);
                stdout().flush().unwrap();
            }
        }

        // Pequena pausa para não usar 100% da CPU
        thread::sleep(Duration::from_millis(50));
    }
}