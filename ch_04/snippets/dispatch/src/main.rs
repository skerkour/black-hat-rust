mod dynamic;
mod statik;

fn main() {
    println!("Static");
    statik::main2();

    println!("Dynamic");
    dynamic::main2();
}
