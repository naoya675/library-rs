// reference: https://heno239.hatenablog.com/entry/2020/07/07/140651

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct ZAlgorithm<T> {
    s: Vec<T>,
    z: Vec<usize>,
    delete: Vec<usize>,
    delete_memo: Vec<Vec<usize>>,
    cur: VecDeque<usize>,
}

impl<T: Copy + PartialEq> ZAlgorithm<T> {
    pub fn new() -> Self {
        Self {
            s: vec![],
            z: vec![],
            delete: vec![],
            delete_memo: vec![vec![]],
            cur: VecDeque::new(),
        }
    }

    pub fn build(&mut self, s: &Vec<T>) {
        for i in 0..s.len() {
            self.set(s[i]);
        }
    }

    fn delete(&mut self, q: usize, len: usize) {
        self.delete[q] = 1;
        self.delete_memo[len].push(q);
        self.z[q] = len - q - 1;
    }

    pub fn set(&mut self, c: T) {
        self.s.push(c);
        self.z.push(0);
        self.delete.push(0);
        self.delete_memo.push(vec![]);
        self.z[0] += 1;

        let len = self.s.len();
        if len == 1 {
            return;
        }
        if self.s[0] == c {
            self.cur.push_back(len - 1);
        } else {
            self.delete[len - 1] = 1;
        }

        while let Some(&q) = self.cur.front() {
            if self.delete[q] != 0 {
                self.cur.pop_front();
                continue;
            }
            if self.s[len - 1 - q] == *self.s.last().unwrap() {
                break;
            } else {
                self.delete(q, len);
                self.cur.pop_front();
            }
        }

        if let Some(&q) = self.cur.front() {
            for p in self.delete_memo[len - q].clone() {
                self.delete(p + q, len);
            }
        }
    }

    pub fn get(&self) -> Vec<usize> {
        let mut res = vec![0; self.s.len()];
        for i in 0..self.s.len() {
            res[i] = self.internal_get(i);
        }
        res
    }

    fn internal_get(&self, k: usize) -> usize {
        if self.delete[k] != 0 {
            self.z[k]
        } else {
            self.s.len() - k
        }
    }
}

pub fn z_algorithm<T: Copy + PartialEq>(s: &Vec<T>) -> Vec<usize> {
    if s.len() == 0 {
        return vec![];
    }
    let mut res = vec![0; s.len()];
    let mut i = 1;
    let mut j = 0;
    while i < s.len() {
        while i + j < s.len() && s[i + j] == s[j] {
            j += 1;
        }
        res[i] = j;
        if j == 0 {
            i += 1;
            continue;
        }
        let mut k = 1;
        while i + k < s.len() && k + res[k] < j {
            res[i + k] = res[k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    res[0] = s.len();
    res
}

/*
// reference: https://atcoder.github.io/ac-library/production/document_en/string.html

pub fn z_algorithm<T: Copy + PartialEq>(s: &Vec<T>) -> Vec<usize> {
    if s.len() == 0 {
        return vec![];
    }
    let mut res = vec![0; s.len()];
    let mut i = 1;
    let mut j = 0;
    while i < s.len() {
        res[i] = if res[j] + j <= i { 0 } else { std::cmp::min(res[j] + j - i, res[i - j]) };
        while i + res[i] < s.len() && s[res[i]] == s[i + res[i]] {
            res[i] += 1;
        }
        if res[j] + j < res[i] + i {
            j = i;
        }
        i += 1;
    }
    res[0] = s.len();
    res
}
*/
