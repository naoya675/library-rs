---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: string/rolling-hash-segment-tree/src/lib.rs
    title: Rolling Hash + Segment Tree
  - icon: ':question:'
    path: tree/euler-tour/src/lib.rs
    title: Euler Tour
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dsl_2_a/src/main.rs
    title: verification/aizu-online-judge/dsl_2_a/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dsl_2_b/src/main.rs
    title: verification/aizu-online-judge/dsl_2_b/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/point_set_range_composite/src/main.rs
    title: verification/library-checker/point_set_range_composite/src/main.rs
  - icon: ':x:'
    path: verification/library-checker/vertex_set_path_composite/src/main.rs
    title: verification/library-checker/vertex_set_path_composite/src/main.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes:
    links:
    - https://atcoder.github.io/ac-library/production/document_en/segtree.html
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// reference: https://atcoder.github.io/ac-library/production/document_en/segtree.html\n\
    \n#[derive(Debug, Clone)]\npub struct SegmentTree<T> {\n    tree: Vec<T>,\n  \
    \  size: usize,\n    size_log: usize,\n    // Monoids: operation (associativity)\
    \ + identity element\n    op: fn(T, T) -> T,\n    e: T,\n    n: usize,\n}\n\n\
    impl<T: Copy> SegmentTree<T> {\n    pub fn new(n: usize, op: fn(T, T) -> T, e:\
    \ T) -> Self {\n        let size = n.next_power_of_two();\n        let size_log\
    \ = (size.ilog2() + 1) as usize;\n        Self {\n            tree: vec![e; 2\
    \ * size],\n            size,\n            size_log,\n            op,\n      \
    \      e,\n            n,\n        }\n    }\n\n    pub fn build(&mut self, vec:\
    \ Vec<T>) {\n        assert!(vec.len() == self.n);\n        for k in 0..self.n\
    \ {\n            self.tree[k + self.size] = vec[k];\n        }\n        for k\
    \ in (0..self.size).rev() {\n            self.update(k);\n        }\n    }\n\n\
    \    pub fn set(&mut self, mut k: usize, x: T) {\n        assert!(k < self.n);\n\
    \        k += self.size;\n        self.tree[k] = x;\n        for i in 1..self.size_log\
    \ + 1 {\n            self.update(k >> i);\n        }\n        /*\n        while\
    \ k > 0 {\n            k >>= 1;\n            self.update(k);\n        }\n    \
    \    */\n    }\n\n    pub fn get(&mut self, mut k: usize) -> T {\n        assert!(k\
    \ < self.n);\n        k += self.size;\n        self.tree[k].clone()\n    }\n\n\
    \    pub fn prod(&mut self, mut l: usize, mut r: usize) -> T {\n        assert!(l\
    \ <= r && r <= self.n);\n        if l == r {\n            return self.e;\n   \
    \     }\n        l += self.size;\n        r += self.size;\n        let mut l_res\
    \ = self.e;\n        let mut r_res = self.e;\n        while l < r {\n        \
    \    if l & 1 != 0 {\n                l_res = (self.op)(l_res, self.tree[l]);\n\
    \                l += 1;\n            }\n            if r & 1 != 0 {\n       \
    \         r -= 1;\n                r_res = (self.op)(self.tree[r], r_res);\n \
    \           }\n            l >>= 1;\n            r >>= 1;\n        }\n       \
    \ (self.op)(l_res, r_res)\n    }\n\n    pub fn all_prod(&mut self) -> T {\n  \
    \      self.tree[1].clone()\n    }\n\n    /*\n    pub fn apply(&mut self, mut\
    \ k: usize, x: T) {\n        assert!(k < self.n);\n        k += self.size;\n \
    \       self.tree[k] = (self.op)(self.tree[k], x);\n        while k > 0 {\n  \
    \          k >>= 1;\n            self.update(k);\n        }\n    }\n    */\n\n\
    \    pub fn apply(&mut self, k: usize, x: T) {\n        self.apply_with(k, x,\
    \ self.op)\n    }\n\n    pub fn apply_with<F>(&mut self, mut k: usize, x: T, f:\
    \ F)\n    where\n        F: Fn(T, T) -> T,\n    {\n        assert!(k < self.n);\n\
    \        k += self.size;\n        self.tree[k] = f(self.tree[k], x);\n       \
    \ while k > 0 {\n            k >>= 1;\n            self.update(k);\n        }\n\
    \    }\n\n    pub fn max_right<F>(&mut self, mut l: usize, f: F) -> usize\n  \
    \  where\n        F: Fn(T) -> bool,\n    {\n        assert!(l <= self.n);\n  \
    \      assert!(f(self.e));\n        if l == self.n {\n            return self.n;\n\
    \        }\n        l += self.size;\n        let mut res = self.e;\n        while\
    \ {\n            while l % 2 == 0 {\n                l >>= 1;\n            }\n\
    \            if !f((self.op)(res, self.tree[l])) {\n                while l <\
    \ self.size {\n                    l = 2 * l;\n                    if f((self.op)(res,\
    \ self.tree[l])) {\n                        res = (self.op)(res, self.tree[l]);\n\
    \                        l += 1;\n                    }\n                }\n \
    \               return l - self.size;\n            }\n            res = (self.op)(res,\
    \ self.tree[l]);\n            l += 1;\n            l & l.wrapping_neg() != l\n\
    \        } {}\n        self.n\n    }\n\n    pub fn min_left<F>(&mut self, mut\
    \ r: usize, f: F) -> usize\n    where\n        F: Fn(T) -> bool,\n    {\n    \
    \    assert!(r <= self.n);\n        assert!(f(self.e));\n        if r == 0 {\n\
    \            return 0;\n        }\n        r += self.size;\n        let mut res\
    \ = self.e;\n        while {\n            r -= 1;\n            while r > 1 &&\
    \ r % 2 != 0 {\n                r >>= 1;\n            }\n            if !f((self.op)(self.tree[r],\
    \ res)) {\n                while r < self.size {\n                    r = 2 *\
    \ r + 1;\n                    if f((self.op)(self.tree[r], res)) {\n         \
    \               res = (self.op)(self.tree[r], res);\n                        r\
    \ -= 1;\n                    }\n                }\n                return r +\
    \ 1 - self.size;\n            }\n            res = (self.op)(self.tree[r], res);\n\
    \            r & r.wrapping_neg() != r\n        } {}\n        0\n    }\n\n   \
    \ fn update(&mut self, k: usize) {\n        self.tree[k] = (self.op)(self.tree[k\
    \ << 1 | 0], self.tree[k << 1 | 1]);\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: data-structure/segment-tree/src/lib.rs
  requiredBy:
  - tree/euler-tour/src/lib.rs
  - string/rolling-hash-segment-tree/src/lib.rs
  timestamp: '2025-08-21 20:46:40+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - verification/aizu-online-judge/dsl_2_a/src/main.rs
  - verification/aizu-online-judge/dsl_2_b/src/main.rs
  - verification/library-checker/vertex_set_path_composite/src/main.rs
  - verification/library-checker/point_set_range_composite/src/main.rs
documentation_of: data-structure/segment-tree/src/lib.rs
layout: document
title: Segment Tree
---

## Description
