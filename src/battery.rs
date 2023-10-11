use std::{fs, path::Path, process};

use crate::utils::fs::read_file;

pub fn check_charging_status() -> bool {
    let power_supply_path = Path::new("/sys/class/power_supply/");
    let entries = fs::read_dir(power_supply_path).unwrap_or_else(|err| {
        eprintln!("Error: Failed to read power supply directory: {}", err);
        process::exit(1);
    });

    for file in entries.into_iter() {
        let entry = file.unwrap_or_else(|err| {
            eprintln!(
                "Error: Failed to read power supply directory entry: {}",
                err
            );
            process::exit(1);
        });
        let path = entry.path();

        if path.is_dir() || path.is_symlink() {
            let Ok(power_type) = read_file(&path.join("type")) else {
                continue;
            };

            if power_type.trim() == "Mains" {
                let Ok(online_data) = read_file(&path.join("online")) else {
                    continue;
                };
                let Ok(online_status) = online_data.parse::<u8>() else {
                    continue;
                };
                if online_status == 1 {
                    return true;
                }
            } else if power_type.trim() == "Battery" {
                let Ok(status_data) = read_file(&path.join("status")) else {
                    continue;
                };
                if status_data.trim() == "Discharging" {
                    return false;
                }
            }
        }
    }

    true
}
