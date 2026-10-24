
use std::fs::File;
use std::io::Write;

fn main() {
    let pau_students = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
        ("Adams Aliyu", "ECO10110101", "Economics", 100),
        ("Shania Bolade", "CSC10328828", "Computer", 200),
        ("Adekunle Gold", "EEE11020202", "Electrical", 200),
        ("Blanca Edemoh", "MEE10202001", "Mechanical", 100),
    ];

    let mut file = File::create("pau_students.txt")
        .expect("Could not create file");

    for student in &pau_students {
        println!(
            "Name: {}, Matric: {}, Department: {}, Level: {}",
            student.0, student.1, student.2, student.3
        );

        writeln!(
            file,
            "{}, {}, {}, {}",
            student.0, student.1, student.2, student.3
        )
        .expect("Could not write student");
    }

    println!("\nPAU Student records saved successfully!");
}
