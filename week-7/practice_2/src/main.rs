use std::io;

fn checker(){

    let mut input = String::new();
    println!("\nEnter a character");
    io::stdin().read_line(&mut input).expect("wrong character");
    let ch:char = input.trim().parse().expect("wrong input");

    if ch >= '0' && ch <= '9'
    {
        println!("Charcter '{}' is a digit",ch);
    }
else {
    println!("Character '{}' is not a digit",ch);
}
}

fn main(){
    //calling function
    println!("\nWelcome! this variable checks wether a character
     variable contains a digit or not");
    checker()
}