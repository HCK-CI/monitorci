#![feature(decl_macro)]
#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate tera;

use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use tera::Context;
mod system_info;

#[get("/")]
fn index() -> Template {
    let mut context = Context::new();

    context.insert("host_name", &system_info::get_system_info());
    // Where `base` is the name of the template
    Template::render("base", &context.into_json())
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
    rocket::ignite()
        .mount("/vendor", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/www/vendor")))
        .mount("/css", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/www/css")))
        .mount("/", routes![index, qemu, setups, storage])
        .attach(Template::fairing())
        .launch();
}
