pub mod utils;

use crate::utils::{read_file, tokenize_selfie};

fn main() {
    let lines = read_file("./train_selfies.txt");
    println!("Number of lines: {}", lines.len());

    let tokens = tokenize_selfie(&lines[1]);
    println!("Tokens: {:?}", tokens);
}
