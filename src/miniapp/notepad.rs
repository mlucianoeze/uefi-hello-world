use std::fmt::Write;
use uefi::prelude::{Boot, SystemTable};
use uefi::proto::console::text::{Key, OutputMode, ScanCode};
use uefi::ResultExt;
use crate::miniapp::{MiniAppError, MiniAppExitCode};
use crate::miniapp::MiniAppExitCode::ExitApp;

pub fn miniapp_notepad(system_table: &mut SystemTable<Boot>, _: &Option<MiniAppError>) -> Result<MiniAppExitCode, MiniAppError> {
    let output_mode = system_table.stdout().current_mode().unwrap().unwrap();
    loop {
        let mut events = unsafe { [system_table.stdin().wait_for_key_event().unsafe_clone()] };
        system_table.boot_services().wait_for_event(&mut events).discard_errdata().unwrap();
        let key = system_table.stdin().read_key().unwrap().unwrap();
        match key {
            Key::Printable(c) => {
                let character = char::from(c);
                match &character {
                    '\r' => system_table.stdout().write_str("\n").unwrap(),
                    '\n' => system_table.stdout().write_str("\n").unwrap(),
                    '\x08' => borrar(system_table, &output_mode),
                    c => system_table.stdout().write_fmt(format_args!("{}", c)).unwrap()
                }
            },
            Key::Special(sc) => {
                match sc {
                    ScanCode::ESCAPE => break,
                    _ => {}
                }
            }
        }
    }
    Ok(ExitApp)
}

fn borrar(system_table: &mut SystemTable<Boot>, output_mode: &OutputMode) {
    let cursor_position = &system_table.stdout().cursor_position();
    if cursor_position.0 == 0 && cursor_position.1 == 1 {
        return; // Para que no borre el ribbon
    }
    if cursor_position.0 == 0 && cursor_position.1 > 1 {
        system_table.stdout().set_cursor_position(output_mode.columns() - 1,
                                                  cursor_position.clone().1 - 1).unwrap();
        system_table.stdout().write_str(" ").unwrap();
        system_table.stdout().set_cursor_position(output_mode.columns() - 1,
                                                  cursor_position.clone().1 - 1).unwrap();
        return;
    }
    system_table.stdout().write_str("\x08").unwrap();
}
