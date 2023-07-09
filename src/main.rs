pub mod encoding;
pub mod utils;

use std::collections::HashMap;

use counter::Counter;
use num_format::{Locale, ToFormattedString};
use rayon::prelude::*;

use crate::{
    encoding::{optimal_encode, split_selfie},
    utils::{generate_subsets, get_init_vocab, read_file},
};

fn par_encode_dataset(selfies: &Vec<String>, vocab: &HashMap<&str, usize>) -> Vec<Vec<usize>> {
    selfies
        .par_iter()
        .map(|selfie| optimal_encode(selfie, vocab).unwrap())
        .collect()
}

fn count_token_occurance(encoded_selfies: &Vec<Vec<usize>>, vocab: &HashMap<&str, usize>) -> Counter<usize> {
    let mut token_counter = Counter::new();
    for v in vocab.values() {
        token_counter.insert(*v, 0);
    }
    for encoded_selfie in encoded_selfies {
        token_counter.update(encoded_selfie.to_vec());
    }

    token_counter
}

fn main() {
    let selfies = read_file("./data/train_selfies.txt");
    println!("Number of lines: {}", selfies.len().to_formatted_string(&Locale::en));

    let mut vocab = get_init_vocab(&selfies);

    let min_ng = 2;
    let max_ng = 8;
    println!("#######################################");
    println!("Generating [{}..{}]-grams...", min_ng, max_ng);
    let mut ngrams = Vec::new();
    for i in min_ng..=max_ng {
        ngrams.extend(generate_subsets(&selfies, i));
    }

    for ngram in ngrams.iter() {
        let len = vocab.len();
        if !vocab.contains_key(ngram) {
            vocab.insert(ngram, len);
        }
    }

    println!("Vocab size: {}", vocab.len().to_formatted_string(&Locale::en));
    println!("Added: {}", ngrams.len().to_formatted_string(&Locale::en));

    // encode selfies
    println!("#######################################");
    println!("Encoding selfies...");
    let encoded_selfies = par_encode_dataset(&selfies, &vocab);

    println!("#######################################");
    println!("Counting tokens...");
    // reverse vocab
    let rev_vocab: HashMap<usize, &str> = vocab.iter().map(|(k, v)| (*v, (*k).clone())).collect();

    let token_counter = count_token_occurance(&encoded_selfies, &vocab);

    let topn = 10;
    let topk = token_counter.k_most_common_ordered(topn);
    println!("Top {} tokens:", topn);
    for (token, count) in topk {
        println!("{}: {}", rev_vocab[&token], count.to_formatted_string(&Locale::en));
    }

    // delete tokens with count < 100
    println!("#######################################");
    println!("Finding tokens to delete...");
    let tokens_to_delete = token_counter
        .iter()
        .filter_map(|(token, count)| {
            if *count < 100 && split_selfie(rev_vocab[token]).len() > 1 {
                Some(*token)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

    println!(
        "Tokens to delete: {}",
        tokens_to_delete.len().to_formatted_string(&Locale::en)
    );

    let total = encoded_selfies.iter().map(|v| v.len()).sum::<usize>();
    println!("Tokens to encode dataset: {}", total.to_formatted_string(&Locale::en));
}
