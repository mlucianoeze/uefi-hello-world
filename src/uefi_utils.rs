use std::fmt::Write;
use uefi::prelude::{Boot, SystemTable};
use uefi::Status;

#[allow(dead_code)]
pub fn debug(message: &str, system_table: &mut SystemTable<Boot>) {
    system_table.stdout().write_str("DEBUG: ").unwrap();
    let micros_per_second: usize = 1_000_000;
    system_table.stdout().write_str(message).unwrap();
    system_table.stdout().write_str("\n").unwrap();
    system_table.boot_services().stall(micros_per_second.clone());
}

pub fn shutdown(system_table: &mut SystemTable<Boot>) {
    system_table.runtime_services().reset(
            uefi::table::runtime::ResetType::SHUTDOWN,
            Status::SUCCESS,
            None)
}