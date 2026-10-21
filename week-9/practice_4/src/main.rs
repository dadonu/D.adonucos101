use std::fs::OpenOptions;
use std::io::Write;

fn main(){

let mut file = OpenOptions::new().append(true).open("data.txt").expect("file does not exist");

    file.write_all("\nHello Class".as_bytes()).expect("failed to read");
    file.write_all("\nThis is the appendage to the documents.".as_bytes()).expect("failed to run");

    println!("File append success");
}