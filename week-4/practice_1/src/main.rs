use std::io;

fn main() {
    println!("\n Student information management!");

    println!("your name please");
    let mut name = String::new();
    io::stdin()
    .read_line(&mut name)
    .expect("destiny");
println!("your name is: {}", name);

println!("\n enter your age");
let mut age = String:: new ();
io::stdin().read_line(&mut age).expect("destiny");
let age:i32 = age.trim().parse().expect("input not an integer");
println!("your age is: {}", age);
}
