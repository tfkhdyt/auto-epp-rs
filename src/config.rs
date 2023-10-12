use std::{
    fs::{self, File},
    io::{self, BufRead},
    path::Path,
};

use crate::utils::fs::write_file;

const CONFIG_PATH: &str = "/etc/auto-epp-rs.conf";
const DEFAULT_CONFIG: &str = "# see available epp state by running: cat /sys/devices/system/cpu/cpu0/cpufreq/energy_performance_available_preferences
[Settings]
epp_state_for_AC=performance
epp_state_for_BAT=power
";
pub const DEFAULT_GOVERNOR: &str = "powersave";

#[derive(Default)]
pub struct EPPState {
    pub ac: String,
    pub bat: String,
}

pub fn get_epp_state() -> io::Result<EPPState> {
    if fs::metadata(CONFIG_PATH).is_err() {
        write_file(Path::new(&CONFIG_PATH), DEFAULT_CONFIG)?;
    }

    let config_file = File::open(CONFIG_PATH)?;
    let reader = io::BufReader::new(config_file);
    let mut epp_state = EPPState::default();

    for line in reader.lines() {
        let ln = line?;

        if ln.starts_with("epp_state_for_AC") {
            epp_state.ac = ln.split('=').collect::<Vec<&str>>()[1].to_owned();
        } else if ln.starts_with("epp_state_for_BAT") {
            epp_state.bat = ln.split('=').collect::<Vec<&str>>()[1].to_owned();
        }
    }

    Ok(epp_state)
}
