use std::io;
fn add(a:i32, b:i32){
    let sum = a+b;

    println!("Sum of A and B = {}", sum);
}
fn main(){
    let mut input1 = String::new();
    println!("Enter input for parameter A:");
    io::stdin().read_line(&mut input1).expect("wrong input1");
    let a:i32 = input1.trim().parse().expect("invalid input1");


    let mut input2 = String::new();
    println!("Enter input for prameter B:");
    io::stdin().read_line(&mut input2).expect("wrong input2");
    let b:i32 = input2.trim().parse().expect("invalid input2");

    //call add function with arguments 

    add(a,b);
}