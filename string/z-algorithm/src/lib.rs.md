---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/zalgorithm/src/main.rs
    title: verification/library-checker/zalgorithm/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://atcoder.github.io/ac-library/production/document_en/string.html
    - https://heno239.hatenablog.com/entry/2020/07/07/140651
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! https://atcoder.github.io/ac-library/production/document_en/string.html\n\
    //! https://heno239.hatenablog.com/entry/2020/07/07/140651\n\nuse std::collections::VecDeque;\n\
    \n#[derive(Debug, Clone)]\npub struct ZAlgorithm<T> {\n    s: Vec<T>,\n    z:\
    \ Vec<usize>,\n    delete: Vec<usize>,\n    delete_memo: Vec<Vec<usize>>,\n  \
    \  cur: VecDeque<usize>,\n}\n\nimpl<T: Copy + PartialEq> ZAlgorithm<T> {\n   \
    \ pub fn new() -> Self {\n        Self {\n            s: vec![],\n           \
    \ z: vec![],\n            delete: vec![],\n            delete_memo: vec![vec![]],\n\
    \            cur: VecDeque::new(),\n        }\n    }\n\n    pub fn build(&mut\
    \ self, s: &Vec<T>) {\n        for i in 0..s.len() {\n            self.set(s[i]);\n\
    \        }\n    }\n\n    fn delete(&mut self, q: usize, len: usize) {\n      \
    \  self.delete[q] = 1;\n        self.delete_memo[len].push(q);\n        self.z[q]\
    \ = len - q - 1;\n    }\n\n    pub fn set(&mut self, c: T) {\n        self.s.push(c);\n\
    \        self.z.push(0);\n        self.delete.push(0);\n        self.delete_memo.push(vec![]);\n\
    \        self.z[0] += 1;\n\n        let len = self.s.len();\n        if len ==\
    \ 1 {\n            return;\n        }\n        if self.s[0] == c {\n         \
    \   self.cur.push_back(len - 1);\n        } else {\n            self.delete[len\
    \ - 1] = 1;\n        }\n\n        while let Some(&q) = self.cur.front() {\n  \
    \          if self.delete[q] != 0 {\n                self.cur.pop_front();\n \
    \               continue;\n            }\n            if self.s[len - 1 - q] ==\
    \ *self.s.last().unwrap() {\n                break;\n            } else {\n  \
    \              self.delete(q, len);\n                self.cur.pop_front();\n \
    \           }\n        }\n\n        if let Some(&q) = self.cur.front() {\n   \
    \         for p in self.delete_memo[len - q].clone() {\n                self.delete(p\
    \ + q, len);\n            }\n        }\n    }\n\n    pub fn get(&self) -> Vec<usize>\
    \ {\n        let mut res = vec![0; self.s.len()];\n        for i in 0..self.s.len()\
    \ {\n            res[i] = self.internal_get(i);\n        }\n        res\n    }\n\
    \n    fn internal_get(&self, k: usize) -> usize {\n        if self.delete[k] !=\
    \ 0 {\n            self.z[k]\n        } else {\n            self.s.len() - k\n\
    \        }\n    }\n}\n\npub fn z_algorithm<T: Copy + PartialEq>(s: &Vec<T>) ->\
    \ Vec<usize> {\n    if s.len() == 0 {\n        return vec![];\n    }\n    let\
    \ mut res = vec![0; s.len()];\n    let mut i = 1;\n    let mut j = 0;\n    while\
    \ i < s.len() {\n        while i + j < s.len() && s[i + j] == s[j] {\n       \
    \     j += 1;\n        }\n        res[i] = j;\n        if j == 0 {\n         \
    \   i += 1;\n            continue;\n        }\n        let mut k = 1;\n      \
    \  while i + k < s.len() && k + res[k] < j {\n            res[i + k] = res[k];\n\
    \            k += 1;\n        }\n        i += k;\n        j -= k;\n    }\n   \
    \ res[0] = s.len();\n    res\n}\n\n/*\npub fn z_algorithm<T: Copy + PartialEq>(s:\
    \ &Vec<T>) -> Vec<usize> {\n    if s.len() == 0 {\n        return vec![];\n  \
    \  }\n    let mut res = vec![0; s.len()];\n    let mut i = 1;\n    let mut j =\
    \ 0;\n    while i < s.len() {\n        res[i] = if res[j] + j <= i { 0 } else\
    \ { std::cmp::min(res[j] + j - i, res[i - j]) };\n        while i + res[i] < s.len()\
    \ && s[res[i]] == s[i + res[i]] {\n            res[i] += 1;\n        }\n     \
    \   if res[j] + j < res[i] + i {\n            j = i;\n        }\n        i +=\
    \ 1;\n    }\n    res[0] = s.len();\n    res\n}\n*/\n"
  dependsOn: []
  isVerificationFile: false
  path: string/z-algorithm/src/lib.rs
  requiredBy: []
  timestamp: '2025-06-07 00:31:50+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/library-checker/zalgorithm/src/main.rs
documentation_of: string/z-algorithm/src/lib.rs
layout: document
title: Z Algorithm
---

## Description
