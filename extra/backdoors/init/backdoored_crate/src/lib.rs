pub fn do_something() {
    println!("do something...");
}

startup::on_startup! {
    println!("Warning! You just ran a malicious package. Please read https://kerkour.com/rust-crate-backdoor for more information.");
}
