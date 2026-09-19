use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter value for a");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid value");

    println!("Enter value for b");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid value");

    println!("Enter value for c");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid value");

    let d:f32 = b * b - 4.0 * a * c;
    if d > 0.0 {
       let x1:f32 = (-b + d.sqrt()) / (2.0 * a);
       let x2:f32 = (-b - d.sqrt()) / (2.0 * a);
       println!("Two roots: {}, {}",x1, x2 );
   }
   else if d == 0.0{
       let x:f32 = -b / (2.0 * a);
       println!("One root: {}",x );
   }


}
