use std::env;

fn main() {
    let file = env::args().nth(1).expect("Filename required");
    println!("{:?}", file);
}
