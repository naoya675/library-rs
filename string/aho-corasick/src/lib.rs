use std::collections::VecDeque;

use trie::Trie;

#[derive(Debug, Clone)]
pub struct AhoCorasick {
    trie: Trie,
    fail: usize,
    pattern: Vec<usize>, // number of matches reachable via suffix links
    size: usize,
    base: char,
}

impl AhoCorasick {
    pub fn new(size: usize, base: char) -> Self {
        Self {
            trie: Trie::new(size + 1, base), // goto + failure
            fail: size,
            pattern: vec![],
            size,
            base,
        }
    }

    pub fn insert(&mut self, word: &[char]) {
        self.trie.insert(word);
    }

    pub fn search(&self, word: &[char]) -> bool {
        self.trie.search(word)
    }

    pub fn search_prefix(&self, word: &[char]) -> bool {
        self.trie.search_prefix(word)
    }

    pub fn goto(&self, node_id: usize, c: char) -> usize {
        assert!(!self.pattern.is_empty(), "call build() before goto()");
        self.trie.next(node_id, self.trie.ctoi(c)).unwrap()
    }

    pub fn fail(&self, node_id: usize) -> usize {
        assert!(!self.pattern.is_empty(), "call build() before fail()");
        self.trie.next(node_id, self.fail).unwrap()
    }

    // build PMA (Pattern Matching Automaton)
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
                        let u = self.trie.accept(next_id).to_vec();
                        let v = self.trie.accept(self.trie.next(fail, i).unwrap()).to_vec();
                        let mut merged = Vec::with_capacity(u.len() + v.len());
                        let mut j = 0;
                        let mut k = 0;
                        while j < u.len() && k < v.len() {
                            if u[j] == v[k] {
                                merged.push(u[j]);
                                j += 1;
                                k += 1;
                            } else if u[j] < v[k] {
                                merged.push(u[j]);
                                j += 1;
                            } else if u[j] > v[k] {
                                merged.push(v[k]);
                                k += 1;
                            }
                        }
                        merged.extend_from_slice(&u[j..]);
                        merged.extend_from_slice(&v[k..]);
                        *self.trie.accept_mut(next_id) = merged;
                    }
                    que.push_back(next_id);
                } else {
                    *self.trie.next_mut(node_id, i) = self.trie.next(fail, i);
                }
            }
        }
    }

    pub fn matches(&self, word: &[char]) -> Vec<usize> {
        self.matches_from(word, 0)
    }

    pub fn matches_from(&self, word: &[char], mut node_id: usize) -> Vec<usize> {
        let mut res = vec![0; self.trie.count()];
        for &c in word {
            node_id = self.goto(node_id, c);
            for &id in self.trie.accept(node_id) {
                res[id] += 1;
            }
        }
        res
    }

    pub fn next_word(&self, word: &[char], mut node_id: usize) -> (usize, usize) {
        let mut ret = 0;
        for &c in word {
            let (pattern, next) = self.next(node_id, c);
            ret += pattern;
            node_id = next;
        }
        (ret, node_id)
    }

    pub fn next(&self, node_id: usize, c: char) -> (usize, usize) {
        if let Some(next) = self.trie.next(node_id, self.trie.ctoi(c)) {
            return (self.pattern[next], next);
        }
        unreachable!()
    }
}
