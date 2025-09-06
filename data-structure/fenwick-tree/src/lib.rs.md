---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':x:'
    path: verification/library-checker/vertex_add_path_sum/src/main.rs
    title: verification/library-checker/vertex_add_path_sum/src/main.rs
  - icon: ':x:'
    path: verification/library-checker/vertex_add_subtree_sum/src/main.rs
    title: verification/library-checker/vertex_add_subtree_sum/src/main.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
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
    \n#[derive(Debug, Clone)]\npub struct FenwickTree<T> {\n    tree: Vec<T>,\n  \
    \  size: usize,\n}\n\nimpl<T: Copy + PartialOrd + Ord + Default> FenwickTree<T>\n\
    where\n    T: std::ops::Add<T, Output = T>,\n    T: std::ops::AddAssign,\n   \
    \ T: std::ops::Sub<T, Output = T>,\n    T: std::ops::SubAssign,\n{\n    pub fn\
    \ new(n: usize) -> Self {\n        let size = n;\n        Self {\n           \
    \ tree: vec![T::default(); size + 1],\n            size,\n        }\n    }\n\n\
    \    pub fn add(&mut self, mut k: usize, x: T) {\n        assert!(k < self.size);\n\
    \        k += 1;\n        while k <= self.size {\n            self.tree[k] +=\
    \ x;\n            k += k & k.wrapping_neg();\n        }\n    }\n\n    pub fn sum(&self,\
    \ l: usize, r: usize) -> T {\n        assert!(l <= r && r <= self.size);\n   \
    \     self.prefix_sum(r) - self.prefix_sum(l)\n    }\n\n    fn prefix_sum(&self,\
    \ mut r: usize) -> T {\n        let mut s = T::default();\n        while r > 0\
    \ {\n            s += self.tree[r];\n            r -= r & r.wrapping_neg();\n\
    \        }\n        s\n    }\n\n    pub fn lower_bound(&self, mut x: T) -> usize\
    \ {\n        let mut s = 0;\n        let mut k = self.size.next_power_of_two();\n\
    \        while k > 0 {\n            if s + k <= self.size && self.tree[s + k]\
    \ < x {\n                x -= self.tree[s + k];\n                s += k;\n   \
    \         }\n            k >>= 1;\n        }\n        s\n    }\n\n    pub fn upper_bound(&self,\
    \ mut x: T) -> usize {\n        let mut s = 0;\n        let mut k = self.size.next_power_of_two();\n\
    \        while k > 0 {\n            if s + k <= self.size && self.tree[s + k]\
    \ <= x {\n                x -= self.tree[s + k];\n                s += k;\n  \
    \          }\n            k >>= 1;\n        }\n        s\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: data-structure/fenwick-tree/src/lib.rs
  requiredBy: []
  timestamp: '2025-08-21 20:46:40+09:00'
  verificationStatus: LIBRARY_ALL_WA
  verifiedWith:
  - verification/library-checker/vertex_add_subtree_sum/src/main.rs
  - verification/library-checker/vertex_add_path_sum/src/main.rs
documentation_of: data-structure/fenwick-tree/src/lib.rs
layout: document
title: Fenwick Tree
---

## Description

## Reference
- [https://ikatakos.com/pot/programming_algorithm/data_structure/binary_indexed_tree](https://ikatakos.com/pot/programming_algorithm/data_structure/binary_indexed_tree)
- [https://qiita.com/Stakumi/items/b7593a99908c98cfe6d0](https://qiita.com/Stakumi/items/b7593a99908c98cfe6d0)
