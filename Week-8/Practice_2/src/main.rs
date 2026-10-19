use std::io::Read;
use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("welcome_message.txt").expect("failed");
    file.write_all(b" E get the way life just be.").expect("write failed");

    let mut file = std::fs::File::open("welcome_message.txt").unwrap();
    let mut contents = String::from("SD for life.\n");
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents );
}