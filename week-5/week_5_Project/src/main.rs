use std::io;

fn main() {
    println!("===============================================");
    println!("             RESTAURANT MENU                ");
    println!("===============================================");
    println!("CODE | ITEM                        | PRICE (N)");
    println!("-----------------------------------------------");
    println!(" P   | Poundo Yam / Edinkaiko soup | 3,200  ");
    println!(" F   | Fried Rice & Chiken         | 3,000 ");
    println!(" A   | Amala & Ewedu Soup          | 2,500  ");
    println!(" E   | Eba & Egusi Soup            | 2,000  ");
    println!(" W   | White Rice & Stew           | 2,500  ");
    println!("===============================================");

    let mut code_input = String::new();
    println!("Enter item code (P, F, A, E, W):");
    io::stdin().read_line(&mut code_input).expect("Failed to read");

    let code = code_input.trim().to_uppercase();

    let unit_price:f64 = match code.as_str(){
        "P" => 3_200.00,
        "F" => 3_000.00, 
        "A" => 2_500.00, 
        "E" => 2_000.00, 
        "W" => 2_500.00,
          _ => {
            println!("Invalid item code");
            return;
           }  
    };
let mut qty_input = String::new();
    println!("Enter the quantity:");
    io::stdin().read_line(&mut qty_input).expect("Failed to read line");

    let quantity: f64 = qty_input.trim().parse().expect("Not a valid number for quantity");

     let total_cost = unit_price * quantity;

    let discount = if total_cost > 10_000.0 {
        total_cost * 0.05
    } else {
        0.0
    };

    let final_amount = total_cost - discount;

    println!("The Final Amount is {}", final_amount);




}
