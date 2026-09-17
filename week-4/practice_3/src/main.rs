use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter the base");
    io::stdin().read_line(&mut input1).expect("destiny string");
    let base:f64 = input1.trim().parse().expect("destiny number");

    println!("Enter the height");
    io::stdin().read_line(&mut input2).expect("destiny2 string");
    let height:f64 = input2.trim().parse().expect("destiny2 number");

    if base < 0.0 {
        let area:f64 = base*height/2.0;
        println!("The area of triangle is {}",area);
    }
}
