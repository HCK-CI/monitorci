extern crate sys_info;

pub fn get_system_info() -> String {
    format!("Host name: {}", sys_info::hostname().unwrap())
}
