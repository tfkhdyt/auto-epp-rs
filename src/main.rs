extern crate nix;

use nix::unistd::Uid;
use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::Path,
    process, thread,
    time::Duration,
};

const CONFIG_PATH: &str = "/etc/auto-epp-rs.conf";
const DEFAULT_CONFIG: &str = "# see available epp state by running: cat /sys/devices/system/cpu/cpu0/cpufreq/energy_performance_available_preferences
[Settings]
epp_state_for_AC=performance
epp_state_for_BAT=power
";

fn check_root() {
    if !Uid::effective().is_root() {
        eprintln!("auto-epp-rs must be run with root privileges.");
        process::exit(1);
    }
}

fn check_driver() {
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

fn read_config() -> (String, String) {
    if let Err(_) = fs::metadata(&CONFIG_PATH) {
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
            epp_state_for_ac = line.split("=").collect::<Vec<&str>>()[1];
        } else if line.starts_with("epp_state_for_BAT") {
            epp_state_for_bat = line.split("=").collect::<Vec<&str>>()[1];
        }
    });

    (epp_state_for_ac.to_owned(), epp_state_for_bat.to_owned())
}

fn set_governor() {
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

fn check_charging_status() -> bool {
    let power_supply_path = Path::new("/sys/class/power_supply/");
    let entries = fs::read_dir(power_supply_path).unwrap_or_else(|err| {
        eprintln!("Error: Failed to read power supply directory: {}", err);
        process::exit(1);
    });

    for file in entries.into_iter() {
        let entry = file.unwrap_or_else(|err| {
            eprintln!(
                "Error: Failed to read power supply directory entry: {}",
                err
            );
            process::exit(1);
        });
        let path = entry.path();

        if path.is_dir() || path.is_symlink() {
            let Ok(power_type) = read_file(&path.join("type")) else {
                continue;
            };

            if power_type.trim() == "Mains" {
                let Ok(online_data) = read_file(&path.join("online")) else {
                    continue;
                };
                let Ok(online_status) = online_data.parse::<u8>() else {
                    continue;
                };
                if online_status == 1 {
                    return true;
                }
            } else if power_type.trim() == "Battery" {
                let Ok(status_data) = read_file(&path.join("status")) else {
                    continue;
                };
                if status_data.trim() == "Discharging" {
                    return false;
                }
            }
        }
    }

    true
}

fn read_file(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut file_str = String::new();
    file.read_to_string(&mut file_str)?;
    Ok(file_str)
}

fn write_file(path: &Path, content: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn set_epp(epp_value: &str) {
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

fn main() {
    check_root();
    check_driver();

    let (epp_state_for_ac, epp_state_for_bat) = read_config();

    loop {
        set_governor();

        if check_charging_status() {
            set_epp(&epp_state_for_ac);
        } else {
            set_epp(&epp_state_for_bat);
        }

        thread::sleep(Duration::from_secs(3));
    }
}
