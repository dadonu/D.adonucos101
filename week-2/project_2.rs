fn main(){
	// let Quantity=q, while Amount = a
let _toshiba_q = 2.0;
let _mac_q = 1.0;
let _hp_q = 3.0;
let _dell_q = 3.0;
let _acer_q = 1.0;
let _toshiba_a = 450_000.0;
let _mac_a = 1_500_000.0;
let _hp_a = 750_000.0;
let _dell_a = 2_850_000.0;
let _acer_a = 250_000.0;
/* multiplying Q by A 
Represent items with variable
(toshiba = P
mac = R
hp = S
dell = T
acer = U)*/
let _t = _toshiba_q * _toshiba_a;
let _m = _mac_q * _mac_a;
let _h = _hp_q * _hp_a;
let _d = _dell_q * _dell_a;
let _a = _acer_q * _acer_a;
println!("P = {}",_t);
println!("R = {}",_m);
println!("S = {}",_h);
println!("T = {}",_d);
println!("U = {}",_a);
//Finding sum
let sum = _t + _m + _h + _d + _a;
println!("sum = {}", sum);
//Finding average = sum / 5.0
// Using avg for average. 
let avg = sum / 5.0;
println!("avg = {}", avg);
}