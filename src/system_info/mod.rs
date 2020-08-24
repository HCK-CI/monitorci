extern crate sys_info;
extern crate sysinfo;

use sysinfo::{DiskExt, ProcessExt, SystemExt};

pub fn get_system_info() -> String {
    sys_info::hostname().unwrap()
}

fn print_process_info_by_name(pname: &str) -> String {
    let mut result = String::new();
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

pub fn get_number_of_cpus() -> String {
    sys_info::cpu_num().unwrap().to_string()
}

pub fn get_cpu_speed() -> String {
    (sys_info::cpu_speed().unwrap() / 1000).to_string()
}

pub fn get_total_memory_info() -> f64 {
    sys_info::mem_info().unwrap().total as f64
}

pub fn get_free_memory_info() -> f64 {
    sys_info::mem_info().unwrap().free as f64
}

pub fn get_os_info() -> String {
    sys_info::os_release().unwrap().to_string()
}

pub fn get_running_qemu_info() -> String {
    print_process_info_by_name("qemu")
}

pub fn get_running_setup_info() -> String {
    print_process_info_by_name("ruby")
}

pub fn get_storage_info() -> String {
    let mut system = sysinfo::System::new_all();
    let mut result = String::new();

    system.refresh_all();
    for disk in system.get_disks() {
        result.push_str(&format!("Name: {:?}, Mount point: {:?}, Available/Total {}G/{}G\n",
            disk.get_name(), disk.get_mount_point(),
            disk.get_available_space() / 1000000000, disk.get_total_space() / 1000000000));
    }

    result
}
