use std::time::Duration;

use bevy::prelude::*;

use crate::{cheat_codes::CheatCodeResource, toast::ShowToast};

use super::{CheatCodeActivatedEvent, ConsoleData};

pub struct PrintToConsoleEvent(pub String);
pub struct SendCommandEvent(pub String);

pub fn add_message_events_to_console(
    mut data: ResMut<ConsoleData>,
    mut ev_console_message: EventReader<PrintToConsoleEvent>,
) {
    for PrintToConsoleEvent(message) in ev_console_message.iter() {
        data.lines.push(message.clone());
    }
}

pub fn show_help_text(
    mut ev_reader: EventReader<CheatCodeActivatedEvent>,
    mut ev_writer: EventWriter<ShowToast>,
    cheat_code_res: Res<CheatCodeResource>,
) {
    for CheatCodeActivatedEvent(kind) in ev_reader.iter() {
        let code = cheat_code_res.codes.get(kind).unwrap();
        ev_writer.send(ShowToast {
            value: code.help_text.clone(),
            duration: Duration::from_secs(5),
        })
    }
}
