#![feature(decl_macro)]
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

#[get("/qemu")]
fn qemu() -> &'static str {
    "Hello world!"
}

#[get("/setups")]
fn setups() -> &'static str {
    "Hello world!"
}

#[get("/storage")]
fn storage() -> &'static str {
    "Hello world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index, qemu, setups, storage]).launch();
}
