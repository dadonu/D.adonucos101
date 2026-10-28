struct AlabaElectronics {
    brand:String,
    price:u64,
/*Mr. Chudi note in the table you said price each so am assuming 
 that's the price of one laptop each*/
}

fn main() {
  
  let brand1 = AlabaElectronics{
    brand:String::from("HP"),
    price:650_000
  };

  let brand2 = AlabaElectronics{
    brand:String::from("IBM"),
    price:755_000
  };

  let brand3 = AlabaElectronics{
    brand:String::from("Toshiba"),
    price:550_000
  };

  let brand4 = AlabaElectronics{
    brand:String::from("Dell"),
    price:850_000
  };

 let total = brand1.cost() + brand2.cost() + brand3.cost() + brand4.cost();

  println!("The grand total of {}, {}, {}, {} when each is multiplied by 3 and added together is = N{total}"
    , brand1.brand,brand2.brand,brand3.brand,brand4.brand);
  }
impl AlabaElectronics{
fn cost(&self)->u64 {
    self.price * 3
}
}
