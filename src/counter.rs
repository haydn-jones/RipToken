#[derive(Default)]
pub struct Counter {
    // Counts number of occurrences of each value.
    // Each key is usize, and each value is usize.
    // Assumes keys are in a fixed range [0, N)
    counts: Vec<usize>,
}

use std::{cmp::Reverse, collections::BinaryHeap};

impl Counter {
    pub fn zeroed(size: usize) -> Self {
        Self { counts: vec![0; size] }
    }

    pub fn insert(&mut self, key: u32) {
        self.counts[key as usize] += 1;
    }

    pub fn update(&mut self, keys: &[u32]) {
        for key in keys.iter() {
            self.insert(*key);
        }
    }
    pub fn len(&self) -> usize {
        self.counts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.counts.is_empty()
    }

    pub fn iter(&self) -> CounterIter {
        self.into_iter()
    }

    pub fn values(&self) -> &[usize] {
        &self.counts
    }
}

pub struct CounterIter<'a> {
    counter: &'a Counter,
    index: usize,
}

impl<'a> IntoIterator for &'a Counter {
    type Item = (u32, &'a usize);
    type IntoIter = CounterIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CounterIter {
            counter: self,
            index: 0,
        }
    }
}

impl<'a> Iterator for CounterIter<'a> {
    type Item = (u32, &'a usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.counter.counts.len() {
            let result = (self.index as u32, &self.counter.counts[self.index]);
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

impl Counter {
    pub fn most_common(&self, k: usize) -> Vec<(u32, usize)> {
        let mut heap = BinaryHeap::with_capacity(k);

        for (value, &count) in self.counts.iter().enumerate() {
            if heap.len() < k {
                heap.push(Reverse((count, value)));
            } else if let Some(&Reverse((min_count, _min_value))) = heap.peek() {
                if count > min_count {
                    heap.pop();
                    heap.push(Reverse((count, value)));
                }
            }
        }

        let mut ret: Vec<(u32, usize)> = heap
            .into_iter()
            .map(|Reverse((count, value))| (value as u32, count))
            .collect();
        ret.sort_by(|a, b| b.1.cmp(&a.1));
        ret
    }

    pub fn least_common(&self, k: usize) -> Vec<(usize, usize)> {
        let mut heap = BinaryHeap::with_capacity(k);

        for (value, &count) in self.counts.iter().enumerate() {
            if heap.len() < k {
                heap.push((count, value));
            } else if let Some(&(max_count, _max_value)) = heap.peek() {
                if count < max_count {
                    heap.pop();
                    heap.push((count, value));
                }
            }
        }

        let mut ret: Vec<(usize, usize)> = heap.into_iter().map(|(count, value)| (value, count)).collect();
        ret.sort_by(|a, b| a.1.cmp(&b.1));
        ret
    }

    pub fn counts_less_n(&self, n: usize) -> Vec<usize> {
        let counts = self
            .counts
            .iter()
            .enumerate()
            .filter_map(|(v, count)| if *count < n { Some(v) } else { None })
            .collect::<Vec<_>>();
        counts
    }
}
