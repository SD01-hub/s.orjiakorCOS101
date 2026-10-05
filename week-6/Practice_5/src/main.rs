fn main() {
    let num:i32 = 5;
    mutate_new_to_zero(num);
    println!("The value of no is:{}",num);
}
fn mutate_new_to_zero(mut param_num: i32) {
    param_num = param_num*0;
    println!("param_num value is :{}",param_num );
}
