#![feature(decl_macro)]
#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate tera;

use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use tera::Context;
mod system_info;
mod brctl;

#[get("/")]
fn index() -> Template {
    let mut context = Context::new();

    context.insert("host_name", &system_info::get_system_info());
    context.insert("number_of_cpus", &system_info::get_number_of_cpus());
    context.insert("os_info", &system_info::get_os_info());
    context.insert("free_memory_ratio", &(100 - system_info::get_free_memory_info() as u64 * 100 / system_info::get_total_memory_info() as u64));
    context.insert("total_memory", &(system_info::get_total_memory_info() / 1_000_000.0));
    context.insert("free_memory", &(system_info::get_free_memory_info() / 1_000_000.0));
    context.insert("storage_info", &system_info::get_storage_info());
    context.insert("setup_info", &system_info::get_process_info_by_name("ruby"));
    context.insert("qemu_info", &system_info::get_process_info_by_name("qemu"));
    context.insert("ivshmem_info", &system_info::get_process_info_by_name("ivshmem-server"));
    context.insert("bridge_info", &brctl::get_bridges());

    // Where `base` is the name of the template
    Template::render("base", &context.into_json())
}

fn main() {
    rocket::ignite()
        .mount("/vendor", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/www/vendor")))
        .mount("/css", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/www/css")))
        .mount("/js", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/www/js")))
        .mount("/", routes![index])
        .attach(Template::fairing())
        .launch();
}
