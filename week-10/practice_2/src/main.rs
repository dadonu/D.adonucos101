fn main(){
    let v = vec![10,20,30];
    let v2 = v;
  let _v = display(v2);

    println!("In main {:?}",_v);
}

fn display(v:Vec<i32>){
    println!("inside display {:?}",v);
}
