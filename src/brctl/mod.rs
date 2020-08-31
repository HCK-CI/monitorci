use std::process::Command;

pub fn get_bridges() -> String {
    let output = Command::new("brctl")
        .arg("show")
        .output()
        .unwrap();

    String::from_utf8_lossy(&output.stdout).into_owned()
}