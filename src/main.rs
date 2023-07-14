pub mod counter;
pub mod encoding;
pub mod types;
pub mod utils;
pub mod vocab;

use num_format::{Locale, ToFormattedString};
use rayon::prelude::*;

use crate::{utils::read_file, vocab::Vocab};

fn main() {
    let selfies = read_file("./data/train_selfies.txt");
    println!("Number of lines: {}", selfies.len().to_formatted_string(&Locale::en));

    let mut vocab = Vocab::new(&selfies);
    let enc_selfies = selfies
        .par_iter()
        .map(|selfie| vocab.base_encode(selfie))
        .collect::<Vec<Vec<u32>>>();

    // get 2-grams
    let ngrams = utils::generate_ngrams(&enc_selfies, 2);

    // insert
    for ngram in ngrams {
        vocab.insert_ngram(&ngram);
    }

    let start = std::time::Instant::now();
    let mut sink: Vec<Vec<u32>> = Vec::new();
    enc_selfies
        .par_iter()
        .map(|selfie| encoding::optimal_encode(selfie, &vocab))
        .collect_into_vec(&mut sink);
    println!("Encoding took: {:?}", start.elapsed());

    //for (i, selfie) in selfies.iter().enumerate() {
    //    let enc = vocab.base_encode(selfie);
    //    let enc = encoding::optimal_encode(&enc, &vocab);
    //    let dec = vocab.decode(&enc);
    //    if dec != *selfie {
    //        println!("Decoding failed: {}", i);
    //        println!("Original: {}", selfie);
    //        println!("Decoded: {}", dec);
    //        break;
    //    }
    //}
}
