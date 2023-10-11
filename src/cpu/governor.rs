use std::{path::Path, process};

use crate::utils::fs::write_file;

pub fn set_governor() {
    let num_cores = num_cpus::get();

    for cpu in 0..num_cores {
        let governor_file_path = format!(
            "/sys/devices/system/cpu/cpu{}/cpufreq/scaling_governor",
            cpu
        );

        if let Err(err) = write_file(Path::new(&governor_file_path), "powersave") {
            eprintln!("Error: Failed to create config file: {}", err);
            process::exit(1);
        }
    }
}
