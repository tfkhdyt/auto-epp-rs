mod battery;
mod config;
mod cpu;
mod user;
mod utils;

use cpu::{driver, epp, governor};
use std::{thread, time::Duration};

use crate::utils::fmt;

fn main() {
    if !user::is_user_root() {
        fmt::fatalln("auto-epp-rs must be run with root privileges", None);
    }

    match driver::get_scaling_driver() {
        Ok(scaling_driver) => {
            if scaling_driver != "amd-pstate-epp" {
                fmt::fatalln("the system is not running amd-pstate-epp", None);
            }
        }
        Err(err) => fmt::fatalln("failed to get scaling driver", Some(&err)),
    };

    let epp_state = match config::get_epp_state() {
        Ok(v) => v,
        Err(err) => fmt::fatalln("failed to get epp state", Some(&err)),
    };

    loop {
        if let Err(err) = governor::set_governor(config::DEFAULT_GOVERNOR) {
            fmt::fatalln("failed to set cpu governor", Some(&err));
        }

        match battery::is_charging() {
            Ok(is_charging) => {
                if is_charging {
                    if let Err(err) = epp::set_epp(&epp_state.ac) {
                        fmt::fatalln("failed to set epp state", Some(&err));
                    }
                } else if let Err(err) = epp::set_epp(&epp_state.bat) {
                    fmt::fatalln("failed to set epp state state", Some(&err));
                }
            }
            Err(err) => fmt::fatalln("failed to get charging state", Some(&err)),
        }

        thread::sleep(Duration::from_secs(3));
    }
}
