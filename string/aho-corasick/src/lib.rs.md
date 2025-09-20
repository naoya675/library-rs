---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: string/trie/src/lib.rs
    title: string/trie/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/aho_corasick/src/main.rs
    title: verification/library-checker/aho_corasick/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::collections::{HashMap, VecDeque};\n\nuse trie::Trie;\n\n#[derive(Debug,\
    \ Clone)]\npub struct AhoCorasick {\n    trie: Trie,\n    fail: usize,\n    pattern:\
    \ Vec<usize>, // number of matches\n    size: usize,\n    base: char,\n}\n\nimpl\
    \ AhoCorasick {\n    pub fn new(size: usize, base: char) -> Self {\n        Self\
    \ {\n            trie: Trie::new(size + 1, base), // goto + failure\n        \
    \    fail: size,\n            pattern: vec![],\n            size,\n          \
    \  base,\n        }\n    }\n\n    #[inline]\n    pub fn insert(&mut self, word:\
    \ &Vec<char>) {\n        self.trie.insert(word);\n    }\n\n    #[inline]\n   \
    \ pub fn search(&self, word: &Vec<char>) -> bool {\n        self.trie.search(word)\n\
    \    }\n\n    #[inline]\n    pub fn search_prefix(&self, word: &Vec<char>) ->\
    \ bool {\n        self.trie.search_prefix(word)\n    }\n\n    // build Pattern\
    \ Matching Automaton (PMA)\n    pub fn build(&mut self, heavy: bool) {\n     \
    \   self.pattern.resize(self.trie.size(), 0);\n        for i in 0..self.trie.size()\
    \ {\n            self.pattern[i] = self.trie.accept(i).len();\n        }\n\n \
    \       let mut que = VecDeque::new();\n        for i in 0..=self.size {\n   \
    \         if let Some(next_id) = self.trie.next(0, i) {\n                *self.trie.next_mut(next_id,\
    \ self.fail) = Some(0);\n                que.push_back(next_id);\n           \
    \ } else {\n                *self.trie.next_mut(0, i) = Some(0);\n           \
    \ }\n        }\n        while let Some(node_id) = que.pop_front() {\n        \
    \    let fail = self.trie.next(node_id, self.fail).unwrap();\n            self.pattern[node_id]\
    \ += self.pattern[fail];\n            for i in 0..self.size {\n              \
    \  if let Some(next_id) = self.trie.next(node_id, i) {\n                    *self.trie.next_mut(next_id,\
    \ self.fail) = self.trie.next(fail, i);\n                    if heavy {\n    \
    \                    // set_union\n                        let mut merged = vec![];\n\
    \                        let u = self.trie.accept(next_id);\n                \
    \        let v = self.trie.accept(self.trie.next(fail, i).unwrap());\n       \
    \                 merged.extend(u.iter().cloned());\n                        merged.extend(v.iter().cloned());\n\
    \                        merged.sort_unstable();\n                        merged.dedup();\n\
    \                        *self.trie.accpet_mut(next_id) = merged;\n          \
    \          }\n                    que.push_back(next_id);\n                } else\
    \ {\n                    *self.trie.next_mut(node_id, i) = self.trie.next(fail,\
    \ i);\n                }\n            }\n        }\n    }\n\n    pub fn matches(&self,\
    \ word: &Vec<char>, mut now: usize) -> HashMap<usize, usize> {\n        let mut\
    \ res: HashMap<usize, usize> = HashMap::new();\n        let mut cnt: HashMap<usize,\
    \ usize> = HashMap::new();\n\n        for &c in word {\n            let c = (c\
    \ as usize) - (self.base as usize);\n            now = self.trie.next(now, c).unwrap();\n\
    \            *cnt.entry(now).or_default() += 1;\n        }\n        for (now,\
    \ cnt) in cnt {\n            for &id in self.trie.accept(now) {\n            \
    \    *res.entry(id).or_default() += cnt;\n            }\n        }\n        res\n\
    \    }\n\n    pub fn next_word(&self, word: &Vec<char>, mut now: usize) -> (usize,\
    \ usize) {\n        let mut total = 0;\n        for &c in word {\n           \
    \ let (pattern, next) = self.next(c, now);\n            total += pattern;\n  \
    \          now = next;\n        }\n        (total, now)\n    }\n\n    pub fn next(&self,\
    \ c: char, now: usize) -> (usize, usize) {\n        let c = (c as usize) - (self.base\
    \ as usize);\n        if let Some(now) = self.trie.next(now, c) {\n          \
    \  return (self.pattern[now], now);\n        }\n        unreachable!()\n    }\n\
    }\n"
  dependsOn:
  - string/trie/src/lib.rs
  isVerificationFile: false
  path: string/aho-corasick/src/lib.rs
  requiredBy: []
  timestamp: '2025-09-21 00:52:09+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/library-checker/aho_corasick/src/main.rs
documentation_of: string/aho-corasick/src/lib.rs
layout: document
title: Aho-Corasick
---

## Description

## Reference
- [https://noshi91.github.io/algorithm-encyclopedia/aho-corasick](https://noshi91.github.io/algorithm-encyclopedia/aho-corasick)
- [https://naoya-2.hatenadiary.org/entry/20090405/aho_corasick](https://naoya-2.hatenadiary.org/entry/20090405/aho_corasick)
- [https://chakku.hatenablog.com/entry/2017/12/01/020546](https://chakku.hatenablog.com/entry/2017/12/01/020546)
- [https://cp-algorithms.com/string/aho_corasick.html](https://cp-algorithms.com/string/aho_corasick.html)
<!--- [https://compiler.club/pattern-matching-in-trees/](https://compiler.club/pattern-matching-in-trees/)-->
