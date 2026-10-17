
use std::io;

fn main() {
    let mut candidates: Vec<(String, u32)> = Vec::new();

    println!("How many candidates do you want to enter?");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let number_of_candidates: usize = input.trim().parse().unwrap();

    // Collect each candidate's information
    for i in 0..number_of_candidates {
        println!("\nEnter candidate {} name:", i + 1);

        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        let name = name.trim().to_string();

        println!("{} enter your years of experience:", name);

        let mut input_years = String::new();
        io::stdin().read_line(&mut input_years).unwrap();

        let years: u32 = input_years.trim().parse().unwrap();

        candidates.push((name, years));
    }

    // Find the candidate with the highest experience
    if let Some(first) = candidates.first() {
        let mut highest = first;

        for candidate in &candidates {
            if candidate.1 > highest.1 {
                highest = candidate;
            }
        }

        println!("\n===== RESULT =====");
        println!("Winner: {}", highest.0);
        println!("Years of experience: {}", highest.1);
    } else {
        println!("No candidates were entered.");
    }
}
