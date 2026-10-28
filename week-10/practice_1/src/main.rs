fn main() {
    let v = vec![101,250,330,400];

    let v2 = v;

    println!("{:?}",v2);
    // changed v to v2 because the ownership have been moved from v to v2
}
