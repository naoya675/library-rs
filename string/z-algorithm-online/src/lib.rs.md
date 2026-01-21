---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':x:'
    path: verification/library-checker/zalgorithm-online/src/main.rs
    title: verification/library-checker/zalgorithm-online/src/main.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::collections::VecDeque;\n\n#[derive(Debug, Clone)]\npub struct ZAlgorithm<T>\
    \ {\n    s: Vec<T>,\n    z: Vec<usize>,\n    delete: Vec<usize>,\n    delete_memo:\
    \ Vec<Vec<usize>>,\n    cur: VecDeque<usize>,\n}\n\nimpl<T: Copy + PartialEq>\
    \ ZAlgorithm<T> {\n    pub fn new() -> Self {\n        Self {\n            s:\
    \ vec![],\n            z: vec![],\n            delete: vec![],\n            delete_memo:\
    \ vec![vec![]],\n            cur: VecDeque::new(),\n        }\n    }\n\n    pub\
    \ fn build(&mut self, s: &[T]) {\n        for i in 0..s.len() {\n            self.set(s[i]);\n\
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
    \n    fn internal_get(&self, k: usize) -> usize {\n        assert!(k < self.s.len());\n\
    \        if self.delete[k] != 0 { self.z[k] } else { self.s.len() - k }\n    }\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: string/z-algorithm-online/src/lib.rs
  requiredBy: []
  timestamp: '2025-12-31 23:53:43+09:00'
  verificationStatus: LIBRARY_ALL_WA
  verifiedWith:
  - verification/library-checker/zalgorithm-online/src/main.rs
documentation_of: string/z-algorithm-online/src/lib.rs
layout: document
title: Z Algorithm (Online)
---

## Description

## Reference
- [https://heno239.hatenablog.com/entry/2020/07/07/140651](https://heno239.hatenablog.com/entry/2020/07/07/140651)
