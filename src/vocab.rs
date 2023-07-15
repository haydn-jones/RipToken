use ahash::HashSet;
use rand::seq::SliceRandom;
use rayon::prelude::*;

use crate::{
    encoding::split_selfie,
    types::{BiMap, DashSet, Selfie, Token},
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Vocab {
    // base vocab
    base_vocab: BiMap<String, Token>,
    aux_vocab: BiMap<Selfie, Token>,
}

impl Vocab {
    pub fn new(selfies: &Vec<String>) -> Self {
        let mut vocab = Vocab {
            base_vocab: BiMap::default(),
            aux_vocab: BiMap::default(),
        };

        let tokens: DashSet<String> = DashSet::default();
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
        }

        vocab
    }

    pub fn insert_ngram(&mut self, tokens: &Selfie) {
        if !self.aux_vocab.contains_left(tokens) {
            self.aux_vocab.insert(tokens.to_vec(), self.get_aux_idx());
        }
    }

    pub fn get_base(&self, token: &str) -> Option<&Token> {
        self.base_vocab.get_by_left(token)
    }

    pub fn base_encode(&self, selfie: &str) -> Selfie {
        split_selfie(selfie)
            .iter()
            .map(|token| *self.get_base(token).unwrap())
            .collect::<Vec<u32>>()
    }

    pub fn get_aux(&self, tokens: &[Token]) -> Option<&Token> {
        self.aux_vocab.get_by_left(tokens)
    }

    pub fn decode(&self, encoded: &Selfie) -> String {
        let mut flattened = Vec::new();
        for token in encoded {
            if let Some(tokens) = self.aux_vocab.get_by_right(token) {
                flattened.extend(tokens);
            } else {
                flattened.push(*token);
            }
        }

        flattened
            .iter()
            .map(|token| (*self.base_vocab.get_by_right(token).unwrap()).clone())
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn get_random_vocab(&self, vocab_size: usize) -> Vocab {
        let aux_size = vocab_size - self.base_vocab.len();
        let mut aux_idxs: Vec<Token> = (self.get_base_idx()..self.get_aux_idx()).collect();
        aux_idxs.shuffle(&mut rand::thread_rng());
        aux_idxs.truncate(aux_size);

        let mut new_voc = Vocab {
            base_vocab: self.base_vocab.clone(),
            aux_vocab: BiMap::default(),
        };

        for aux_idx in aux_idxs.iter() {
            let aux = self.aux_vocab.get_by_right(aux_idx).unwrap().clone();
            new_voc.insert_ngram(&aux);
        }

        new_voc
    }

    pub fn batch_remove_ngrams(&mut self, idxs: &[Token]) {
        let todel = HashSet::from_iter(idxs.iter().cloned());
        let mut tokeep: Vec<Selfie> = Vec::default();
        for (ngram, tok) in self.aux_vocab.iter() {
            if !todel.contains(tok) {
                tokeep.push((*ngram).clone());
            }
        }

        self.aux_vocab.clear();
        for ngram in tokeep {
            self.insert_ngram(&ngram)
        }
    }

    fn get_aux_idx(&self) -> Token {
        (self.base_vocab.len() + self.aux_vocab.len()).try_into().unwrap()
    }

    fn get_base_idx(&self) -> Token {
        self.base_vocab.len().try_into().unwrap()
    }

    pub fn len(&self) -> usize {
        self.base_vocab.len() + self.aux_vocab.len()
    }

    pub fn is_empty(&self) -> bool {
        self.base_vocab.is_empty() && self.aux_vocab.is_empty()
    }

    pub fn aux_vals(&self) -> Vec<Token> {
        self.aux_vocab.right_values().cloned().collect::<Vec<u32>>()
    }

    pub fn is_aux(&self, idx: &Token) -> bool {
        self.aux_vocab.contains_right(idx)
    }

    pub fn tok_to_ngram(&self, idx: Token) -> Option<&Selfie> {
        self.aux_vocab.get_by_right(&idx)
    }
}
