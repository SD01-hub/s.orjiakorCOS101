use std::io;
fn aps_checker(profession:&str, years:u32) -> Option<(&'static str, &'static str)> {
    let table = vec![
        (0,  2,  "APS 1-2",   "Intern",               "None",                "Paralegal",            "Placement"),
        (3,  4,  "APS 3-5",   "Administrator",        "Research Assistant",  "Junior Associate",     "Classroom Teacher"),
        (5,  8,  "APS 5-8",   "Senior Administrator", "PhD Candidate",       "Associate",            "Senior Teacher"),
        (9,  10, "EL1 8-10",  "Office Manager",       "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"),
        (11, 15, "EL2 10-13", "Director",             "Senior Lecturer",     "Senior Associate 3-4", "Deputy Principal"),
        (16, 30, "SES",       "CEO",                  "Dean",                "Partner",              "Principal"),

    ];
    for row in table {
        if years >= row.0 && years <= row.1 {
           let title = match profession {
             "office admin" => row.3,
                "academic"     => row.4,
                "lawyer"       => row.5,
                "teacher"      => row.6,
                _              => return None,    
            };
            return Some((title, row.2));
        }
    } None
}
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

     println!("Please enter your Profession :");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let profession = input1.trim().to_lowercase();

     println!("Enter years of experence :");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let years:u32 = input2.trim().parse().expect("NOT a valid number");

     println!("\n---------- RESULT ----------");
    println!("Profession: {}", profession);
    println!("Experience: {} year(s)", years);

     match aps_checker(&profession, years) {
        Some((title, level)) => {
            println!("APS level:  {}", level);
            println!("Job title:  {}", title);
        }
        None => {
            println!("No matching level or profession found.");
        }
    }

    println!("----------------------------");
}