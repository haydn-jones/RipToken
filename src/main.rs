pub mod counter;
pub mod encoding;
pub mod utils;
pub mod vocab;

use std::collections::HashSet;

use num_format::{Locale, ToFormattedString};

use crate::{
    counter::Counter,
    encoding::{par_encode_dataset, split_selfie},
    utils::{count_token_occurance, generate_ngrams, read_file},
    vocab::Vocab,
};

fn encode_and_count(selfies: &[String], vocab: &Vocab) -> (Counter, Vec<Vec<usize>>) {
    let encoded_selfies = par_encode_dataset(selfies, vocab);
    let counts = count_token_occurance(&encoded_selfies, vocab);

    (counts, encoded_selfies)
}

fn generate_random_vocab(base_vocab: &Vocab, target_size: usize) -> Vocab {
    let mut random_vocab = HashSet::new();
    for key in base_vocab.keys() {
        // we have to push keys of size 1
        if split_selfie(&key).len() == 1 {
            random_vocab.insert(key);
        }
    }

    while random_vocab.len() < target_size {
        let random_idx = rand::random::<usize>() % base_vocab.len();
        let random_key = base_vocab.get_rev(&random_idx).unwrap();
        if !random_vocab.contains(random_key) {
            random_vocab.insert(random_key.to_string());
        }
    }

    // make new vocab
    let mut new_vocab = Vocab::new();
    for key in random_vocab.iter() {
        new_vocab.insert(key);
    }

    new_vocab
}

fn delete_by_usage(counts: &Counter, min_count: usize, vocab: &Vocab) -> Vec<String> {
    let lt_counts = counts.counts_less_n(min_count);
    lt_counts
        .iter()
        .filter_map(|token| {
            if let Some(key) = vocab.get_rev(token) {
                if split_selfie(key).len() > 1 {
                    Some(key.to_string())
                } else {
                    None
                }
            } else {
                panic!("Token not found in vocab!");
            }
        })
        .collect::<Vec<_>>()
}

fn delete_by_percent(counts: &Counter, worst_percent: f32, vocab: &Vocab) -> Vec<String> {
    let num_vocab = vocab.len() as f32;
    let num_to_delete = (num_vocab * worst_percent) as usize;

    counts
        .least_common(counts.len())
        .iter()
        .filter_map(|(token, _count)| {
            if let Some(key) = vocab.get_rev(token) {
                if split_selfie(key).len() > 1 {
                    Some(key.to_string())
                } else {
                    None
                }
            } else {
                panic!("Token not found in vocab!");
            }
        })
        .take(num_to_delete)
        .collect::<Vec<_>>()
}

fn print_stats(counter: &Counter, vocab: &Vocab) {
    let topn = 10;
    let topk = counter.most_common(topn);
    let count = counter.values().iter().sum::<usize>();

    println!("Tokens to encode dataset: {}", count.to_formatted_string(&Locale::en));
    println!("Top {} tokens:", topn);
    for (token, count) in topk.iter() {
        let token = vocab.get_rev(token).unwrap();
        println!("{}: {}", token, count.to_formatted_string(&Locale::en));
    }
}

fn main() {
    let selfies = read_file("./data/train_selfies.txt");
    println!("Number of lines: {}", selfies.len().to_formatted_string(&Locale::en));

    let mut vocab = Vocab::from_data(&selfies);

    // encode dataset and print stats
    println!("#######################################");
    println!("Encoding selfies...");
    let (token_counter, _encoded_selfies) = encode_and_count(&selfies, &vocab);
    print_stats(&token_counter, &vocab);

    let min_ng = 2;
    let max_ng = 10;
    println!("#######################################");
    println!("Generating [{}..{}]-grams...", min_ng, max_ng);
    let mut ngrams = Vec::new();
    for i in min_ng..=max_ng {
        ngrams.extend(generate_ngrams(&selfies, i));
    }

    for ngram in &ngrams {
        vocab.insert(ngram);
    }

    println!("Vocab size: {}", vocab.len().to_formatted_string(&Locale::en));
    println!("Added: {}", ngrams.len().to_formatted_string(&Locale::en));

    // encode selfies
    println!("#######################################");
    println!("Encoding selfies...");
    let (token_counter, _encoded_selfies) = encode_and_count(&selfies, &vocab);
    print_stats(&token_counter, &vocab);

    // delete tokens with count < 100
    println!("#######################################");
    println!("Finding tokens to delete...");
    let tokens_to_delete = delete_by_usage(&token_counter, 100, &vocab);
    println!(
        "Tokens to delete: {}",
        tokens_to_delete.len().to_formatted_string(&Locale::en)
    );

    // delete tokens
    println!("#######################################");
    println!("Deleting tokens...");
    vocab.batch_remove(&tokens_to_delete);
    println!("Vocab size: {}", vocab.len().to_formatted_string(&Locale::en));

    // Generate random vocabs
    println!("#######################################");
    println!("Generating random vocabs...");
    for i in 0..3600 {
        println!("------------");
        println!("Iteration: {}", i);
        println!("Current vocab size: {}", vocab.len().to_formatted_string(&Locale::en));

        let new_vocab = generate_random_vocab(&vocab, 1024);
        let (token_counter, _) = encode_and_count(&selfies, &new_vocab);

        let count = token_counter.values().iter().sum::<usize>();
        println!("Tokens to encode dataset: {}", count.to_formatted_string(&Locale::en));

        // delete worst 1%
        let todelete = delete_by_percent(&token_counter, 0.01, &new_vocab);
        println!("Deleting worst 1%: {}", todelete.len().to_formatted_string(&Locale::en));
        vocab.batch_remove(&todelete);
    }
}
