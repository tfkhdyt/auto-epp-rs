use std::{path::Path, process};

use crate::utils::fs::write_file;

pub fn set_epp(epp_value: &str) {
    let cpu_count = num_cpus::get();

    for cpu in 0..cpu_count {
        let epp_file_path = format!(
            "/sys/devices/system/cpu/cpu{}/cpufreq/energy_performance_preference",
            cpu
        );

        if let Err(err) = write_file(Path::new(&epp_file_path), epp_value) {
            eprintln!(
                "Error: Failed to set energy performance preference: {}",
                err
            );
            process::exit(1);
        }
    }
}
