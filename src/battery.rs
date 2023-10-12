use std::{fs, io, path::Path};

use crate::utils::fs::read_file;

pub fn is_charging() -> io::Result<bool> {
    let power_supply_path = Path::new("/sys/class/power_supply/");
    let entries = fs::read_dir(power_supply_path)?;

    for file in entries.into_iter() {
        let entry = file?;
        let path = entry.path();

        if path.is_dir() || path.is_symlink() {
            let Ok(power_type) = read_file(&path.join("type")) else {
                continue;
            };

            if power_type == "Mains" {
                let Ok(online_data) = read_file(&path.join("online")) else {
                    continue;
                };
                let Ok(online_status) = online_data.parse::<u8>() else {
                    continue;
                };
                if online_status == 1 {
                    return Ok(true);
                }
            } else if power_type == "Battery" {
                let Ok(status_data) = read_file(&path.join("status")) else {
                    continue;
                };
                if status_data == "Discharging" {
                    return Ok(false);
                }
            }
        }
    }

    Ok(true)
}
