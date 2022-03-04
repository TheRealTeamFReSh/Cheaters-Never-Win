use crate::cheat_codes::CheatCodeActivationResult;
use crate::runner::CollectedChars;
use crate::{cheat_codes::CheatCodeResource, states::GameStates};
use bevy_kira_audio::{Audio, AudioChannel};

use super::{event::*, CheatCodeActivatedEvent, ConsoleData};
use bevy::prelude::*;

pub fn command_handler(
    mut cmd_reader: EventReader<SendCommandEvent>,
    mut print_to_console: EventWriter<PrintToConsoleEvent>,
    mut data: ResMut<ConsoleData>,
    mut game_state: ResMut<State<GameStates>>,
    mut cheat_codes_res: ResMut<CheatCodeResource>,
    mut collected_chars: ResMut<CollectedChars>,
    mut ev_writer: EventWriter<CheatCodeActivatedEvent>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    for SendCommandEvent(command) in cmd_reader.iter() {
        // skip if the command is empty
        if command.is_empty() {
            return;
        }

        // extracting args
        let args: Vec<&str> = command.trim().split(' ').collect();

        // show the command entered by the user if it's not a clear
        if args[0] != "clear" {
            let mut user_input = String::from("> ");
            user_input.push_str(command.clone().trim());
            print_to_console.send(PrintToConsoleEvent(user_input));
        }

        // dispatch the command
        match args[0] {
            "clear" => data.lines.clear(),
            "help" => print_to_console.send(PrintToConsoleEvent(super::utils::display_help())),
            "cheat" => {
                print_to_console.send(PrintToConsoleEvent(format!(
                    "Activating cheat code: <{}>...",
                    args[1]
                )));

                let can_activate = is_valid_cheat(&mut collected_chars, args[1], &cheat_codes_res);

                if can_activate {
                    let activation_res = cheat_codes_res.activate_code(args[1]);
                    print_to_console.send(PrintToConsoleEvent(format!(
                        "Activation result: {}",
                        activation_res.repr()
                    )));

                    if let CheatCodeActivationResult::Activated(kind) = activation_res {
                        ev_writer.send(CheatCodeActivatedEvent(kind));
                        let audio_channel = AudioChannel::new("sfx-channel".to_owned());
                        audio.set_volume_in_channel(2.0, &audio_channel);
                        audio.play_in_channel(asset_server.load("powerup.ogg"), &audio_channel);
                    }
                } else {
                    print_to_console.send(PrintToConsoleEvent(format!(
                        "Failed to activate. Need more information."
                    )));
                }
            }
            "exit" => {
                print_to_console.send(PrintToConsoleEvent("Closing session...".to_string()));
                game_state.pop().unwrap();
            }
            _ => {
                print_to_console.send(PrintToConsoleEvent(format!(
                    "Command \"{}\" not found.\nType \"help\" to print the list of available commands.",
                    args[0]
                )));
            }
        }
    }
}

pub fn is_valid_cheat(
    collected_chars: &mut CollectedChars,
    code_text: &str,
    cheat_codes: &CheatCodeResource,
) -> bool {
    let original_collected_chars = collected_chars.values.clone();
    let original_collected_chars_map = collected_chars.values_map.clone();

    let cheats = cheat_codes.codes.clone();
    let code = cheats
        .into_iter()
        .find(|(_kind, code)| code.text == code_text);

    if let Some((_, cheat)) = code {
        for ch in cheat.text.chars() {
            let index_opt = collected_chars.values.iter().position(|val| *val == ch);

            if let Some(index) = index_opt {
                collected_chars.values.remove(index);

                // Update values map
                let char_entry = collected_chars.values_map.get(&ch);
                if let Some(_count) = char_entry {
                    *collected_chars.values_map.get_mut(&ch).unwrap() -= 1;
                }
            } else {
                collected_chars.values = original_collected_chars;
                collected_chars.values_map = original_collected_chars_map;
                return false;
            }
        }
        return true;
    }

    collected_chars.values = original_collected_chars;
    collected_chars.values_map = original_collected_chars_map;
    false
}
