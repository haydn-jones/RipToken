pub mod counter;
pub mod encoding;
pub mod utils;
pub mod vocab;

use num_format::{Locale, ToFormattedString};
use rayon::prelude::*;

use crate::{utils::read_file, vocab::Vocab};

fn main() {
    let selfies = read_file("./data/train_selfies.txt");
    println!("Number of lines: {}", selfies.len().to_formatted_string(&Locale::en));

    let mut vocab = Vocab::new(&selfies);
    let selfies = selfies
        .par_iter()
        .map(|selfie| vocab.base_encode(selfie))
        .collect::<Vec<Vec<u32>>>();

    // get 2-grams
    let ngrams = utils::generate_ngrams(&selfies, 2);

    // insert
    for ngram in ngrams {
        vocab.insert_ngram(&ngram);
    }

    let start = std::time::Instant::now();
    let mut sink: Vec<Vec<u32>> = Vec::new();
    selfies
        .par_iter()
        .map(|selfie| encoding::optimal_encode(selfie, &vocab))
        .collect_into_vec(&mut sink);
    println!("Encoding took: {:?}", start.elapsed());
}
