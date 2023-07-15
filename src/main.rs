pub mod encoding;
pub mod types;
pub mod utils;
pub mod vocab;

use ::counter::Counter;
use kdam::tqdm;
use num_format::{Locale, ToFormattedString};
use rayon::prelude::*;
use types::Selfie;

use crate::{
    encoding::par_encode_dataset,
    utils::{count_token_occurence, read_file},
    vocab::Vocab,
};

fn encode_and_stat(enc_selfies: &[Selfie], vocab: &Vocab) -> Counter<u32> {
    println!("###############################################");
    let start = std::time::Instant::now();
    let sink = par_encode_dataset(enc_selfies, vocab);
    println!("Encoding took: {:?}", start.elapsed());
    println!("Encoded {} selfies", sink.len().to_formatted_string(&Locale::en));

    let counts = count_token_occurence(&sink, vocab);
    println!(
        "Tokens to encode: {}",
        sink.iter()
            .map(|selfie| selfie.len())
            .sum::<usize>()
            .to_formatted_string(&Locale::en)
    );
    println!("Top 20 tokens:");
    for (token, count) in counts.k_most_common_ordered(20) {
        println!(
            "{}: {}",
            vocab.decode(&Selfie::from([token])),
            count.to_formatted_string(&Locale::en)
        );
    }

    counts
}

fn main() {
    let selfies = read_file("./data/train_selfies.txt");
    println!("Number of lines: {}", selfies.len().to_formatted_string(&Locale::en));

    let mut vocab = Vocab::new(&selfies);
    let enc_selfies = selfies
        .par_iter()
        .map(|selfie| vocab.base_encode(selfie))
        .collect::<Vec<Selfie>>();

    for n in 2..=12 {
        println!("Generating {}-grams", n);
        let ngrams = utils::generate_ngrams(&enc_selfies, n);
        // insert
        for ngram in ngrams.iter() {
            vocab.insert_ngram(&ngram);
        }
        println!("   - {}", ngrams.len().to_formatted_string(&Locale::en));
    }
    println!("Vocab size: {}", vocab.len().to_formatted_string(&Locale::en));

    let counts = encode_and_stat(&enc_selfies, &vocab);

    // Delete anything used less than twice
    let todel = counts
        .iter()
        .filter(|(_, count)| **count < 2)
        .map(|(token, _)| *token)
        .collect::<Vec<_>>();
    println!(
        "Number of tokens to delete: {}",
        todel.len().to_formatted_string(&Locale::en)
    );
    vocab.batch_remove_ngrams(&todel);
    println!("Vocab size: {}", vocab.len().to_formatted_string(&Locale::en));
    let _counts = encode_and_stat(&enc_selfies, &vocab);

    let mut total_counts = Counter::<Selfie>::default();
    let mut times_sampled = Counter::<Selfie>::default();
    for _iter in tqdm!(0..1_000) {
        let sub_vocab = vocab.get_random_vocab(1000);
        for token in sub_vocab.aux_vals() {
            let ngram = sub_vocab.tok_to_ngram(token).unwrap();
            times_sampled[ngram] += 1;
        }

        let enc = par_encode_dataset(&enc_selfies, &sub_vocab);
        let counts = count_token_occurence(&enc, &sub_vocab);

        for (token, count) in counts.iter() {
            let ngram = sub_vocab.tok_to_ngram(*token).unwrap();
            total_counts[ngram] += count;
        }
    }

    println!("Top 20 tokens:");
    for (ngram, count) in total_counts.k_most_common_ordered(20) {
        println!(
            "{}: {} | {}",
            vocab.decode(&ngram),
            count.to_formatted_string(&Locale::en),
            times_sampled[&ngram].to_formatted_string(&Locale::en)
        );
    }
}
