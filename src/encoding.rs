use std::collections::VecDeque;

use rayon::prelude::*;

use crate::vocab::Vocab;

// Dynamic programming approach to find optimal encoding
// i.e. the encoding with the fewest tokens
pub fn optimal_encode(selfie: &str, vocab: &Vocab) -> Result<Vec<usize>, &'static str> {
    let token_indices: Vec<(usize, usize)> = split_selfie_indices(selfie);
    let mut encodings: Vec<Vec<usize>> = vec![vec![]; token_indices.len() + 1];

    for i in 0..token_indices.len() {
        for j in 0..=i {
            if (encodings[j].len() + 1) >= encodings[i + 1].len() && !encodings[i + 1].is_empty() {
                continue;
            }
            let substring = &selfie[token_indices[j].0..token_indices[i].1];

            if let Some(value) = vocab.get(substring) {
                encodings[i + 1] = encodings[j].clone();
                encodings[i + 1].push(*value);
            }
        }
    }

    encodings.last().cloned().ok_or("Cannot encode string with given vocab")
}

// This assumes the selfie is valid, please don't give poor little Rusty invalid selfies
// he's just a little guy
pub fn split_selfie(selfie: &str) -> Vec<&str> {
    let mut tokens = Vec::with_capacity(4);

    let mut token_start = 0;
    selfie.char_indices().for_each(|(i, c)| {
        if c == ']' {
            tokens.push(&selfie[token_start..i + 1]);
            token_start = i + 1;
        }
    });

    tokens
}

pub fn split_selfie_indices(selfie: &str) -> Vec<(usize, usize)> {
    let mut tokens = Vec::with_capacity(4);

    let mut token_start = 0;
    selfie.char_indices().for_each(|(i, c)| {
        if c == ']' {
            tokens.push((token_start, i + 1));
            token_start = i + 1;
        }
    });

    tokens
}

pub fn split_selfie_indices_n(selfie: &str, n: usize) -> Vec<(usize, usize)> {
    let mut tokens = Vec::with_capacity(4);
    let mut queue = VecDeque::with_capacity(n);

    let mut token_start = 0;
    selfie.char_indices().for_each(|(i, c)| {
        if c == ']' {
            queue.push_back(token_start);
            token_start = i + 1;
            if queue.len() == n {
                tokens.push((queue.pop_front().unwrap(), i + 1));
            }
        }
    });

    tokens
}

pub fn par_encode_dataset(selfies: &[String], vocab: &Vocab) -> Vec<Vec<usize>> {
    selfies
        .par_iter()
        .map(|selfie| optimal_encode(selfie, vocab).unwrap())
        .collect()
}
