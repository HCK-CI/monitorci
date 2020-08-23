extern crate sys_info;
extern crate sysinfo;

use sysinfo::{DiskExt, ProcessExt, SystemExt};

pub fn get_system_info() -> String {
    format!("Host name: {}", sys_info::hostname().unwrap())
}

fn print_process_info_by_name(title: &str, pname: &str) -> String {
    let mut result = String::from(title);
    let mut system = sysinfo::System::new_all();

    system.refresh_all();
    for (pid, proccess) in system.get_processes() {
        if proccess.name().contains(pname) {
            result.push_str(&format!("{:5}: {:?}\n",
                pid, proccess.cmd()));
        }
    }

    result
}

pub fn get_running_qemu_info() -> String {
    print_process_info_by_name("Running QEMU:\n", "qemu")
}

pub fn get_running_setup_info() -> String {
    print_process_info_by_name("Running HCK-CI setups:\n", "auto_hck")
}

pub fn get_storage_info() -> String {
    let mut system = sysinfo::System::new_all();
    let mut result = String::from("Storage information:\n");

    system.refresh_all();
    for disk in system.get_disks() {
        result.push_str(&format!("Name: {:?}, Mount point: {:?}, Available/Total {}G/{}G\n",
            disk.get_name(), disk.get_mount_point(),
            disk.get_available_space() / 1000000000, disk.get_total_space() / 1000000000));
    }

    result
}
