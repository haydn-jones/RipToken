pub mod utils;

use crate::utils::{get_init_vocab, read_file, encode_selfie, decode_selfie, optimal_encode};

use rayon::prelude::*;

fn main() {
    let selfies = read_file("./data/train_selfies.txt");
    println!("Number of lines: {}", selfies.len());

    let mut tokens = get_init_vocab(&selfies);
    // print vocab
    for (k, v) in &tokens {
        println!("{}: {}", k, v);
    }
    tokens.insert("[Branch1][C][C][Branch1]".to_string(), tokens.len());
    tokens.insert("[C][C][Branch1][C][C][Branch1]".to_string(), tokens.len());
    tokens.insert("[Branch1][C][C]".to_string(), tokens.len());
    tokens.insert("[C][C]".to_string(), tokens.len());

    println!("Number of unique tokens: {}", tokens.len());

    let encoded = encode_selfie(&selfies[0], &tokens);
    println!(
        "Encoded: {:?} -> {}",
        encoded,
        decode_selfie(&encoded, &tokens)
    );

    let opt_encode = optimal_encode(&selfies[0], &tokens).unwrap();
    println!(
        "Optimal encoded: {:?} -> {}",
        opt_encode,
        decode_selfie(&opt_encode, &tokens)
    );

    // time how long it takes to encode all selfies
    let start = std::time::Instant::now();
    for selfie in &selfies {
        let _ = encode_selfie(selfie, &tokens);
    }
    println!("Time to encode all selfies: {:?}", start.elapsed());

    // time how long it takes to optimally encode all selfies
    let start = std::time::Instant::now();
    let _encs: Vec<Vec<usize>> = selfies.par_iter().map(|selfie| optimal_encode(selfie, &tokens).unwrap()).collect();
    println!(
        "Time to optimally encode all selfies: {:?}",
        start.elapsed()
    );
}
