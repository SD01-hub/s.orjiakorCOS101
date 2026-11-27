
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("staff_tb.sql").unwrap();

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    println!("{}", contents);
}