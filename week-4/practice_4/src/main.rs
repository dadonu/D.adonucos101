use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your name young man ");
io::stdin().read_line(&mut input1).expect("destiny string");

println!("Enter your age fast");
io::stdin().read_line(&mut input2).expect("destiny2 string");
let age:i16 = input2.trim().parse().expect("destiny number");    
   if age>18{
    println!("WELCOME!,{}",input1,);
}
    else{
      println!("Go and tell your mum  {}",input1);
    
   } 
}
