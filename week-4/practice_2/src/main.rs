use std::io;

fn main() {
   let mut input1 = String::new();
   let mut input2 = String::new();
   let mut input3 = String::new();

   println!("enter first edge of triangle:");
   io::stdin().read_line(&mut input1).expect("destiny string");
   let a:f32 = input1.trim().parse().expect("destiny number"); 

   println!("enter the second edge of the triangle");
   io::stdin().read_line(&mut input2).expect("destiny2 string");
   let b:f32 = input2.trim().parse().expect("destiny2 number");

   println!("enter the third edge of the triangle ");
   io::stdin().read_line(&mut input3).expect("dstiny3 string");
   let c:f32 = input3.trim().parse().expect("destimy3 number");

   let s:f32 = (a+b+c)/2.0;
   let mut area:f32 = s*(s-a)*(s-b)*(s-c);
   area = area.sqrt();

   println!("Area of a triangle: {}", area);
}
