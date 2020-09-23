use std::{env, fs::File, io::Read};

use lilc3::LC3;

fn main() {
    let file = env::args().nth(1).expect("Filename required");
    let file = match File::open(&file) {
        Ok(file) => file,
        Err(e) => panic!("Failed to open file: {}\n{}", &file, e),
    };

    let bytes: Vec<u8> = file.bytes().map(Result::unwrap).collect();
    let mut machine = LC3::new(&bytes);
    machine.run();
}
