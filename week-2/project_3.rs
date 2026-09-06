fn main(){
let p: f64 = 210_000.0;
let dr: f64 =  5.0;
// dr = depreciation rate;
let n: f64 = 3.0;
//Using	A = (P*(1-dr/100)).powf(n);
let a = p*(1.0-(dr/100.0)).powf(n);
println!("depreciation amount is {}", a)
}