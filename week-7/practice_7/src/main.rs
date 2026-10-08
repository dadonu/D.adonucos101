fn main() {
//Array with data type (explicit integer datatype);

let arr1:[i32;4] = [10,20,30,40];
println!("\n Array with data type");
println!("\narray is {:?}",arr1);
println!("\narray size is :{}",arr1.len());

//Array without data type (implicit float type)
let arr2 = [10.4,20.7,30.4,40.9,51.2,72.2];
println!("\n Array without data type");
println!("\narray is {:?}",arr2);
println!("\narray size is :{}",arr2.len());

/*Array with default values that creates and 
innitaites all it's elements with a default value of -1.*/

let arr3:[i32;8] = [-1;8];
println!("\n Array with default values");
println!("\narray is {:?}",arr3);
println!("\narray size is :{}",arr3.len());
}
