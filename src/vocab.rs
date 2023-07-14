use std::collections::{HashMap, HashSet};

use dashmap::DashSet;

use rand::seq::SliceRandom;
use rayon::prelude::*;

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
    pub fn new(selfies: &Vec<String>) -> Self {
        let mut vocab = Vocab {
            base_vocab: HashMap::new(),
            rev_base: HashMap::new(),

            aux_vocab: HashMap::new(),
            rev_aux: HashMap::new(),
        };

        let tokens: DashSet<String> = DashSet::new();
        selfies.par_iter().for_each(|selfie| {
            let unique = split_selfie(selfie)
                .iter()
                .map(|x| x.to_string())
                .collect::<HashSet<String>>();
            unique.iter().for_each(|token| {
                tokens.insert(token.to_string());
            });
        });

        // Collect tokens into a vector
        let mut tokens: Vec<String> = tokens.iter().map(|x| x.clone()).collect();
        tokens.sort();

        for token in tokens.iter() {
            let len = vocab.base_vocab.len();
            vocab.base_vocab.insert(token.clone(), len);
            vocab.rev_base.insert(len, token.clone());
        }

        vocab
    }

    pub fn insert_ngram(&mut self, tokens: &[usize]) {
        let len = self.aux_vocab.len() + self.base_vocab.len();
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
        let keystart = self.base_vocab.len();
        let keyend = self.base_vocab.len() + self.aux_vocab.len();
        let mut vec: Vec<usize> = (keystart..keyend).collect();
        vec.shuffle(&mut rand::thread_rng());
        vec.truncate(length);

        HashMap::from_iter(vec.iter().map(|x| {
            let key = self.rev_aux.get(x).unwrap();
            (key.clone(), *x)
        }))
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

        Vocab {
            base_vocab: self.base_vocab.clone(),
            rev_base: self.rev_base.clone(),
            aux_vocab: aux.clone(),
            rev_aux: aux_rev.clone(),
        }
    }
}
