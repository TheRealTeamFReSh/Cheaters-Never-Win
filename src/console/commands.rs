use crate::{cheat_codes::CheatCodeResource, states::GameStates};

use super::{event::*, ConsoleData};
use bevy::prelude::*;

pub fn command_handler(
    mut cmd_reader: EventReader<SendCommandEvent>,
    mut print_to_console: EventWriter<PrintToConsoleEvent>,
    mut data: ResMut<ConsoleData>,
    mut game_state: ResMut<State<GameStates>>,
    mut cheat_codes_res: ResMut<CheatCodeResource>,
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
            "use" => {
                print_to_console.send(PrintToConsoleEvent(format!(
                    "Activating cheat code: <{}>...",
                    args[1]
                )));
                print_to_console.send(PrintToConsoleEvent(format!(
                    "Activation result: {}",
                    cheat_codes_res.activate_code(args[1]).to_string()
                )));
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
