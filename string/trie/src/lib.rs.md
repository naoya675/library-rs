---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "#[derive(Debug, Clone)]\npub struct Node {\n    next: Vec<Option<usize>>,\n\
    \    accept: Vec<usize>,\n    c: usize,\n    common: usize,\n}\n\nimpl Node {\n\
    \    pub fn new(c: usize, size: usize) -> Self {\n        Self {\n           \
    \ next: vec![None; size],\n            accept: vec![],\n            c,\n     \
    \       common: 0,\n        }\n    }\n}\n\n#[derive(Debug, Clone)]\npub struct\
    \ Trie {\n    nodes: Vec<Node>,\n    root: usize,\n    size: usize,\n    base:\
    \ char,\n}\n\nimpl Trie {\n    pub fn new(size: usize, base: char) -> Self {\n\
    \        let root = Node::new(size, size);\n        Self {\n            nodes:\
    \ vec![root],\n            root: 0,\n            size,\n            base,\n  \
    \      }\n    }\n\n    fn insert_internal(&mut self, word: &Vec<char>, word_id:\
    \ usize) {\n        let mut node_id = self.root;\n        for &w in word {\n \
    \           let c = (w as usize) - (self.base as usize);\n            if let Some(next_id)\
    \ = self.nodes[node_id].next[c] {\n                self.nodes[node_id].common\
    \ += 1;\n                node_id = next_id;\n            } else {\n          \
    \      let next_id = self.nodes.len();\n                self.nodes.push(Node::new(c,\
    \ self.size));\n                self.nodes[node_id].next[c] = Some(next_id);\n\
    \                self.nodes[node_id].common += 1;\n                node_id = next_id;\n\
    \            }\n        }\n        self.nodes[node_id].common += 1;\n        self.nodes[node_id].accept.push(word_id);\n\
    \    }\n\n    fn search_internal(&self, word: &Vec<char>, prefix: bool) -> (bool,\
    \ usize) {\n        let mut node_id = self.root;\n        for &w in word {\n \
    \           let c = (w as usize) - (self.base as usize);\n            if let Some(next_id)\
    \ = self.nodes[node_id].next[c] {\n                node_id = next_id;\n      \
    \      } else {\n                return (false, 0);\n            }\n        }\n\
    \        if prefix {\n            (true, self.nodes[node_id].common)\n       \
    \ } else {\n            let empty = !self.nodes[node_id].accept.is_empty();\n\
    \            (empty, self.nodes[node_id].common)\n        }\n    }\n\n    pub\
    \ fn insert(&mut self, word: &Vec<char>) {\n        self.insert_internal(word,\
    \ self.nodes[0].common);\n    }\n\n    pub fn search(&self, word: &Vec<char>)\
    \ -> (bool, usize) {\n        self.search_internal(word, false)\n    }\n\n   \
    \ pub fn search_prefix(&self, word: &Vec<char>) -> (bool, usize) {\n        self.search_internal(word,\
    \ true)\n    }\n\n    pub fn count(&self) -> usize {\n        self.nodes[self.root].common\n\
    \    }\n\n    pub fn size(&self) -> usize {\n        self.nodes.len()\n    }\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: string/trie/src/lib.rs
  requiredBy: []
  timestamp: '2025-04-18 21:05:47+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: string/trie/src/lib.rs
layout: document
title: Trie
---

## Description

### Usage

```
let mut trie = Trie::new(26, 'a');
let a = "hoge".chars().collect();
let b = "hogehoge".chars().collect();
let c = "hoge".chars().collect();
trie.insert(&a);
trie.insert(&b);
assert!(trie.search(&c) == (true, 2));
assert!(trie.search_prefix(&c) == (true, 2));
```

### Usage Example

- [https://atcoder.jp/contests/abc353/tasks/abc353_e](https://atcoder.jp/contests/abc353/tasks/abc353_e)
