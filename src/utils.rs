use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use dashmap::DashSet;
use rayon::prelude::*;

use crate::{counter::Counter, encoding::split_selfie_indices_n, vocab::Vocab};

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

pub fn generate_ngrams(selfies: &[String], n: usize) -> DashSet<&str> {
    let ngrams = DashSet::new();

    selfies.par_iter().for_each(|selfie| {
        let selfie_set = split_selfie_indices_n(selfie, n)
            .iter()
            .map(|w| &selfie[w.0..w.1])
            .collect::<HashSet<&str>>();
        for ngram in selfie_set {
            ngrams.insert(ngram);
        }
    });

    ngrams
}

pub fn count_token_occurence(encoded_selfies: &[Vec<usize>], _vocab: &Vocab) -> Counter {
    // let mut token_counter = Counter::zeroed(vocab.len());
    let mut token_counter = Counter::zeroed(9);
    encoded_selfies.iter().for_each(|encoded_selfie| {
        token_counter.update(encoded_selfie);
    });

    token_counter
}
