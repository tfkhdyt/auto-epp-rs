use std::{path::Path, process};

use crate::utils::fs::read_file;

pub fn check_driver() {
    let scaling_driver_path = Path::new("/sys/devices/system/cpu/cpu0/cpufreq/scaling_driver");

    let scaling_driver = read_file(scaling_driver_path).unwrap_or_else(|err| {
        eprintln!("Error: Unable to read scaling driver file: {}", err);
        process::exit(1);
    });

    if scaling_driver.trim() != "amd-pstate-epp" {
        eprintln!("Error: The system is not running amd-pstate-epp");
        process::exit(1);
    }
}
