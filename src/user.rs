use std::process;

use nix::unistd::Uid;

pub fn check_root() {
    if !Uid::effective().is_root() {
        eprintln!("auto-epp-rs must be run with root privileges.");
        process::exit(1);
    }
}
