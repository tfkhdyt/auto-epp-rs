use std::{io, path::Path};

use crate::utils::fs::read_file;

pub fn get_scaling_driver() -> io::Result<String> {
    let scaling_driver_path = Path::new("/sys/devices/system/cpu/cpu0/cpufreq/scaling_driver");
    let scaling_driver = read_file(scaling_driver_path)?;
    Ok(scaling_driver)
}
