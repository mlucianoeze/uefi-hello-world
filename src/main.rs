#![no_main]
#![feature(restricted_std)]

use std::fmt::Write;

use uefi::prelude::*;
use uefi::proto::console::text::{Key, ScanCode};
use uefi::proto::console::text::Color::{Black, LightGray, Red};

use crate::miniapp::{MiniApp, MiniAppError, MiniAppExitCode};
use crate::miniapp::MiniAppError::SelectorIllegalOption;
use crate::miniapp::MiniAppExitCode::{ExitApp, RestartApp, Shutdown};
use crate::miniapp::notepad::miniapp_notepad;
use crate::uefi_utils::shutdown;

mod miniapp;
mod uefi_utils;

#[no_mangle]
pub extern "C" fn efi_main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    let app = MiniApp {
        title: "Hola mundo",
        app: miniapp_selector
    };
    let mut prev_error: Option<MiniAppError> = None;
    loop {
        let menu_result = app.load(&mut system_table, &prev_error);
        prev_error = None;
        match menu_result {
            Err(error) => prev_error = Some(error),
            Ok(menu_exit_code) => {
                match menu_exit_code {
                    ExitApp => return Status::SUCCESS,
                    RestartApp => continue,
                    Shutdown => shutdown(&mut system_table)
                }
            }
        }

    }
}

fn miniapp_selector(system_table: &mut SystemTable<Boot>, prev_error: &Option<MiniAppError>) -> Result<MiniAppExitCode, MiniAppError> {
    let miniapps = get_miniapps();
    let cursor_visible = system_table.stdout().cursor_visible().clone();
    system_table.stdout().enable_cursor(true).unwrap();
    print_menu(system_table, prev_error, miniapps);
    let cursor_position_escmsg = system_table.stdout().cursor_position().clone();
    let total_rows = system_table.stdout().current_mode()
        .unwrap().unwrap()
        .rows();
    system_table.stdout().set_cursor_position(0, total_rows - 1).unwrap();
    system_table.stdout().write_str("[ESC] Apagar").unwrap();
    system_table.stdout()
        .set_cursor_position(cursor_position_escmsg.0, cursor_position_escmsg.1)
        .unwrap();
    let mut events = unsafe { [system_table.stdin().wait_for_key_event().unsafe_clone()] };
    system_table.boot_services().wait_for_event(&mut events).discard_errdata().unwrap();
    let key = system_table.stdin().read_key().unwrap().unwrap();
    match key {
        Key::Printable(c) => {
            return match char::from(c) {
                '\r' => Ok(ExitApp),
                '\n' => Ok(ExitApp),
                '0' => Ok(ExitApp),
                c => {
                    let num = c.to_digit(10);
                    match num {
                        None => return Err(SelectorIllegalOption),
                        Some(_) => {}
                    }
                    let miniapp = miniapps.get((num.unwrap() - 1) as usize);
                    match miniapp {
                        None => return Err(SelectorIllegalOption),
                        Some(app) => {
                            system_table.stdout().enable_cursor(cursor_visible.clone()).unwrap();
                            let result = load_miniapp(system_table, app);
                            match result {
                                Err(error) => return Err(error),
                                Ok(_exit_code) => Ok(RestartApp)
                            }
                        }
                    }
                }
            }
        },
        Key::Special(ScanCode::ESCAPE) => Ok(Shutdown),
        _ => Ok(RestartApp)
    }
}

fn print_menu(system_table: &mut SystemTable<Boot>, prev_error: &Option<MiniAppError>, miniapps: [MiniApp; 1]) {
    system_table.stdout().write_str("Podés elegir una mini-app para ejecutar:\n").unwrap();
    for (i, miniapp) in miniapps.iter().enumerate() {
        system_table.stdout().write_fmt(format_args!("{}. {}\n", i + 1,
                                                     miniapp.title)).unwrap();
    }
    system_table.stdout().write_str("\n").unwrap();
    system_table.stdout().write_str("0. Al setup\n").unwrap();
    system_table.stdout().write_str("\n").unwrap();
    print_errmsg(system_table, prev_error);
    system_table.stdout().write_str("Seleccionar [0]: ").unwrap();
}

fn load_miniapp(system_table: &mut SystemTable<Boot>, miniapp: &MiniApp) -> Result<MiniAppExitCode, MiniAppError> {
    loop {
        let app_result = miniapp.load(system_table, &None);
        match &app_result {
            Err(_) => return app_result,
            Ok(value) => {
                match value {
                    ExitApp => return app_result,
                    _ => continue
                }
            }
        }
    }
}

fn get_miniapps() -> [MiniApp<'static>; 1] {
    [
        MiniApp {
            title: "Notepad (que no guarda nada)",
            app: miniapp_notepad
        }
    ]
}

fn print_errmsg(system_table: &mut SystemTable<Boot>, error: &Option<MiniAppError>) {
    system_table.stdout().set_color(Red, Black).unwrap();
    match error {
        None => system_table.stdout().write_str("\n").unwrap(),
        Some(err) => system_table.stdout().write_fmt(format_args!("{}\n", err.to_str())).unwrap()
    }
    system_table.stdout().set_color(LightGray, Black).unwrap();
}

