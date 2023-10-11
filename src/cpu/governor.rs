use std::{path::Path, process};

use crate::{
    config,
    utils::fs::{read_file, write_file},
};

pub fn set_governor() {
    let num_cores = num_cpus::get();

    for cpu in 0..num_cores {
        let governor_file = format!(
            "/sys/devices/system/cpu/cpu{}/cpufreq/scaling_governor",
            cpu
        );
        let governor_file_path = Path::new(&governor_file);

        if let Ok(current_governor) = read_file(governor_file_path) {
            if current_governor.trim() == config::DEFAULT_GOVERNOR {
                continue;
            }
        }

        if let Err(err) = write_file(governor_file_path, config::DEFAULT_GOVERNOR) {
            eprintln!("Error: Failed to create config file: {}", err);
            process::exit(1);
        }
    }
}
