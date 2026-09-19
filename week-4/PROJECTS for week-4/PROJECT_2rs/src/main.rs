use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");

     println!("Enter your age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:i32 = input2.trim().parse().expect("Not a valid number");

     println!("Enter your years of experience: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let exper:u32 = input3.trim().parse().expect("Not a valid number");

    if exper > 0 {
        println!("EXPERIENCED {} ",input3 );
    } else {
        println!("NOT EXPERIENCED");
    }
    if exper > 0 && age > 40 {
        println!("annual incentive = N1,560,000");
    } else if exper > 0 && age > 30 && age <= 39 {
        println!("annual incentive = N1,480,000");
    } else if exper > 0 && age < 28 { 
        println!("annual incentive = N1,300,000");
    } else if exper == 0 {
        println!("annual incentive = N100,000");
    }
}
