use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// use dashmap::DashSet;
use rayon::prelude::*;

use crate::{
    counter::Counter,
    types::{DashSet, HashSet},
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

pub fn generate_ngrams(selfies: &[Vec<u32>], n: usize) -> DashSet<Vec<u32>> {
    let ngrams = DashSet::default();

    selfies.par_iter().for_each(|selfie| {
        let selfie_set = selfie
            .windows(n)
            .map(|ngram| ngram.to_vec())
            .collect::<HashSet<Vec<_>>>();

        for ngram in selfie_set.iter() {
            ngrams.insert(ngram.clone());
        }
    });

    ngrams
}

pub fn count_token_occurence(encoded_selfies: &[Vec<u32>], vocab: &Vocab) -> Counter {
    let mut token_counter = Counter::zeroed(vocab.len());
    encoded_selfies.iter().for_each(|encoded_selfie| {
        token_counter.update(encoded_selfie);
    });

    token_counter
}
