struct Laptop {
    brand: String,
    price: i32,
}

impl Laptop {
    fn display_cost(&self, quantity: i32) -> i32 {
        self.price * quantity
    }
}

fn main() {
    let hp = Laptop {
        brand: String::from("HP"),
        price: 650_000,
    };

    let ibm = Laptop {
        brand: String::from("IBM"),
        price: 755_000,
    };

    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        price: 550_000,
    };

    let dell = Laptop {
        brand: String::from("Dell"),
        price: 850_000,
    };

    let quantity = 3;

    let hp_total = hp.display_cost(quantity);
    let ibm_total = ibm.display_cost(quantity);
    let toshiba_total = toshiba.display_cost(quantity);
    let dell_total = dell.display_cost(quantity);

    let total_cost = hp_total + ibm_total + toshiba_total + dell_total;

    println!("{} Laptop cost is {}", hp.brand, hp_total);
    println!("{} Laptop cost is {}", ibm.brand, ibm_total);
    println!("{} Laptop cost is {}", dell.brand, dell_total);
    println!("{} Laptop cost is {}", toshiba.brand, toshiba_total);
    println!("-----------------------------------------------------------");
    println!("The total cost of purchasing the Laptops is {}",total_cost );
}