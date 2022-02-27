use bevy::{input::keyboard::KeyboardInput, prelude::*};

use super::event::SendCommandEvent;

pub fn handle_input_keys(
    mut data: ResMut<super::ConsoleData>,
    mut evr_keys: EventReader<KeyboardInput>,
    mut send_command: EventWriter<SendCommandEvent>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    // don't do anything if control key is pressed
    if keyboard_input.pressed(KeyCode::LControl) || keyboard_input.pressed(KeyCode::RControl) {
        return;
    }

    for ev in evr_keys.iter() {
        if ev.state.is_pressed() {
            if let Some(key_code) = ev.key_code {
                match key_code {
                    KeyCode::Back => {
                        if !data.input.is_empty() {
                            data.input.pop();
                        }
                    }
                    KeyCode::Space => data.input.push(' '),
                    KeyCode::Tab => data.input.push_str("  "),
                    KeyCode::Comma => data.input.push(','),
                    KeyCode::Colon => data.input.push(':'),
                    KeyCode::Semicolon => data.input.push(';'),
                    KeyCode::Apostrophe => data.input.push('\''),
                    KeyCode::At => data.input.push('@'),
                    KeyCode::LBracket => data.input.push('['),
                    KeyCode::RBracket => data.input.push(']'),
                    KeyCode::Minus | KeyCode::NumpadSubtract => data.input.push('-'),
                    KeyCode::Period | KeyCode::NumpadDecimal => data.input.push('.'),
                    KeyCode::Asterisk | KeyCode::NumpadMultiply => data.input.push('*'),
                    KeyCode::Slash | KeyCode::NumpadDivide => data.input.push('/'),
                    KeyCode::Plus | KeyCode::NumpadAdd => data.input.push('+'),
                    KeyCode::Key0 | KeyCode::Numpad0 => data.input.push('0'),
                    KeyCode::Key1 | KeyCode::Numpad1 => data.input.push('1'),
                    KeyCode::Key2 | KeyCode::Numpad2 => data.input.push('2'),
                    KeyCode::Key3 | KeyCode::Numpad3 => data.input.push('3'),
                    KeyCode::Key4 | KeyCode::Numpad4 => data.input.push('4'),
                    KeyCode::Key5 | KeyCode::Numpad5 => data.input.push('5'),
                    KeyCode::Key6 | KeyCode::Numpad6 => data.input.push('6'),
                    KeyCode::Key7 | KeyCode::Numpad7 => data.input.push('7'),
                    KeyCode::Key8 | KeyCode::Numpad8 => data.input.push('8'),
                    KeyCode::Key9 | KeyCode::Numpad9 => data.input.push('9'),

                    KeyCode::LShift
                    | KeyCode::RShift
                    | KeyCode::Escape
                    | KeyCode::LAlt
                    | KeyCode::RAlt
                    | KeyCode::LControl
                    | KeyCode::RControl
                    | KeyCode::F1
                    | KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Right
                    | KeyCode::Left
                    | KeyCode::F2
                    | KeyCode::F3
                    | KeyCode::F4
                    | KeyCode::F5
                    | KeyCode::F6
                    | KeyCode::F7
                    | KeyCode::F8
                    | KeyCode::F9
                    | KeyCode::F10
                    | KeyCode::F11
                    | KeyCode::F12
                    | KeyCode::Insert
                    | KeyCode::Delete
                    | KeyCode::Grave
                    | KeyCode::Backslash => {}

                    KeyCode::Return => {
                        // sending the command
                        send_command.send(SendCommandEvent(data.input.clone()));
                        // clearing the input
                        data.input.clear();
                    }
                    _ => {
                        let key_code_str = if keyboard_input.pressed(KeyCode::LShift)
                            || keyboard_input.pressed(KeyCode::RShift)
                        {
                            format!("{:?}", key_code).to_uppercase()
                        } else {
                            format!("{:?}", key_code).to_lowercase()
                        };

                        trace!("Pressed key: {:?}", key_code_str);
                        data.input.push_str(&key_code_str);
                    }
                }
            }
        }
    }
}
