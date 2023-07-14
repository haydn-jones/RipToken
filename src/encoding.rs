use std::collections::VecDeque;

use crate::vocab::Vocab;

// Dynamic programming approach to find optimal encoding
// i.e. the encoding with the fewest tokens
pub fn optimal_encode(tokens: &[u32], vocab: &Vocab) -> Vec<u32> {
    let mut encodings: Vec<Vec<u32>> = (0..tokens.len() + 1).map(|i| tokens[0..i].to_vec()).collect();

    for i in 0..tokens.len() {
        for j in 0..i {
            if (encodings[j].len() + 1) >= encodings[i + 1].len() {
                continue;
            }
            if let Some(value) = vocab.get_aux(&tokens[j..=i]) {
                encodings[i + 1] = encodings[j].clone();
                encodings[i + 1].push(*value);
            }
        }
    }

    encodings.pop().unwrap()
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

pub fn par_encode_dataset(_selfies: &[String], _vocab: &Vocab) -> Vec<Vec<usize>> {
    //selfies
    //    .par_iter()
    //    .map(|selfie| optimal_encode(selfie, vocab).unwrap())
    //    .collect()

    Vec::new()
}
