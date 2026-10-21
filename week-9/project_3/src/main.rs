use std::io::Write;
fn main() {
  
  let commissioner = vec!["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
  let ministry = vec!["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
  let zone = vec!["South West", "North East", "South South", "South West", "South East"];
let mut file = std::fs::File::create("The EFCC Merge.csv").unwrap();
file.write_all("The EFCC Merge\n".as_bytes()).unwrap();
file.write_all("Commissioner, Ministry, Zone\n".as_bytes()).unwrap();

for i in 0..5{
file.write_all(commissioner[i].as_bytes()).unwrap();
file.write_all(",".as_bytes()).unwrap();
file.write_all(ministry[i].as_bytes()).unwrap();
file.write_all(",".as_bytes()).unwrap();
file.write_all(zone[i].as_bytes()).unwrap();
file.write_all("\n".as_bytes()).unwrap();

}
println!("Merged successfully");

}
