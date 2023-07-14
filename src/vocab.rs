use core::fmt;
use std::collections::HashMap;

use rand::seq::IteratorRandom;

use crate::{encoding::split_selfie};

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

    fn take_aux(&self, length: usize) -> HashMap<Vec<usize>, usize> {
        //here want to return a new Hashmap<Vec<usize>, usize> with n randomly selected entries from self.aux_vocab
        let keys = self.aux_vocab.keys().choose_multiple(&mut rand::thread_rng(), length);
        let mut to_return: HashMap<Vec<usize>, usize> = HashMap::new();
        keys.iter().for_each(|x| {
            to_return.insert((**x).clone(), *self.aux_vocab.get(x.clone()).unwrap());
        });
        to_return
    }

    fn reverse_aux(&self, to_reverse: HashMap<Vec<usize>, usize>) -> HashMap<usize, Vec<usize>> {
        let keys = to_reverse.keys().clone();
        let mut to_return: HashMap<usize, Vec<usize>> = HashMap::new();
        keys.for_each(|x| {
            to_return.insert(*to_reverse.get(x).unwrap(), (*x).clone());
        });
        to_return

    }

    pub fn spawn_child_vocab(&self, vocab_size: usize) -> Vocab {
        //here, want to take the base vocab, and then VOCAB_SIZE - base_voc.len() aux vocab items randomly, to create a new vocab 
        let aux = self.take_aux(vocab_size - self.base_vocab.len());
        let aux_rev = self.reverse_aux(aux.clone());
        let vocab = Vocab {
            base_vocab: self.base_vocab.clone(),
            rev_base: self.rev_base.clone(),
            aux_vocab: aux.clone(),
            rev_aux: aux_rev.clone(),
        };
        vocab
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
