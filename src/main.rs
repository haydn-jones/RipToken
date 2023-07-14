pub mod counter;
pub mod encoding;
pub mod utils;
pub mod vocab;

use num_format::{Locale, ToFormattedString};

use crate::{
    counter::Counter,
    encoding::{optimal_encode, par_encode_dataset, split_selfie},
    utils::{count_token_occurence, read_file},
    vocab::Vocab,
};

fn encode_and_count(selfies: &[String], vocab: &Vocab) -> (Counter, Vec<Vec<usize>>) {
    let encoded_selfies = par_encode_dataset(selfies, vocab);
    let counts = count_token_occurence(&encoded_selfies, vocab);

    (counts, encoded_selfies)
}

fn main() {
    let selfies = read_file("./data/train_selfies.txt");
    // selfies.truncate(10000);
    println!("Number of lines: {}", selfies.len().to_formatted_string(&Locale::en));

    let mut vocab = Vocab::new(&selfies);

    // get 2-grams
    let ngrams = utils::generate_ngrams(&selfies, 2);

    // insert
    for ngram in ngrams.iter() {
        let ngram_tokens = split_selfie(&ngram);
        let ngram_idxs = ngram_tokens
            .iter()
            .map(|token| vocab.get_base(token).unwrap())
            .collect::<Vec<usize>>();

        vocab.insert_ngram(&ngram_idxs);
    }

    // Encode selfie 48
    let to_enc = &selfies[48];
    let encoded = optimal_encode(&to_enc, &vocab);
    println!("Encoded: {:?}", encoded);

    let subsamp = vocab.spawn_child_vocab(128);
    let encoded = optimal_encode(&to_enc, &subsamp);
    println!("Encoded: {:?}", encoded);
}
