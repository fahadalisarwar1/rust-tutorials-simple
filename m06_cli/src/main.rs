use std::env;
use std::fs::File;
use std::io::Read;
fn main() {
    for arg in std::env::args() {
        println!("{}", arg);
    }

    let filename = "hobbies.txt";
    let mut text = String::new();

    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut text);
    println!("{}", text);
}
