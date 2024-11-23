---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: algebra/internal-trait/src/lib.rs
    title: algebra/internal-trait/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dsl_2_b/src/main.rs
    title: verification/aizu-online-judge/dsl_2_b/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use internal_trait::Zero;\n\n#[derive(Debug, Clone)]\npub struct FenwickTree<T>\
    \ {\n    tree: Vec<T>,\n    size: usize,\n}\n\nimpl<T: Copy + Zero> FenwickTree<T>\n\
    where\n    T: std::ops::Neg<Output = T>,\n    T: std::ops::Add<T, Output = T>,\n\
    \    T: std::ops::AddAssign,\n    T: std::ops::Sub<T, Output = T>,\n    T: std::ops::SubAssign,\n\
    {\n    pub fn new(n: usize) -> Self {\n        let size = n;\n        Self {\n\
    \            tree: vec![T::zero(); size],\n            size,\n        }\n    }\n\
    \n    pub fn build(&mut self, v: Vec<T>) {\n        assert!(v.len() <= self.size);\n\
    \        for i in 0..v.len() {\n            self.add(i, v[i]);\n        }\n  \
    \  }\n\n    pub fn add(&mut self, mut k: usize, x: T) {\n        assert!(k < self.size);\n\
    \        k += 1;\n        while k <= self.size {\n            self.tree[k - 1]\
    \ += x;\n            k += k & k.wrapping_neg();\n        }\n    }\n\n    pub fn\
    \ sum(&mut self, l: usize, r: usize) -> T {\n        assert!(l <= r && r <= self.size);\n\
    \        self.prefix_sum(r) - self.prefix_sum(l)\n    }\n\n    fn prefix_sum(&mut\
    \ self, mut r: usize) -> T {\n        let mut s = T::zero();\n        while r\
    \ > 0 {\n            s += self.tree[r - 1];\n            r -= r & r.wrapping_neg();\n\
    \        }\n        s\n    }\n}\n"
  dependsOn:
  - algebra/internal-trait/src/lib.rs
  isVerificationFile: false
  path: data-structure/fenwick-tree/src/lib.rs
  requiredBy: []
  timestamp: '2024-09-23 04:03:48+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/dsl_2_b/src/main.rs
documentation_of: data-structure/fenwick-tree/src/lib.rs
layout: document
title: Fenwick Tree
---

## Description
