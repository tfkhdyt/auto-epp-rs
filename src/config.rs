use std::{
    fs::{self, File},
    io::{self, BufRead},
    path::Path,
    process,
};

use crate::utils::fs::write_file;

const CONFIG_PATH: &str = "/etc/auto-epp-rs.conf";
const DEFAULT_CONFIG: &str = "# see available epp state by running: cat /sys/devices/system/cpu/cpu0/cpufreq/energy_performance_available_preferences
[Settings]
epp_state_for_AC=performance
epp_state_for_BAT=power
";
pub const DEFAULT_GOVERNOR: &str = "powersave";

pub fn read_config() -> (String, String) {
    if fs::metadata(CONFIG_PATH).is_err() {
        if let Err(err) = write_file(Path::new(&CONFIG_PATH), DEFAULT_CONFIG) {
            eprintln!("Error: Failed to create config file: {}", err);
            process::exit(1);
        }
    }

    let config_file = File::open(CONFIG_PATH).unwrap_or_else(|err| {
        eprintln!("Error: Failed to open config file: {}", err);
        process::exit(1);
    });

    let reader = io::BufReader::new(config_file);

    let mut epp_state_for_ac = String::new();
    let mut epp_state_for_bat = String::new();

    for line in reader.lines() {
        let ln = line.unwrap_or_else(|err| {
            eprintln!("Error: Failed to open config file: {}", err);
            process::exit(1);
        });

        if ln.starts_with("epp_state_for_AC") {
            epp_state_for_ac = ln.split('=').collect::<Vec<&str>>()[1].to_owned();
        } else if ln.starts_with("epp_state_for_BAT") {
            epp_state_for_bat = ln.split('=').collect::<Vec<&str>>()[1].to_owned();
        }
    }

    (epp_state_for_ac, epp_state_for_bat)
}
