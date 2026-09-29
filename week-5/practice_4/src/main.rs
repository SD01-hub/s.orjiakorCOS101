fn main() {

    let fullname = "chibudum John Umeh";
    let department = "Computer Science";
    let uni = "Pan-atlantic University";

    let mut school = "school of Science".to_string();
    school.push_str("and Technology");

    println!("My name is: {}",fullname);
    println!("The lenght my fullname is: {}",fullname.len() );
    println!("I am a student of {} Department", department);
    println!("{}", school);
    println!("{}",uni );
}
