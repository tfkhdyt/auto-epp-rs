# auto-epp-rs

[![auto-epp-rs](https://img.shields.io/aur/version/auto-epp-rs?color=1793d1&label=auto-epp-rs&lors=arch-linux&style=for-the-badge)](https://aur.archlinux.org/packages/auto-epp-rs/)

**auto-epp-rs** is a program that manages the energy performance preferences (EPP) of your AMD CPU using the AMD-Pstate driver. It adjusts the EPP settings based on whether your system is running on AC power or battery power, helping optimize power consumption and performance. This project is a rewrite of the original Python version [jothi-prasath/auto-epp](https://github.com/jothi-prasath/auto-epp) in Rust, aiming to improve memory efficiency and overall performance.

## Requirements

- AMD CPU with the AMD-Pstate-EPP driver enabled.
- Linux 6.3+
- Rust (make deps)

## How to enable amd-pstate-epp

Note: Since Linux 6.5, `amd-pstate-epp` is enabled by default, so you can skip this section.

### GRUB

This can be done by editing the `GRUB_CMDLINE_LINUX_DEFAULT` params in `/etc/default/grub`. Follow these steps:

1. Open the grub file using the following command:

```bash
sudoedit /etc/default/grub
```

2. Within the file, modify the `GRUB_CMDLINE_LINUX_DEFAULT` line to include the setting for AMD P-State EPP:

```bash
GRUB_CMDLINE_LINUX_DEFAULT="quiet splash amd_pstate=active"
```

### systemd-boot

This can be done by editing the `options` params in `/efi/loader/entries/your-entry.conf`. Follow these steps:

1. Open the grub file using the following command:

```bash
sudoedit /efi/loader/entries/your-entry.conf
```

2. Within the file, modify the `options` line to include the setting for AMD P-State EPP:

```bash
options    ... amd_pstate=active
```

## Installation

### Arch Linux

On Arch Linux, and Arch-based distributions, auto-epp-rs can be found in the AUR. Install with an AUR helper like yay:

```bash
yay -S auto-epp-rs
```

<!-- ### Manual Install

```bash
git clone https://github.com/tfkhdyt/auto-epp-rs
cd auto-epp-rs
sudo make install
``` -->

## Usage

Monitor the service status

```bash
systemctl status auto-epp-rs
```

To restart the service

```bash
sudo systemctl restart auto-epp-rs
```

To stop the service

```bash
sudo systemctl stop auto-epp-rs
```

Edit the config file

```bash
sudoedit /etc/auto-epp-rs.conf
```
