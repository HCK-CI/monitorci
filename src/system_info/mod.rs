extern crate sys_info;

pub fn get_system_info() -> String {
    format!("Host name: {}", sys_info::hostname().unwrap())
}

pub fn get_running_qemu_info() -> String {
    format!("Running QEMU:\n")
}

pub fn get_running_setup_info() -> String {
    format!("Running HCK-CI setups:\n")
}

pub fn get_storage_info() -> String {
    format!("Storage inforamtions:\n")
}
