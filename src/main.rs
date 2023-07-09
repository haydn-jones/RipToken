pub mod utils;

use crate::utils::{get_init_vocab, optimal_encode, read_file, split_selfie};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn par_encode_dataset(selfies: &Vec<String>, vocab: &HashMap<String, usize>) -> Vec<Vec<usize>> {
    selfies
        .par_iter()
        .map(|selfie| optimal_encode(selfie, vocab).unwrap())
        .collect()
}

fn count_token_occurance(
    encoded_selfies: &Vec<Vec<usize>>,
    vocab: &HashMap<String, usize>,
) -> HashMap<usize, usize> {
    let mut token_counter = HashMap::from_iter(vocab.values().map(|v| (*v, 0)));
    for encoded_selfie in encoded_selfies {
        for token in encoded_selfie {
            token_counter.entry(*token).and_modify(|v| *v += 1);
        }
    }
    token_counter
}

fn generate_subsets(vector: &[String], size: usize) -> HashSet<String> {
    let mut perms = HashSet::new();
    (0..size).map(|_| vector.iter()).multi_cartesian_product().for_each(|v|{
        perms.insert(v.iter().fold(String::new(), |mut acc, s| {acc.push_str(s); acc}));
    });
    perms
}

fn main() {
    let selfies = read_file("./data/train_selfies.txt");
    println!("Number of lines: {}", selfies.len());

    let mut vocab = get_init_vocab(&selfies);

    // generate 2-grams
    let tokens: Vec<String> = vocab.keys().cloned().collect();

    println!("#######################################");
    for i in 2..=3 {
        let ngrams = generate_subsets(&tokens, i);
        println!("{}-grams: {}", i, ngrams.len());
        // print first 10 ngrams
        for ngram in ngrams {
            let len = vocab.len();
            vocab.entry(ngram).or_insert_with(|| len);
        }
    }

    println!("Vocab size: {}", vocab.len());

    // encode selfies
    println!("#######################################");
    println!("Encoding selfies...");
    let encoded_selfies = par_encode_dataset(&selfies, &vocab);

    println!("#######################################");
    println!("Counting tokens...");
    let token_counter = count_token_occurance(&encoded_selfies, &vocab);

    // reverse vocab
    let rev_vocab: HashMap<usize, String> = vocab.iter().map(|(k, v)| (*v, k.clone())).collect();

    // delete tokens with count < 100
    println!("#######################################");
    println!("Finding tokens to delete...");
    let mut tokens_to_delete = Vec::new();
    for (token, count) in token_counter {
        if count < 100 && split_selfie(&rev_vocab[&token]).len() > 1 {
            tokens_to_delete.push(token);
        }
    }

    println!("Tokens to delete: {}", tokens_to_delete.len());
    // get number of tokens in total

    let total = encoded_selfies.iter().map(|v| v.len()).sum::<usize>();
    println!("Total tokens: {}", total);

}
