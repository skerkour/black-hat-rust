use std::path::Path;

fn main() {
    let home = std::env::var("HOME").unwrap();
    let home = Path::new(&home);
    let warning_file = home.join("WARNING_BUILD");

    let message = "Warning! You just ran a malicious package. Please read https://kerkour.com/rust-crate-backdoor for more information.";
    let _ = std::fs::write(warning_file, message);
}
