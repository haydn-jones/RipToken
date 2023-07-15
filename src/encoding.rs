use rayon::prelude::*;

use crate::{types::Selfie, vocab::Vocab};

// Dynamic programming approach to find optimal encoding
// i.e. the encoding with the fewest tokens
pub fn optimal_encode(tokens: &Selfie, vocab: &Vocab) -> Selfie {
    let mut encodings: Vec<Selfie> = (0..tokens.len() + 1).map(|i| tokens[0..i].to_vec()).collect();

    for i in 0..tokens.len() {
        for j in 0..=i {
            if !encodings[i + 1].is_empty() && (encodings[j].len() + 1) >= encodings[i + 1].len() {
                continue;
            }

            if let Some(value) = vocab.get_aux(&tokens[j..=i]) {
                let mut new_enc = encodings[j].clone();
                new_enc.push(*value);
                if encodings[i + 1].is_empty() || new_enc.len() < encodings[i + 1].len() {
                    encodings[i + 1] = new_enc;
                }
            }
        }
    }

    encodings.last().unwrap().clone()
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

pub fn par_encode_dataset(selfies: &[Selfie], vocab: &Vocab) -> Vec<Selfie> {
    selfies.par_iter().map(|selfie| optimal_encode(selfie, vocab)).collect()
}
