use std::{io, path::Path};

use crate::utils::fs::{read_file, write_file};

pub fn set_epp(epp_value: &str) -> io::Result<()> {
    let cpu_count = num_cpus::get();

    for cpu in 0..cpu_count {
        let epp_file = format!(
            "/sys/devices/system/cpu/cpu{}/cpufreq/energy_performance_preference",
            cpu
        );
        let epp_file_path = Path::new(&epp_file);

        if let Ok(current_epp) = read_file(epp_file_path) {
            if current_epp == epp_value {
                continue;
            }
        }

        write_file(Path::new(&epp_file_path), epp_value)?;
    }

    Ok(())
}
