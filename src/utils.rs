use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use dashmap::DashSet;
use rayon::prelude::*;

use crate::encoding::{split_selfie, split_selfie_indices_n};

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

pub fn get_init_vocab(selfies: &[String]) -> HashMap<&str, usize> {
    // get set of unique tokens
    let mut vocab = HashMap::new();
    for selfie in selfies {
        let tokens = split_selfie(selfie);
        for token in tokens {
            if !vocab.contains_key(token) {
                let len = vocab.len();
                vocab.insert(token, len);
            }
        }
    }

    vocab
}

pub fn generate_subsets(selfies: &[String], n: usize) -> DashSet<&str> {
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
