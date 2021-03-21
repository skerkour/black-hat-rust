use std::thread;

fn main() {
    let mut my_vec: Vec<i64> = Vec::new();

    thread::spawn(move || { // <- notice the move keyword here
        add_to_vec(&mut my_vec);
    })
    .join()
    .expect("joining thread");

    println!("{:?}", &my_vec);
}

fn add_to_vec(vec: &mut Vec<i64>) {
    vec.push(42);
}
