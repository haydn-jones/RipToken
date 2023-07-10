pub mod encoding;
pub mod utils;
pub mod vocab;
pub mod test_utils;

use num_format::{Locale, ToFormattedString};

use crate::{
    encoding::{par_encode_dataset, split_selfie},
    utils::{count_token_occurence, generate_ngrams, read_file},
    vocab::Vocab
};

fn main() {
    let selfies = read_file("./data/train_selfies.txt");
    println!("Number of lines: {}", selfies.len().to_formatted_string(&Locale::en));

    let mut vocab = Vocab::from_data(&selfies);

    let min_ng = 2;
    let max_ng = 8;
    println!("#######################################");
    println!("Generating [{}..{}]-grams...", min_ng, max_ng);
    let mut ngrams = Vec::new();
    for i in min_ng..=max_ng {
        ngrams.extend(generate_ngrams(&selfies, i));
    }

    for ngram in ngrams.iter() {
        vocab.insert(ngram);
    }

    println!("Vocab size: {}", vocab.len().to_formatted_string(&Locale::en));
    println!("Added: {}", ngrams.len().to_formatted_string(&Locale::en));

    // encode selfies
    println!("#######################################");
    println!("Encoding selfies...");
    let encoded_selfies = par_encode_dataset(&selfies, &vocab);

    println!("#######################################");
    println!("Counting tokens...");

    let token_counter = count_token_occurence(&encoded_selfies, &vocab);

    let topn = 10;
    let topk = token_counter.k_most_common_ordered(topn);
    println!("Top {} tokens:", topn);
    for (token, count) in topk.iter() {
        println!(
            "{}: {}",
            vocab.get_rev(token).unwrap(),
            count.to_formatted_string(&Locale::en)
        );
    }

    // delete tokens with count < 100
    println!("#######################################");
    println!("Finding tokens to delete...");
    let tokens_to_delete = token_counter
        .iter()
        .filter_map(|(token, count)| {
            if *count < 100 && split_selfie(vocab.get_rev(token).unwrap()).len() > 1 {
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

#[cfg(test)]
mod utils_unit_tests;

