use std::io;

fn read_input(prompt: &str) -> f64 {
    println!("{}",prompt );
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("not a valid string");
    input.trim().parse().expect("Not valid")
}

fn trapezium_area() {
    let height = read_input("Enter height");
    let base1 = read_input("Enter base1");
    let base2 = read_input("Enter base2");
    let area = height / 2.0 * (base1 + base2);
    println!("The area of the trapezium is {}",area );
}

fn rhombus_area() {
    let diagonal1 = read_input("Enter diagonal1");
    let diagonal2 = read_input("Enter diagonal2");
    let area = 0.5 * diagonal1 * diagonal2;
    println!("The area of the rhombus is {}",area );
}

fn parallelogram_area() {
    let base = read_input("Enter base");
    let altitude = read_input("Enter altitude");
    let area = base * altitude;
    println!("The area of the parallelogram is {}",area );
}

fn cube_surface_area() {
    let side = read_input("Enter the side length of the cube:");
    let surface_area = 6.0 * side * side;
    println!("The surface area of the cube is: {}", surface_area);
}

fn cylinder_volume() {
    let radius = read_input("Enter the radius of the cylinder:");
    let height = read_input("Enter the height of the cylinder:");
    let pi = std::f64::consts::PI;
    let volume = pi * radius * radius * height;
    println!("The volume of the cylinder is:{}", volume);
}

fn main() {
    loop {
        println!("===  The Shape Calculator ===");
        println!("1. Trapezium (Area)");
        println!("2. Rhombus (Area)");
        println!("3. Parallelogram (Area)");
        println!("4. Cube (Surface Area)");
        println!("5. Cylinder (Volume)");
        println!("6. Exit");

        let choice = read_input("choice options from (1-6):") as u32;

        match choice {
            1 => trapezium_area(),
            2 => rhombus_area(),
            3 => parallelogram_area(),
            4 => cube_surface_area(),
            5 => cylinder_volume(),
            6 => {
                println!("Exiting, byebye");
                break;
            }
            _ => println!("Invalid option")
        }
    }
}    