use std::collections::{HashMap, HashSet};

use dashmap::DashSet;
use rayon::prelude::*;

use crate::encoding::split_selfie;

pub struct Vocab {
    vocab: HashMap<String, usize>,
    rev_vocab: HashMap<usize, String>,
}

impl Vocab {
    pub fn new() -> Self {
        Vocab {
            vocab: HashMap::new(),
            rev_vocab: HashMap::new(),
        }
    }

    pub fn from_data(selfies: &[String]) -> Self {
        let mut vocab = Vocab::new();

        let tokens = DashSet::new();
        selfies.par_iter().for_each(|selfie| {
            let tokens_set = split_selfie(selfie).iter().copied().collect::<HashSet<&str>>();
            for token in tokens_set {
                tokens.insert(token);
            }
        });

        for token in tokens {
            vocab.insert(token);
        }

        vocab
    }

    pub fn insert(&mut self, token: &str) {
        let len = self.vocab.len();
        if !self.contains(token) {
            self.vocab.insert(token.to_string(), len);
            self.rev_vocab.insert(len, token.to_string());
        }
    }

    pub fn get(&self, token: &str) -> Option<&usize> {
        self.vocab.get(token)
    }

    pub fn get_rev(&self, token: &usize) -> Option<&String> {
        self.rev_vocab.get(token)
    }

    pub fn len(&self) -> usize {
        self.vocab.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vocab.is_empty()
    }

    pub fn contains(&self, token: &str) -> bool {
        self.vocab.contains_key(token)
    }

    pub fn contains_rev(&self, token: &usize) -> bool {
        self.rev_vocab.contains_key(token)
    }

    pub fn values(&self) -> Vec<usize> {
        self.vocab.values().copied().collect()
    }
}

impl Default for Vocab {
    fn default() -> Self {
        Self::new()
    }
}
