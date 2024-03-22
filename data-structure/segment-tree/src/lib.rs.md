---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dsl_2_a/src/main.rs
    title: verification/aizu-online-judge/dsl_2_a/src/main.rs
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
  code: "#[derive(Debug, Clone)]\npub struct SegmentTree<T> {\n    tree: Vec<T>,\n\
    \    size: usize,\n    op: fn(T, T) -> T, // evaluation funciton\n    e: T,  \
    \            // identity element\n}\n\nimpl<T: Copy> SegmentTree<T> {\n    pub\
    \ fn new(n: usize, op: fn(T, T) -> T, e: T) -> Self {\n        let size = n.next_power_of_two();\n\
    \        Self {\n            tree: vec![e; 2 * size],\n            size,\n   \
    \         op,\n            e,\n        }\n    }\n\n    pub fn build(&mut self,\
    \ v: Vec<T>) {\n        for i in 0..v.len() {\n            self.set(i, v[i]);\n\
    \        }\n    }\n\n    pub fn set(&mut self, mut k: usize, x: T) {\n       \
    \ assert!(k < self.size);\n        k += self.size;\n        self.tree[k] = x;\n\
    \        while k > 0 {\n            k /= 2;\n            self.update(k);\n   \
    \     }\n    }\n\n    pub fn get(&mut self, mut k: usize) -> T {\n        assert!(k\
    \ < self.size);\n        k += self.size;\n        self.tree[k].clone()\n    }\n\
    \n    pub fn prod(&mut self, mut l: usize, mut r: usize) -> T {\n        if l\
    \ == r {\n            return self.e;\n        }\n        l += self.size;\n   \
    \     r += self.size;\n        let mut l_res = self.e;\n        let mut r_res\
    \ = self.e;\n        while l < r {\n            if l % 2 == 1 {\n            \
    \    l_res = (self.op)(l_res, self.tree[l]);\n                l += 1;\n      \
    \      }\n            if r % 2 == 1 {\n                r -= 1;\n             \
    \   r_res = (self.op)(self.tree[r], r_res);\n            }\n            l /= 2;\n\
    \            r /= 2;\n        }\n        (self.op)(l_res, r_res)\n    }\n\n  \
    \  pub fn all_prod(&mut self) -> T {\n        self.tree[1].clone()\n    }\n\n\
    \    pub fn apply(&mut self, mut k: usize, x: T) {\n        assert!(k < self.size);\n\
    \        k += self.size;\n        self.tree[k] = (self.op)(self.tree[k], x);\n\
    \        while k > 0 {\n            k /= 2;\n            self.tree[k] = (self.op)(self.tree[k\
    \ << 1 | 0], self.tree[k << 1 | 1]);\n        }\n    }\n\n    fn update(&mut self,\
    \ k: usize) {\n        self.tree[k] = (self.op)(self.tree[k << 1 | 0], self.tree[k\
    \ << 1 | 1]);\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: data-structure/segment-tree/src/lib.rs
  requiredBy: []
  timestamp: '2024-03-22 18:42:01+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/dsl_2_a/src/main.rs
  - verification/aizu-online-judge/dsl_2_b/src/main.rs
documentation_of: data-structure/segment-tree/src/lib.rs
layout: document
title: Segment Tree
---

## Description
