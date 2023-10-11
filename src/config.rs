use std::{fs, path::Path, process};

use crate::utils::fs::{read_file, write_file};

const CONFIG_PATH: &str = "/etc/auto-epp-rs.conf";
const DEFAULT_CONFIG: &str = "# see available epp state by running: cat /sys/devices/system/cpu/cpu0/cpufreq/energy_performance_available_preferences
[Settings]
epp_state_for_AC=performance
epp_state_for_BAT=power
";

pub fn read_config() -> (String, String) {
    if fs::metadata(CONFIG_PATH).is_err() {
        if let Err(err) = write_file(Path::new(&CONFIG_PATH), DEFAULT_CONFIG) {
            eprintln!("Error: Failed to create config file: {}", err);
            process::exit(1);
        }
    }

    let config = read_file(Path::new(&CONFIG_PATH)).unwrap_or_else(|err| {
        eprintln!("Error: Unable to read config file: {}", err);
        process::exit(1);
    });

    let config_lines = config.lines();
    let mut epp_state_for_ac = "";
    let mut epp_state_for_bat = "";

    config_lines.into_iter().for_each(|line| {
        if line.starts_with("epp_state_for_AC") {
            epp_state_for_ac = line.split('=').collect::<Vec<&str>>()[1];
        } else if line.starts_with("epp_state_for_BAT") {
            epp_state_for_bat = line.split('=').collect::<Vec<&str>>()[1];
        }
    });

    (epp_state_for_ac.to_owned(), epp_state_for_bat.to_owned())
}
