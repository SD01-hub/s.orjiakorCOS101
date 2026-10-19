
use std::fs::File;
use std::io::Write;

fn main() {
    let lager = vec![
        "33 Export",
        "Desperados",
        "Goldberg",
        "Gulder",
        "Heineken",
        "Star",
    ];

    let stout = vec![
        "Legend",
        "Turbo King",
        "Williams",
    ];

    let non_alcoholic = vec![
        "Maltina",
        "Amstel Malta",
        "Malta Gold",
        "Fayrouz",
    ];

    let mut file = File::create("breweries.txt")
        .expect("create failed");

    file.write_all(b"LAGER\n")
        .expect("Write failed");

    for things in &lager {
        writeln!(file, "{}", things).expect("Write failed");
    }

    file.write_all(b"\nSTOUT\n")
        .expect("Write failed");

    for things in &stout {
        writeln!(file, "{}", things).expect("Write failed");
    }

    file.write_all(b"\nNON-ALCOHOLIC\n")
        .expect("Write failed");

    for things in &non_alcoholic {
        writeln!(file, "{}", things).expect("Write failed");
    }

    println!("successful!");
}
