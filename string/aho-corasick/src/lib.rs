use std::collections::{HashMap, VecDeque};

use trie::Trie;

#[derive(Debug, Clone)]
pub struct AhoCorasick {
    trie: Trie,
    fail: usize,
    pattern: Vec<usize>, // number of matches
    size: usize,
    base: char,
}

impl AhoCorasick {
    pub fn new(size: usize, base: char) -> Self {
        Self {
            trie: Trie::new(size + 1, base), // trie + failed link
            fail: size,
            pattern: vec![],
            size,
            base,
        }
    }

    #[inline]
    pub fn insert(&mut self, word: &Vec<char>) {
        self.trie.insert(word);
    }

    #[inline]
    pub fn search(&self, word: &Vec<char>) -> bool {
        self.trie.search(word)
    }

    #[inline]
    pub fn search_prefix(&self, word: &Vec<char>) -> bool {
        self.trie.search_prefix(word)
    }

    // build Pattern Matching Automaton (PMA)
    pub fn build(&mut self, heavy: bool) {
        self.pattern.resize(self.trie.size(), 0);
        for i in 0..self.trie.size() {
            self.pattern[i] = self.trie.accept(i).len();
        }

        let mut que = VecDeque::new();
        for i in 0..=self.size {
            if let Some(next_id) = self.trie.next(0, i) {
                *self.trie.next_mut(next_id, self.fail) = Some(0);
                que.push_back(next_id);
            } else {
                *self.trie.next_mut(0, i) = Some(0);
            }
        }
        while let Some(node_id) = que.pop_front() {
            let fail = self.trie.next(node_id, self.fail).unwrap();
            self.pattern[node_id] += self.pattern[fail];
            for i in 0..self.size {
                if let Some(next_id) = self.trie.next(node_id, i) {
                    *self.trie.next_mut(next_id, self.fail) = self.trie.next(fail, i);
                    if heavy {
                        // set_union
                        let mut merged = vec![];
                        let u = self.trie.accept(next_id);
                        let v = self.trie.accept(self.trie.next(fail, i).unwrap());
                        merged.extend(u.iter().cloned());
                        merged.extend(v.iter().cloned());
                        merged.sort_unstable();
                        merged.dedup();
                        *self.trie.accpet_mut(next_id) = merged;
                    }
                    que.push_back(next_id);
                } else {
                    *self.trie.next_mut(node_id, i) = self.trie.next(fail, i);
                }
            }
        }
    }

    pub fn matches(&self, word: &Vec<char>, mut now: usize) -> HashMap<usize, usize> {
        let mut res: HashMap<usize, usize> = HashMap::new();
        let mut cnt: HashMap<usize, usize> = HashMap::new();

        for &c in word {
            let c = (c as usize) - (self.base as usize);
            now = self.trie.next(now, c).unwrap();
            *cnt.entry(now).or_default() += 1;
        }
        for (now, cnt) in cnt {
            for &id in self.trie.accept(now) {
                *res.entry(id).or_default() += cnt;
            }
        }
        res
    }

    pub fn next_word(&self, word: &Vec<char>, mut now: usize) -> (usize, usize) {
        let mut total = 0;
        for &c in word {
            let (pattern, next) = self.next(c, now);
            total += pattern;
            now = next;
        }
        (total, now)
    }

    pub fn next(&self, c: char, now: usize) -> (usize, usize) {
        let c = (c as usize) - (self.base as usize);
        if let Some(now) = self.trie.next(now, c) {
            return (self.pattern[now], now);
        }
        unreachable!()
    }
}
