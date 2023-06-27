pub mod notepad;

use uefi::prelude::*;
use uefi::proto::console::text::{Output};
use uefi::proto::console::text::Color::{Black, LightGray};
use std::fmt::Write;

pub enum MiniAppExitCode {
    RestartApp,
    ExitApp,
    Shutdown
}

#[derive(Debug)]
pub enum MiniAppError {
    SelectorIllegalOption
}

impl MiniAppError {
    pub fn to_str(&self) -> &'static str {
        match self {
            MiniAppError::SelectorIllegalOption => "Opción incorrecta"
        }
    }
}

#[derive(Copy, Clone)]
pub struct MiniApp<'a> {
    pub title: &'a str,
    pub app: fn(&mut SystemTable<Boot>, error: &Option<MiniAppError>) -> Result<MiniAppExitCode, MiniAppError>
}

impl MiniApp<'_> {
    pub fn ribbon(&self, stdout: &mut Output) {
        stdout.clear().unwrap();
        stdout.set_color(Black, LightGray).unwrap();
        stdout.write_str(" ").unwrap();
        stdout.write_str(&self.title).unwrap();
        loop {
            if stdout.cursor_position().0 == 0 {
                break;
            }
            stdout.write_str(" ").unwrap();
        }
        stdout.set_color(LightGray, Black).unwrap();
    }

    pub fn load(&self, system_table: &mut SystemTable<Boot>, error: &Option<MiniAppError>) -> Result<MiniAppExitCode, MiniAppError> {
        self.ribbon(system_table.stdout());
        (self.app)(system_table, error)
    }
}
