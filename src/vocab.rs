use core::fmt;
use std::collections::HashMap;

use crate::encoding::split_selfie;

pub struct Vocab {
    // base vocab
    base_vocab: HashMap<String, usize>,
    rev_base: HashMap<usize, String>,

    // aux vocab
    aux_vocab: HashMap<Vec<usize>, usize>,
    rev_aux: HashMap<usize, Vec<usize>>,
}

impl Vocab {
    pub fn new(base_tokens: Vec<String>) -> Self {
        let mut vocab = Vocab {
            base_vocab: HashMap::new(),
            rev_base: HashMap::new(),

            aux_vocab: HashMap::new(),
            rev_aux: HashMap::new(),
        };

        // [C]: 0
        // [F]: 1
        for token in base_tokens {
            let len = vocab.base_vocab.len();
            vocab.base_vocab.insert(token.clone(), len);
            vocab.rev_base.insert(len, token.clone());
        }

        vocab
    }

    pub fn insert_ngram(&mut self, tokens: &[usize]) {
        let len = self.aux_vocab.len();
        if !self.aux_vocab.contains_key(tokens) {
            self.aux_vocab.insert(tokens.to_vec(), len);
            self.rev_aux.insert(len, tokens.to_vec());
        }
    }

    pub fn get_base(&self, token: &str) -> Option<usize> {
        self.base_vocab.get(token).copied()
    }

    pub fn base_encode(&self, selfie: &str) -> Vec<usize> {
        split_selfie(selfie)
            .iter()
            .map(|token| self.get_base(token).unwrap())
            .collect::<Vec<usize>>()
    }

    pub fn get_aux(&self, tokens: &[usize]) -> Option<usize> {
        self.aux_vocab.get(tokens).copied()
    }

    pub fn decode(&self, encoded: &[usize]) -> String {
        let mut flattened = Vec::new();
        for token in encoded {
            if let Some(tokens) = self.rev_aux.get(token) {
                flattened.extend(tokens);
            } else {
                flattened.push(*token);
            }
        }

        flattened
            .iter()
            .map(|token| (*self.rev_base.get(token).unwrap()).clone())
            .collect::<Vec<String>>()
            .join("")
    }
}

impl fmt::Display for Vocab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tokens = (0..10)
            .map(|i| {
                let token = self.rev_base.get(&i).unwrap();
                format!("{}: {}", i, token)
            })
            .collect::<Vec<String>>();

        let aux_tokens = (0..10)
            .map_while(|i| {
                if let Some(tokens) = self.rev_aux.get(&i) {
                    let str_tokens = tokens
                        .iter()
                        .map(|token| (*self.rev_base.get(token).unwrap()).clone())
                        .collect::<Vec<String>>();
                    Some(format!("{}: {} | {:?} ", i, str_tokens.join(""), tokens))
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();

        write!(f, "Base:\n{}\nAux:\n{}", tokens.join("\n"), aux_tokens.join("\n"))
    }
}
