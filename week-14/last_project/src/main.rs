use std::fs::File;
use std::io::{self, Read};

fn read_file(filename: &str) {
    let mut file = File::open(filename).expect("Could not open file");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Could not read file");

    println!("{}", contents);
}

fn administrator() {
    println!("--- ADMINISTRATOR ---");
    read_file("globacom_db.sql");
}

fn project_manager() {
     println!("--- PROJECT MANAGER ---");
    read_file("project_tb.sql");
}

fn employee() {
     println!("--- EMPLOYEE ---");
    read_file("staff_tb.sql");
}

fn customer() {
    println!("--- CUSTOMER ---");
    read_file("customer_tb.sql");
}

fn vendor() {
    println!("--- VENDOR ---");
    read_file("dataplan_tb.sql");
}

fn main() {

println!("WELCOME TO GLOBACOM DATABASE");
println!("Select your role:");
println!("1. Administrator");
println!("2. Project Manager");
println!("3. Employee");
println!("4. Customer");
println!("5. Vendor");

let mut choice = String::new();

io::stdin().read_line(&mut choice).expect("Failed to read input");

match choice.trim() {
    "1" => administrator(),
    "2" => project_manager(),
    "3" => employee(),
    "4" => customer(),
    "5" => vendor(),
    _ => println!("Invalid choice!"),
}
}