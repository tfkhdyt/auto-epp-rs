mod battery;
mod config;
mod cpu;
mod user;
mod utils;

use std::{thread, time::Duration};

use cpu::{driver, epp, governor};

fn main() {
    user::check_root();
    driver::check_driver();

    let (epp_state_for_ac, epp_state_for_bat) = config::read_config();

    loop {
        governor::set_governor();

        if battery::check_charging_status() {
            epp::set_epp(&epp_state_for_ac);
        } else {
            epp::set_epp(&epp_state_for_bat);
        }

        thread::sleep(Duration::from_secs(3));
    }
}
