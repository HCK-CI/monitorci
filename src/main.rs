#![feature(decl_macro)]
#[macro_use] extern crate rocket;

mod system_info;

#[get("/")]
fn index() -> String {
    system_info::get_system_info()
}

#[get("/qemu")]
fn qemu() -> String {
    system_info::get_running_qemu_info()
}

#[get("/setups")]
fn setups() -> String {
    system_info::get_running_setup_info()
}

#[get("/storage")]
fn storage() -> String {
    system_info::get_storage_info()
}

fn main() {
    rocket::ignite().mount("/", routes![index, qemu, setups, storage]).launch();
}
