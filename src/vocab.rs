use std::collections::{HashMap, HashSet};

use dashmap::DashSet;
use rand::seq::SliceRandom;
use rayon::prelude::*;

use crate::encoding::split_selfie;

pub struct Vocab {
    // base vocab
    base_vocab: HashMap<String, u32>,
    rev_base: HashMap<u32, String>,

    // aux vocab
    aux_vocab: HashMap<Vec<u32>, u32>,
    rev_aux: HashMap<u32, Vec<u32>>,
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
            let len = vocab.get_base_idx();
            vocab.base_vocab.insert(token.clone(), len);
            vocab.rev_base.insert(len, token.clone());
        }

        vocab
    }

    pub fn insert_ngram(&mut self, tokens: &[u32]) {
        let len = self.get_aux_idx();
        if !self.aux_vocab.contains_key(tokens) {
            self.aux_vocab.insert(tokens.to_vec(), len);
            self.rev_aux.insert(len, tokens.to_vec());
        }
    }

    pub fn get_base(&self, token: &str) -> Option<&u32> {
        self.base_vocab.get(token)
    }

    pub fn base_encode(&self, selfie: &str) -> Vec<u32> {
        split_selfie(selfie)
            .iter()
            .map(|token| *self.get_base(token).unwrap())
            .collect::<Vec<u32>>()
    }

    pub fn get_aux(&self, tokens: &[u32]) -> Option<&u32> {
        self.aux_vocab.get(tokens)
    }

    pub fn decode(&self, encoded: &[u32]) -> String {
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

    fn take_aux(&self, length: usize) -> HashMap<Vec<u32>, u32> {
        let mut vec: Vec<u32> = (self.get_base_idx()..self.get_aux_idx()).collect();
        vec.shuffle(&mut rand::thread_rng());
        vec.truncate(length);

        HashMap::from_iter(vec.iter().map(|x| {
            let key = self.rev_aux.get(x).unwrap();
            (key.clone(), *x)
        }))
    }

    fn reverse_aux(&self, to_reverse: HashMap<Vec<u32>, u32>) -> HashMap<u32, Vec<u32>> {
        let keys = to_reverse.keys().clone();
        let mut to_return: HashMap<u32, Vec<u32>> = HashMap::new();
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

    fn get_aux_idx(&self) -> u32 {
        (self.base_vocab.len() + self.aux_vocab.len()).try_into().unwrap()
    }

    fn get_base_idx(&self) -> u32 {
        self.base_vocab.len().try_into().unwrap()
    }
}
