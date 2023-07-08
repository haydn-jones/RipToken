use std::fs::File;
use std::io::{BufRead, BufReader};

// read file into vector of lines
pub fn read_file(filename: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let file = File::open(filename).expect("file not found");
    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
        lines.push(line.expect("Could not parse line"));
    }
    lines
}

// This assumes the selfie is valid, please don't give poor little Rusty invalid selfies
// he's just a little guy
pub fn tokenize_selfie(selfie: &str) -> Vec<String> {
    let mut tokens = Vec::new();

    let mut token_start = 0;
    for (i, c) in selfie.char_indices() {
        match c {
            ']' => {
                tokens.push(selfie[token_start..i+1].to_string());
                token_start = i+1;
            },
            _ => continue,
        }
    }

    tokens
}