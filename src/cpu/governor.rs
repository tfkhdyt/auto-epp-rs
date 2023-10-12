use std::{io, path::Path};

use crate::utils::fs::{read_file, write_file};

pub fn set_governor(governor: &str) -> io::Result<()> {
    let num_cores = num_cpus::get();

    for cpu in 0..num_cores {
        let governor_file = format!(
            "/sys/devices/system/cpu/cpu{}/cpufreq/scaling_governor",
            cpu
        );
        let governor_file_path = Path::new(&governor_file);

        if let Ok(current_governor) = read_file(governor_file_path) {
            if current_governor == governor {
                continue;
            }
        }

        write_file(governor_file_path, governor)?;
    }

    Ok(())
}
