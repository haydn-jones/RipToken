use std::collections::HashMap;

// Dynamic programming approach to find optimal encoding
// i.e. the encoding with the fewest tokens
pub fn optimal_encode(selfie: &str, vocab: &HashMap<&str, usize>) -> Result<Vec<usize>, &'static str> {
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

pub fn encode_selfie(selfie: &str, vocab: &HashMap<String, usize>) -> Vec<usize> {
    let mut encoded = Vec::new();

    let tokens = split_selfie(selfie);
    for token in tokens {
        encoded.push(vocab[token]);
    }

    encoded
}

pub fn decode_selfie(encoded: &Vec<usize>, vocab: &HashMap<String, usize>) -> String {
    // reverse vocab
    let rev_vocab: HashMap<usize, String> = vocab.iter().map(|(k, v)| (*v, k.clone())).collect();
    let mut decoded = String::new();

    for token in encoded {
        decoded.push_str(&rev_vocab[token]);
    }
    decoded
}

// This assumes the selfie is valid, please don't give poor little Rusty invalid selfies
// he's just a little guy
pub fn split_selfie(selfie: &str) -> Vec<&str> {
    let mut tokens = Vec::new();

    let mut token_start = 0;
    for (i, c) in selfie.char_indices() {
        match c {
            ']' => {
                tokens.push(&selfie[token_start..i + 1]);
                token_start = i + 1;
            }
            _ => continue,
        }
    }

    tokens
}

pub fn split_selfie_indices(selfie: &str) -> Vec<(usize, usize)> {
    let mut tokens = Vec::new();

    let mut token_start = 0;
    for (i, c) in selfie.char_indices() {
        match c {
            ']' => {
                tokens.push((token_start, i + 1));
                token_start = i + 1;
            }
            _ => continue,
        }
    }

    tokens
}

pub fn split_selfie_indices_n(selfie: &str, n: usize) -> Vec<(usize, usize)> {
    let mut tokens = Vec::new();
    let mut queue = Vec::new();

    let mut token_start = 0;
    for (i, c) in selfie.char_indices() {
        match c {
            ']' => {
                queue.push(token_start);
                if queue.len() == n {
                    tokens.push((queue[0], i + 1));
                    queue.remove(0);
                }
                token_start = i + 1;
            }
            _ => continue,
        }
    }

    tokens
}
