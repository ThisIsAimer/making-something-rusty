use std::fs;

fn main() {
    let text = fs::read_to_string("logs.txt");
    println!("{:#?}",text);
    
}
