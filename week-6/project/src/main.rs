use std::io;

fn main() {
    println!("Destiny's Restaurant Menu😊 
        \nInput your name  (end or submit order by entering Q)👌");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("name");
    println!("Hi, {} \nWelcome 2 Destiny Restaurant
     \nWhat would you like to order?",
     name);


    let mut grand_total = 0;

loop {

println!("We have:
    \nP for Poundo Yam/Edinkaiko Soup = N3,200 
    \nF for Fried rice & Chicken = N3,000
    \nA for Amala & Ewedu Soup = N2,500
    \nE for Eba & Egusi Soup = N2,000 
    \nW for White Rice & Stew = N2,500");
println!("Enter food code (P,F,A,E,W)");
let mut code = String::new();
io::stdin().read_line(&mut code).expect("wrong code");
let code = code.trim().to_uppercase();

if code == "Q" {
    break;
}



let price:i32;

if code == "P"{
    price = 3_200;
}
else if code == "F" {
    price = 3_000;
}
else if code == "A" {
    price = 2_500;
}
else if code == "E"{
    price = 2_000;
}
else if code == "W"{
    price = 2_500;
}
else{
 println!("Invalid food code selected");
let _price: i32;
break;
}

println!("Enter Quantity");
let mut quantity = String::new();
io::stdin().read_line(&mut quantity).expect("wrong input");
let quantity:i32 = quantity.trim().parse().expect("wrong input");

 let total = price * quantity;
 println!(" Subtotal = N{}",total);
 println!("Do you want more? 😊");

  grand_total = grand_total + total;
 }

println!("Your gross total is = {}",grand_total);

 if grand_total > 10_000 {
    let discount = (grand_total * 5) / 100;
   let final_total = grand_total - discount;

println!("You were given 5% discount of N{}",final_total);
 }
  
else {
    println!("Total amount to pay = N{}",grand_total);
}
println!(" Thanks for patronizing us👍👍👍");

}
