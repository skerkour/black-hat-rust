use malicious_macro::Evil;

#[derive(Evil)]
pub struct RandomStruct {}

pub fn do_something() {
    println!("do something...");
}

malicious_macro::evil!();
