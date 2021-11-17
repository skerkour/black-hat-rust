use proc_macro::TokenStream;
use std::path::Path;

fn write_warning(file: &str) {
    let home = std::env::var("HOME").unwrap();
    let home = Path::new(&home);
    let warning_file = home.join(file);

    let message = "Warning! You just ran a malicious package. Please read https://kerkour.com/rust-crate-backdoor for more information.";
    let _ = std::fs::write(warning_file, message);
}

#[proc_macro_derive(Evil)]
pub fn evil_derive(_item: TokenStream) -> TokenStream {
    write_warning("WARNING_DERIVE");

    "".parse().unwrap()
}

#[proc_macro]
pub fn evil(_item: TokenStream) -> TokenStream {
    write_warning("WARNING_MACRO");

    "".parse().unwrap()
}
