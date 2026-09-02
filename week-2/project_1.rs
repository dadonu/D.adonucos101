fn main(){
let p: f64 = 520000000.0;
	let n: f64 = 5.0;
let r: f64 = 10.0;
	let a = p*(1.0+(r/100.0)).powf (n);
	/* In the place I have "powf" I didn't have an idea innitially
	but after computing error knowing my computation was right
	I found out "^" meant for binary bit*/
	println!("Amount: #{} ", a);
let ci = a-p;
	println!("Compound Interest: #{}", ci)
}