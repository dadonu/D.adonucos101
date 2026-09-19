use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!(" Enter the value of a");
    io::stdin().read_line(&mut a).expect("wrong input1");
    let a:f64 = a.trim().parse().expect("wrong b value");
if a == 0.0{
println!(" Error: a cannot be zero in quadratic equation.");
        return;
}
    println!("Enter the value of b");
    io::stdin().read_line(&mut b).expect("wrong input2");
    let b:f64 = b.trim().parse().expect("wrong b value");

    println!("Enter the value of c");
    io::stdin().read_line(&mut c).expect("wrong input3");
    let c:f64 = c.trim().parse().expect("wrong c value");


    let  d = b*b-4.0*a*c;

    println!("values input, a = {},b = {}, c = {}, d = {}", a,b,c,d);

    if d>0.0{
        let root1 = (-b+d.sqrt())/(2.0*a);
        let root2 =(-b-d.sqrt())/(2.0*a);

        println!("There are two roots");
        println!("root1 = {:.1}",root1);
        println!("root2 = {:.1}",root2);
    }
    else if d == 0.0 {
        let root = -b/(2.0*a);
        println!("root is one:root = {}", root);
    }
    else {
        println!("There is no real roots");
    }
}
