---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/grl_5_d/src/main.rs
    title: verification/aizu-online-judge/grl_5_d/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/grl_5_e/src/main.rs
    title: verification/aizu-online-judge/grl_5_e/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://atcoder.github.io/ac-library/production/document_en/fenwicktree.html
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// reference: https://atcoder.github.io/ac-library/production/document_en/fenwicktree.html\n\
    \n#[derive(Debug, Clone)]\npub struct FenwickTreeAbstract<T> {\n    tree: Vec<T>,\n\
    \    size: usize,\n    // Abelian Group: operation (associative, commutative)\
    \ + identity element + inverse element\n    op: fn(T, T) -> T,\n    e: T,\n  \
    \  inv: fn(T) -> T,\n}\n\nimpl<T: Copy> FenwickTreeAbstract<T>\nwhere\n    T:\
    \ std::ops::Add<T, Output = T>,\n    T: std::ops::AddAssign,\n    T: std::ops::Sub<T,\
    \ Output = T>,\n    T: std::ops::SubAssign,\n{\n    pub fn new(n: usize, op: fn(T,\
    \ T) -> T, e: T, inv: fn(T) -> T) -> Self {\n        let size = n;\n        Self\
    \ {\n            tree: vec![e; size + 1],\n            size,\n            op,\n\
    \            e,\n            inv,\n        }\n    }\n\n    // apply\n    pub fn\
    \ add(&mut self, mut k: usize, x: T) {\n        assert!(k < self.size);\n    \
    \    k += 1;\n        while k <= self.size {\n            self.tree[k] = (self.op)(self.tree[k],\
    \ x);\n            k += k & k.wrapping_neg();\n        }\n    }\n\n    // prod\n\
    \    pub fn sum(&mut self, l: usize, r: usize) -> T {\n        assert!(l <= r\
    \ && r <= self.size);\n        (self.op)(self.prefix_sum(r), (self.inv)(self.prefix_sum(l)))\n\
    \    }\n\n    pub fn prefix_sum(&mut self, mut r: usize) -> T {\n        let mut\
    \ s = self.e;\n        while r > 0 {\n            s = (self.op)(s, self.tree[r]);\n\
    \            r -= r & r.wrapping_neg();\n        }\n        s\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: data-structure/fenwick-tree-abstract/src/lib.rs
  requiredBy: []
  timestamp: '2025-06-21 17:54:09+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/grl_5_d/src/main.rs
  - verification/aizu-online-judge/grl_5_e/src/main.rs
documentation_of: data-structure/fenwick-tree-abstract/src/lib.rs
layout: document
title: Fenwick Tree (Abstract)
---

## Description
