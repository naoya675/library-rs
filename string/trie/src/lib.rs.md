---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: string/aho-corasick/src/lib.rs
    title: Aho-Corasick algorithm
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "#[derive(Debug, Clone)]\npub struct TrieNode {\n    next: Vec<Option<usize>>,\n\
    \    accept: Vec<usize>,\n    common: usize,\n    c: usize,\n}\n\nimpl TrieNode\
    \ {\n    pub fn new(c: usize, size: usize) -> Self {\n        Self {\n       \
    \     next: vec![None; size],\n            accept: vec![],\n            common:\
    \ 0,\n            c,\n        }\n    }\n}\n\n#[derive(Debug, Clone)]\npub struct\
    \ Trie {\n    nodes: Vec<TrieNode>,\n    root: usize,\n    size: usize,\n    base:\
    \ char,\n}\n\nimpl Trie {\n    pub fn new(size: usize, base: char) -> Self {\n\
    \        let root = TrieNode::new(size, size);\n        Self {\n            nodes:\
    \ vec![root],\n            root: 0,\n            size,\n            base,\n  \
    \      }\n    }\n\n    #[inline]\n    pub fn next(&self, node_id: usize, i: usize)\
    \ -> Option<usize> {\n        assert!(node_id < self.size());\n        assert!(i\
    \ < self.size);\n        self.nodes[node_id].next[i]\n    }\n\n    #[inline]\n\
    \    pub fn next_mut(&mut self, node_id: usize, i: usize) -> &mut Option<usize>\
    \ {\n        assert!(node_id < self.size());\n        assert!(i < self.size);\n\
    \        &mut self.nodes[node_id].next[i]\n    }\n\n    #[inline]\n    pub fn\
    \ accept(&self, node_id: usize) -> &Vec<usize> {\n        assert!(node_id < self.size());\n\
    \        &self.nodes[node_id].accept\n    }\n\n    #[inline]\n    pub fn accpet_mut(&mut\
    \ self, node_id: usize) -> &mut Vec<usize> {\n        assert!(node_id < self.size());\n\
    \        &mut self.nodes[node_id].accept\n    }\n\n    fn insert_internal(&mut\
    \ self, word: &[char], word_id: usize, node_id: usize, id: usize) {\n        if\
    \ word.len() == word_id {\n            self.nodes[node_id].accept.push(id);\n\
    \        } else {\n            let c = (word[word_id] as usize) - (self.base as\
    \ usize);\n            if let Some(next_id) = self.nodes[node_id].next[c] {\n\
    \                self.nodes[node_id].common += 1;\n                self.insert_internal(word,\
    \ word_id + 1, next_id, id);\n            } else {\n                let next_id\
    \ = self.nodes.len();\n                self.nodes.push(TrieNode::new(c, self.size));\n\
    \                self.nodes[node_id].next[c] = Some(next_id);\n              \
    \  self.nodes[node_id].common += 1;\n                self.insert_internal(word,\
    \ word_id + 1, next_id, id);\n            }\n        }\n    }\n\n    pub fn insert(&mut\
    \ self, word: &[char]) {\n        self.insert_internal(word, 0, 0, self.nodes[0].common);\n\
    \    }\n\n    /*\n     * Non-recursive\n    fn insert_internal(&mut self, word:\
    \ &[char], word_id: usize) {\n        let mut node_id = 0;\n        for &w in\
    \ word {\n            let c = (w as usize) - (self.base as usize);\n         \
    \   if let Some(next_id) = self.nodes[node_id].next[c] {\n                self.nodes[node_id].common\
    \ += 1;\n                node_id = next_id;\n            } else {\n          \
    \      let next_id = self.nodes.len();\n                self.nodes.push(TrieNode::new(c,\
    \ self.size));\n                self.nodes[node_id].next[c] = Some(next_id);\n\
    \                self.nodes[node_id].common += 1;\n                node_id = next_id;\n\
    \            }\n        }\n        self.nodes[node_id].accept.push(word_id);\n\
    \    }\n\n    pub fn insert(&mut self, word: &[char]) {\n        self.insert_internal(word,\
    \ self.nodes[0].common);\n    }\n     */\n\n    fn search_internal(&self, word:\
    \ &[char], word_id: usize, node_id: usize, prefix: bool) -> bool {\n        if\
    \ word.len() == word_id {\n            return if prefix { true } else { !self.nodes[node_id].accept.is_empty()\
    \ };\n        }\n        let c = (word[word_id] as usize) - (self.base as usize);\n\
    \        if let Some(next_id) = self.nodes[node_id].next[c] {\n            self.search_internal(word,\
    \ word_id + 1, next_id, prefix)\n        } else {\n            false\n       \
    \ }\n    }\n\n    pub fn search(&self, word: &[char]) -> bool {\n        self.search_internal(word,\
    \ 0, 0, false)\n    }\n\n    pub fn search_prefix(&self, word: &[char]) -> bool\
    \ {\n        self.search_internal(word, 0, 0, true)\n    }\n\n    /*\n     * Non-recursive\n\
    \    fn search_internal(&self, word: &[char], prefix: bool) -> bool {\n      \
    \  let mut node_id = self.root;\n        for &w in word {\n            let c =\
    \ (w as usize) - (self.base as usize);\n            if let Some(next_id) = self.nodes[node_id].next[c]\
    \ {\n                node_id = next_id;\n            } else {\n              \
    \  return false;\n            }\n        }\n        return if prefix { true }\
    \ else { !self.nodes[node_id].accept.is_empty() };\n    }\n\n    pub fn search(&self,\
    \ word: &[char]) -> bool {\n        self.search_internal(word, false)\n    }\n\
    \n    pub fn search_prefix(&self, word: &[char]) -> bool {\n        self.search_internal(word,\
    \ true)\n    }\n     */\n\n    fn query_internal<F>(&self, word: &[char], mut\
    \ f: F, word_id: usize, node_id: usize)\n    where\n        F: FnMut(usize),\n\
    \    {\n        for &index in &self.nodes[node_id].accept {\n            f(index);\n\
    \        }\n        if word.len() == word_id {\n            return;\n        }\
    \ else {\n            let c = (word[word_id] as usize) - (self.base as usize);\n\
    \            if let Some(next_id) = self.nodes[node_id].next[c] {\n          \
    \      self.query_internal(word, f, word_id + 1, next_id)\n            }\n   \
    \     }\n    }\n\n    pub fn query<F>(&self, word: &[char], f: F)\n    where\n\
    \        F: FnMut(usize),\n    {\n        self.query_internal(word, f, 0, 0);\n\
    \    }\n\n    pub fn count(&self) -> usize {\n        self.nodes[self.root].common\n\
    \    }\n\n    pub fn size(&self) -> usize {\n        self.nodes.len()\n    }\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: string/trie/src/lib.rs
  requiredBy:
  - string/aho-corasick/src/lib.rs
  timestamp: '2026-01-01 00:11:18+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: string/trie/src/lib.rs
layout: document
redirect_from:
- /library/string/trie/src/lib.rs
- /library/string/trie/src/lib.rs.html
title: string/trie/src/lib.rs
---
