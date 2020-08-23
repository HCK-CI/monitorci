extern crate sys_info;
extern crate sysinfo;

use sysinfo::{DiskExt, ProcessExt, SystemExt};

pub fn get_system_info() -> String {
    format!("Host name: {}", sys_info::hostname().unwrap())
}

pub fn get_running_qemu_info() -> String {
    let mut result = String::from("Running QEMU:\n");
    let mut system = sysinfo::System::new_all();

    // First we update all information of our system struct.
    system.refresh_all();

    // Now let's print every process' id and name:
    for (pid, proccess) in system.get_processes() {
        if proccess.name().contains("qemu") {
            result.push_str(&format!("{}:{} => status: {:?}\n",
                pid, proccess.name(), proccess.status()));
        }
    }

    result
}

pub fn get_running_setup_info() -> String {
    format!("Running HCK-CI setups:\n")
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
