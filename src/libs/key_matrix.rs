use macroquad::prelude::*;

pub fn key_matrix(row: usize) -> u8 {
    static KEYCODES_DATA: [&[KeyCode]; 11] = [
        &[
            KeyCode::Key7,
            KeyCode::Key6,
            KeyCode::Key5,
            KeyCode::Key4,
            KeyCode::Key3,
            KeyCode::Key2,
            KeyCode::Key1,
            KeyCode::Key0,
        ],
        &[
            KeyCode::Semicolon,
            KeyCode::RightBracket,
            KeyCode::LeftBracket,
            KeyCode::F11,
            KeyCode::Equal,
            KeyCode::Minus,
            KeyCode::Key9,
            KeyCode::Key8,
        ],
        &[
            KeyCode::B,
            KeyCode::A,
            KeyCode::F11,
            KeyCode::Slash,
            KeyCode::Period,
            KeyCode::Comma,
            KeyCode::F11,
            KeyCode::F8,
        ],
        &[
            KeyCode::J,
            KeyCode::I,
            KeyCode::H,
            KeyCode::G,
            KeyCode::F,
            KeyCode::E,
            KeyCode::D,
            KeyCode::C,
        ],
        &[
            KeyCode::R,
            KeyCode::Q,
            KeyCode::P,
            KeyCode::O,
            KeyCode::N,
            KeyCode::M,
            KeyCode::L,
            KeyCode::K,
        ],
        &[
            KeyCode::Z,
            KeyCode::Y,
            KeyCode::X,
            KeyCode::W,
            KeyCode::V,
            KeyCode::U,
            KeyCode::T,
            KeyCode::S,
        ],
        &[
            KeyCode::F3,
            KeyCode::F2,
            KeyCode::F1,
            KeyCode::F11,
            KeyCode::F11,
            KeyCode::F11,
            KeyCode::LeftControl,
            KeyCode::LeftShift,
        ],
        &[
            KeyCode::Enter,
            KeyCode::F11,
            KeyCode::Backspace,
            KeyCode::F11,
            KeyCode::F11,
            KeyCode::F11,
            KeyCode::F5,
            KeyCode::F4,
        ],
        &[
            KeyCode::Right,
            KeyCode::Down,
            KeyCode::Up,
            KeyCode::Left,
            KeyCode::Delete,
            KeyCode::Insert,
            KeyCode::Home,
            KeyCode::Space,
        ],
        &[
            KeyCode::F11,
            KeyCode::F11,
            KeyCode::F11,
            KeyCode::F11,
            KeyCode::F11,
            KeyCode::F11,
            KeyCode::F11,
            KeyCode::F11,
        ],
        &[
            KeyCode::F11,
            KeyCode::F11,
            KeyCode::F11,
            KeyCode::F11,
            KeyCode::F11,
            KeyCode::F11,
            KeyCode::F11,
            KeyCode::F11,
        ],
    ];

    let mut result = 0xff_u8;
    if row < 11 {
        for i in 0..8 {
            if macroquad::input::is_key_down(KEYCODES_DATA[row][i]) {
                result &= !(1 << (7 - i));
            }
        }
    } else {
        log::error!("KeyMatrix: Tried to scan row > 10");
    }
    result
}
