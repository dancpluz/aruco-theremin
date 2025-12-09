//! Módulo para gerenciamento de comandos de teclado

/// Representa um comando de tecla
#[derive(Debug, Clone, Copy)]
pub enum KeyCommand {
    Exit,
    ToggleAudio,
    ResetPitch,
    AdjustPitch(f32), // Fator de ajuste
    None,
}

/// Mapeamento de teclas para comandos
pub const KEY_MAPPINGS: &[(i32, KeyCommand)] = &[
    (27, KeyCommand::Exit),           // ESC
    (32, KeyCommand::ToggleAudio),    // SPACE
    (114, KeyCommand::ResetPitch),    // 'r'
    (43, KeyCommand::AdjustPitch(1.05)), // '+' (fine)
    (61, KeyCommand::AdjustPitch(1.05)), // '=' (fine, Shift not pressed)
    (45, KeyCommand::AdjustPitch(0.95)), // '-' (fine)
    (95, KeyCommand::AdjustPitch(0.95)), // '_' (fine, Shift pressed)
];

/// Converte um código de tecla em um comando
pub fn key_to_command(key: i32) -> KeyCommand {
    for (key_code, command) in KEY_MAPPINGS {
        if key == *key_code {
            return *command;
        }
    }
    KeyCommand::None
}

/// Obtém o fator de ajuste de pitch baseado na tecla
pub fn get_pitch_factor(key: i32) -> Option<f32> {
    match key_to_command(key) {
        KeyCommand::AdjustPitch(factor) => Some(factor),
        _ => None,
    }
}

/// Gera uma mensagem descritiva para o ajuste de pitch
pub fn get_pitch_adjustment_message(factor: f32) -> String {
    match factor {
        f if f == 1.1 => String::from("⬆️  Pitch aumentado (grosso)"),
        f if f == 0.9 => String::from("⬇️  Pitch diminuído (grosso)"),
        f if f == 1.05 => String::from("🔧 Pitch aumentado (fino)"),
        f if f == 0.95 => String::from("🔧 Pitch diminuído (fino)"),
        _ => format!("🔄 Pitch ajustado por fator {:.2}", factor),
    }
}

/// Valida e aplica limite ao pitch
pub fn clamp_pitch(pitch: f32, min_pitch: f32, max_pitch: f32) -> f32 {
    pitch.clamp(min_pitch, max_pitch)
}