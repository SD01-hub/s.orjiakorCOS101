
use std::fs::File;
use std::io::Write;

fn main() {
    let efcc_records = [
("Aigbogun Alamba Daudu", "Internal Affairs", "South West"),
("Murtala Afeez Bendu", "Justice", "North East"),
("Okorocha Calistus Ogbona", "Defense", "South South"),
("Adewale Jimoh Akanbi", "Power & Steel", "South West"),
("Osazuwa Faith Etiye", "Petroleum", "South East"),
    ];

    let mut file = File::create("efcc_records.txt")
        .expect("Could not create file");

    for people in &efcc_records {
        println!("Commissioner : {},  Ministry : {},  Zone : {}",people.0, people.1, people.2 );

        writeln!(
            file,
            "{}, {}, {}", people.0, people.1, people.2).expect("Could not Write");
    }
        println!("The EFCC Merge saved successfully");
    }
