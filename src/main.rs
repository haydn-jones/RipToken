pub mod counter;
pub mod encoding;
pub mod utils;
pub mod vocab;

use std::{collections::HashSet, fs::File};

use num_format::{Locale, ToFormattedString};

use crate::{
    counter::Counter,
    encoding::{par_encode_dataset, split_selfie},
    utils::{count_token_occurence, read_file},
    vocab::Vocab,
};

fn encode_and_count(selfies: &[String], vocab: &Vocab) -> (Counter, Vec<Vec<usize>>) {
    let encoded_selfies = par_encode_dataset(selfies, vocab);
    let counts = count_token_occurence(&encoded_selfies, vocab);

    (counts, encoded_selfies)
}

fn main() {
    let mut selfies = read_file("./data/train_selfies.txt");
    selfies.truncate(10000);
    println!("Number of lines: {}", selfies.len().to_formatted_string(&Locale::en));

    let mut tokens: HashSet<String> = HashSet::new();

    selfies.iter().for_each(|selfie| {
        let selfie_tokens = split_selfie(selfie);
        // convert to string
        let mut selfie_tokens = selfie_tokens
            .iter()
            .map(|token| token.to_string())
            .collect::<Vec<String>>();
        tokens.extend(selfie_tokens.drain(..));
    });

    let mut vocab = Vocab::new(tokens.into_iter().collect::<Vec<String>>());

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

    println!("Vocab:\n{}", vocab);

    println!("Selfie: {}", &selfies[0]);
    println!("Encoded: {:?}", encoding::optimal_encode(&selfies[0], &vocab));
    println!("Encoded decoded: {}", vocab.decode(&encoding::optimal_encode(&selfies[0], &vocab).unwrap()));
}
