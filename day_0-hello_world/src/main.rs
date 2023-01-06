// Enter your code here 
use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Fail");
    
    println!("Hello, World.");
    println!("{input}");
}