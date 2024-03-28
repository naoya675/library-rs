---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dsl_2_f/src/main.rs
    title: verification/aizu-online-judge/dsl_2_f/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dsl_2_g/src/main.rs
    title: verification/aizu-online-judge/dsl_2_g/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dsl_2_h/src/main.rs
    title: verification/aizu-online-judge/dsl_2_h/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dsl_2_i/src/main.rs
    title: verification/aizu-online-judge/dsl_2_i/src/main.rs
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
  code: "#[derive(Debug, Clone)]\npub struct LazySegmentTree<T, F> {\n    tree: Vec<T>,\n\
    \    lazy: Vec<F>,\n    size: usize,\n    size_log: usize,\n    op: fn(T, T) ->\
    \ T, // evaluation funciton\n    e: T,              // identity element\n    mapping:\
    \ fn(F, T) -> T,\n    composition: fn(F, F) -> F,\n    id: F,\n}\n\nimpl<T: Copy,\
    \ F: Copy> LazySegmentTree<T, F> {\n    pub fn new(\n        n: usize,\n     \
    \   op: fn(T, T) -> T,\n        e: T,\n        mapping: fn(F, T) -> T,\n     \
    \   composition: fn(F, F) -> F,\n        id: F,\n    ) -> Self {\n        let\
    \ size = n.next_power_of_two();\n        let size_log = format!(\"{:b}\", size).to_string().len();\n\
    \        Self {\n            tree: vec![e; 2 * size],\n            lazy: vec![id;\
    \ 2 * size],\n            size,\n            size_log,\n            op,\n    \
    \        e,\n            mapping,\n            composition,\n            id,\n\
    \        }\n    }\n\n    pub fn build(&mut self, v: Vec<T>) {\n        assert!(v.len()\
    \ <= self.size);\n        for i in 0..v.len() {\n            self.set(i, v[i]);\n\
    \        }\n    }\n\n    pub fn set(&mut self, mut k: usize, x: T) {\n       \
    \ assert!(k < self.size);\n        k += self.size;\n        for i in (1..self.size_log\
    \ + 1).rev() {\n            self.push(k >> i);\n        }\n        self.tree[k]\
    \ = x;\n        for i in 1..self.size_log + 1 {\n            self.update(k >>\
    \ i);\n        }\n    }\n\n    pub fn get(&mut self, mut k: usize) -> T {\n  \
    \      assert!(k < self.size);\n        k += self.size;\n        for i in (1..self.size_log\
    \ + 1).rev() {\n            self.push(k >> i);\n        }\n        self.tree[k].clone()\n\
    \    }\n\n    pub fn prod(&mut self, mut l: usize, mut r: usize) -> T {\n    \
    \    assert!(l <= r && r <= self.size);\n        if l == r {\n            return\
    \ self.e;\n        }\n        l += self.size;\n        r += self.size;\n     \
    \   for i in (1..self.size_log + 1).rev() {\n            if ((l >> i) << i) !=\
    \ l {\n                self.push(l >> i);\n            }\n            if ((r >>\
    \ i) << i) != r {\n                self.push(r >> i);\n            }\n       \
    \ }\n        let mut l_res = self.e;\n        let mut r_res = self.e;\n      \
    \  while l < r {\n            if l % 2 == 1 {\n                l_res = (self.op)(l_res,\
    \ self.tree[l]);\n                l += 1;\n            }\n            if r % 2\
    \ == 1 {\n                r -= 1;\n                r_res = (self.op)(self.tree[r],\
    \ r_res);\n            }\n            l /= 2;\n            r /= 2;\n        }\n\
    \        (self.op)(l_res, r_res)\n    }\n\n    pub fn all_prod(&mut self) -> T\
    \ {\n        self.tree[1].clone()\n    }\n\n    // pub fn apply(&mut self, mut\
    \ k: usize, f: F) {\n    //     assert!(k < self.size);\n    //     k += self.size;\n\
    \    //     for i in (1..self.size_log + 1).rev() {\n    //         self.push(k\
    \ >> i);\n    //     }\n    //     self.tree[k] = (self.mapping)(f, self.tree[k]);\n\
    \    //     for i in 1..self.size_log + 1 {\n    //         self.update(k >> i);\n\
    \    //     }\n    // }\n\n    pub fn apply(&mut self, mut l: usize, mut r: usize,\
    \ f: F) {\n        assert!(l <= r && r <= self.size);\n        if l == r {\n \
    \           return;\n        }\n        l += self.size;\n        r += self.size;\n\
    \        for i in (1..self.size_log + 1).rev() {\n            if ((l >> i) <<\
    \ i) != l {\n                self.push(l >> i);\n            }\n            if\
    \ ((r >> i) << i) != r {\n                self.push((r - 1) >> i);\n         \
    \   }\n        }\n        let l2 = l;\n        let r2 = r;\n        while l <\
    \ r {\n            if l % 2 == 1 {\n                self.all_apply(l, f);\n  \
    \              l += 1;\n            }\n            if r % 2 == 1 {\n         \
    \       r -= 1;\n                self.all_apply(r, f);\n            }\n      \
    \      l /= 2;\n            r /= 2;\n        }\n        l = l2;\n        r = r2;\n\
    \        for i in 1..self.size_log + 1 {\n            if ((l >> i) << i) != l\
    \ {\n                self.update(l >> i);\n            }\n            if ((r >>\
    \ i) << i) != r {\n                self.update((r - 1) >> i);\n            }\n\
    \        }\n    }\n\n    fn all_apply(&mut self, k: usize, f: F) {\n        self.tree[k]\
    \ = (self.mapping)(f, self.tree[k]);\n        if k < self.size {\n           \
    \ self.lazy[k] = (self.composition)(f, self.lazy[k]);\n        }\n    }\n\n  \
    \  fn push(&mut self, k: usize) {\n        self.all_apply(k << 1 | 0, self.lazy[k]);\n\
    \        self.all_apply(k << 1 | 1, self.lazy[k]);\n        self.lazy[k] = self.id;\n\
    \    }\n\n    fn update(&mut self, k: usize) {\n        self.tree[k] = (self.op)(self.tree[k\
    \ << 1 | 0], self.tree[k << 1 | 1]);\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: data-structure/lazy-segment-tree/src/lib.rs
  requiredBy: []
  timestamp: '2024-03-22 18:55:17+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/dsl_2_h/src/main.rs
  - verification/aizu-online-judge/dsl_2_i/src/main.rs
  - verification/aizu-online-judge/dsl_2_g/src/main.rs
  - verification/aizu-online-judge/dsl_2_f/src/main.rs
documentation_of: data-structure/lazy-segment-tree/src/lib.rs
layout: document
title: Lazy Segment Tree
---

## Description

## Reference

[https://atcoder.github.io/ac-library/production/document_en/lazysegtree.html](https://atcoder.github.io/ac-library/production/document_en/lazysegtree.html)