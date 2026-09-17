use std::io;

fn main() 
{
    let mut input = String::new();

    println!("Enter your height (in CM):");
    io::stdin().read_line(&mut input).expect("destiny input");
    let height:f64 = input.trim().parse().expect("wrong value");

    if  height >= 150.0 && height <= 170.0
    {
        println!("you're an average height person");
    }
    else if  height > 170.0 && height <= 195.0
    {
        println!(" you're tall");
    }
    else if  height < 150.0 && height >100.0
    {
        println!(" you're dwarf");
    }
    else {
        println!("Abnormal height!!!");
    }

}
