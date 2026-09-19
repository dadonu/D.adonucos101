use std::io;

fn main() {

  println!("\nINCENTIVE CALCULATOR");

    let _old_adult:u64 = 1_560_000;
    let _middle_adult:u64 = 1_480_000;
let _young_adult:u64 = 1_300_000;
let _younger_adult:u64 = 100_000;
  
  println!("\nEnter your name");
  let mut name = String::new();
  io::stdin().read_line(&mut name).expect("wrong input");

println!("\nAre you experienced? ( Enter 1 for yes/0 for no):");
   let mut answer = String::new();
   io::stdin().read_line(&mut answer).expect("wrong answer");
let experienced:u32 = answer.trim().parse().expect("wrong age input");

if experienced == 1{

println!("\nEnter your age");
  let mut age = String::new();
  io::stdin().read_line(&mut age).expect("wrong age string");
  let  _age:u32 = age.trim().parse().expect("wrong age number");

  if age >= 40.to_string(){ 
    println!(" Annual incentive is {},{}", _old_adult,name);
}

else if age >= 30.to_string() && age <=39.to_string(){
  println!("Annual incentive is {},{}", _middle_adult,name);
} 
  else if age <28.to_string(){
    println!("Annual incentive is {},{}", _young_adult,name);
  }  
}

else if experienced == 0{
  println!(" Annual incentive is {},{}", _younger_adult,name);
}
  else {
  println!("Invalid input, try inputing 1 or 0 in your experienced button thank! {}", name);

}


}


