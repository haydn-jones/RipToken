use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use counter::Counter;
// use dashmap::DashSet;
use rayon::prelude::*;

use crate::{
    types::{DashSet, HashSet, Selfie, DashMap},
    vocab::Vocab,
};

// read file into vector of lines
pub fn read_file(filename: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let file = File::open(filename).expect("file not found");
    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
        lines.push(line.expect("Could not parse line"));
    }
    lines
}

pub fn generate_ngrams(selfies: &[Selfie], n: usize) -> DashSet<Selfie> {
    // This function only generates ngrams that occur more than 'n' times

    let ngrams = DashSet::default();
    let ngram_counts: DashMap<Selfie, usize> = DashMap::default();

    selfies.par_iter().for_each(|selfie| {
        let selfie_set = selfie
            .windows(n)
            .map(|ngram| ngram.to_vec())
            .collect::<HashSet<Vec<_>>>();

        for ngram in selfie_set.iter() {
            ngram_counts.entry(ngram.clone()).and_modify(|count| *count += 1).or_insert(1);
            if ngram_counts.get(ngram).unwrap().value() > &n {
                ngrams.insert(ngram.clone());
            }
        }
    });

    ngrams
}

pub fn count_token_occurence(encoded_selfies: &[Selfie], vocab: &Vocab) -> Counter<u32> {
    let mut token_counter = Counter::default();
    for token in vocab.aux_vals() {
        token_counter.insert(token, 0);
    }

    for selfie in encoded_selfies.iter() {
        for token in selfie.iter() {
            if vocab.is_aux(token) {
                token_counter[token] += 1;
            }
        }
    }

    token_counter
}
