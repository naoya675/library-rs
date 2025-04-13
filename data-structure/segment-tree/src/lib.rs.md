---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: string/rolling-hash-segment-tree/src/lib.rs
    title: Rolling Hash + Segment Tree
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dsl_2_a/src/main.rs
    title: verification/aizu-online-judge/dsl_2_a/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://atcoder.jp/contests/abc223/editorial/2774
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "#[derive(Debug, Clone)]\npub struct SegmentTree<T> {\n    tree: Vec<T>,\n\
    \    size: usize,\n    size_log: usize,\n    op: fn(T, T) -> T, // evaluation\
    \ funciton\n    e: T,              // identity element\n    n: usize,\n}\n\nimpl<T:\
    \ Copy> SegmentTree<T> {\n    pub fn new(n: usize, op: fn(T, T) -> T, e: T) ->\
    \ Self {\n        let size = n.next_power_of_two();\n        let size_log = (size.ilog2()\
    \ + 1) as usize;\n        Self {\n            tree: vec![e; 2 * size],\n     \
    \       size,\n            size_log,\n            op,\n            e,\n      \
    \      n,\n        }\n    }\n\n    pub fn build(&mut self, v: Vec<T>) {\n    \
    \    assert!(v.len() <= self.n);\n        for i in 0..v.len() {\n            self.set(i,\
    \ v[i]);\n        }\n    }\n\n    pub fn set(&mut self, mut k: usize, x: T) {\n\
    \        assert!(k < self.n);\n        k += self.size;\n        self.tree[k] =\
    \ x;\n        for i in 1..self.size_log + 1 {\n            self.update(k >> i);\n\
    \        }\n        // while k > 0 {\n        //     k >>= 1;\n        //    \
    \ self.update(k);\n        // }\n    }\n\n    pub fn get(&mut self, mut k: usize)\
    \ -> T {\n        assert!(k < self.n);\n        k += self.size;\n        self.tree[k].clone()\n\
    \    }\n\n    pub fn prod(&mut self, mut l: usize, mut r: usize) -> T {\n    \
    \    assert!(l <= r && r <= self.n);\n        if l == r {\n            return\
    \ self.e;\n        }\n        l += self.size;\n        r += self.size;\n     \
    \   let mut l_res = self.e;\n        let mut r_res = self.e;\n        while l\
    \ < r {\n            if l & 1 != 0 {\n                l_res = (self.op)(l_res,\
    \ self.tree[l]);\n                l += 1;\n            }\n            if r & 1\
    \ != 0 {\n                r -= 1;\n                r_res = (self.op)(self.tree[r],\
    \ r_res);\n            }\n            l >>= 1;\n            r >>= 1;\n       \
    \ }\n        (self.op)(l_res, r_res)\n    }\n\n    pub fn all_prod(&mut self)\
    \ -> T {\n        self.tree[1].clone()\n    }\n\n    pub fn apply(&mut self, mut\
    \ k: usize, x: T) {\n        assert!(k < self.n);\n        k += self.size;\n \
    \       self.tree[k] = (self.op)(self.tree[k], x);\n        while k > 0 {\n  \
    \          k >>= 1;\n            self.update(k);\n        }\n    }\n\n    pub\
    \ fn max_right<F>(&mut self, mut l: usize, f: F) -> usize\n    where\n       \
    \ F: Fn(T) -> bool,\n    {\n        assert!(l <= self.n);\n        assert!(f(self.e));\n\
    \        if l == self.n {\n            return self.n;\n        }\n        l +=\
    \ self.size;\n        let mut res = self.e;\n        while {\n            while\
    \ l % 2 == 0 {\n                l >>= 1;\n            }\n            if !f((self.op)(res,\
    \ self.tree[l])) {\n                while l < self.size {\n                  \
    \  l = 2 * l;\n                    if f((self.op)(res, self.tree[l])) {\n    \
    \                    res = (self.op)(res, self.tree[l]);\n                   \
    \     l += 1;\n                    }\n                }\n                return\
    \ l - self.size;\n            }\n            res = (self.op)(res, self.tree[l]);\n\
    \            l += 1;\n            l & l.wrapping_neg() != l\n        } {}\n  \
    \      self.n\n    }\n\n    pub fn min_left<F>(&mut self, mut r: usize, f: F)\
    \ -> usize\n    where\n        F: Fn(T) -> bool,\n    {\n        assert!(r <=\
    \ self.n);\n        assert!(f(self.e));\n        if r == 0 {\n            return\
    \ 0;\n        }\n        r += self.size;\n        let mut res = self.e;\n    \
    \    while {\n            r -= 1;\n            while r > 1 && r % 2 != 0 {\n \
    \               r >>= 1;\n            }\n            if !f((self.op)(self.tree[r],\
    \ res)) {\n                while r < self.size {\n                    r = 2 *\
    \ r + 1;\n                    if f((self.op)(self.tree[r], res)) {\n         \
    \               res = (self.op)(self.tree[r], res);\n                        r\
    \ -= 1;\n                    }\n                }\n                return r +\
    \ 1 - self.size;\n            }\n            res = (self.op)(self.tree[r], res);\n\
    \            r & r.wrapping_neg() != r\n        } {}\n        0\n    }\n\n   \
    \ fn update(&mut self, k: usize) {\n        self.tree[k] = (self.op)(self.tree[k\
    \ << 1 | 0], self.tree[k << 1 | 1]);\n    }\n}\n\npub struct RangeMinimumQuery;\n\
    \nimpl RangeMinimumQuery {\n    pub fn new(n: usize) -> SegmentTree<i64> {\n \
    \       SegmentTree::new(n, |a, b| std::cmp::min(a, b), i64::MAX)\n    }\n}\n\n\
    pub struct RangeMaximumQuery;\n\nimpl RangeMaximumQuery {\n    pub fn new(n: usize)\
    \ -> SegmentTree<i64> {\n        SegmentTree::new(n, |a, b| std::cmp::max(a, b),\
    \ i64::MIN)\n    }\n}\n\npub struct RangeSumQuery;\n\nimpl RangeSumQuery {\n \
    \   pub fn new(n: usize) -> SegmentTree<i64> {\n        SegmentTree::new(n, |a,\
    \ b| a + b, 0)\n    }\n}\n\npub struct ParenthesisCheckQuery;\n\nimpl ParenthesisCheckQuery\
    \ {\n    pub fn new(n: usize) -> SegmentTree<(i64, i64)> {\n        SegmentTree::new(\n\
    \            n,\n            |a, b| {\n                (\n                   \
    \ a.0 + std::cmp::max(b.0 - a.1, 0),\n                    std::cmp::max(a.1 -\
    \ b.0, 0) + b.1,\n                )\n            },\n            (0, 0),\n   \
    \     )\n    }\n}\n\n// reference: https://atcoder.jp/contests/abc223/editorial/2774\n\
    // '(' == (0, 1)\n// ')' == (1, 0)\n"
  dependsOn: []
  isVerificationFile: false
  path: data-structure/segment-tree/src/lib.rs
  requiredBy:
  - string/rolling-hash-segment-tree/src/lib.rs
  timestamp: '2025-04-14 00:11:45+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/dsl_2_a/src/main.rs
documentation_of: data-structure/segment-tree/src/lib.rs
layout: document
title: Segment Tree
---

## Description

## Usage example

[https://atcoder.jp/contests/practice2/tasks/practice2_j](https://atcoder.jp/contests/practice2/tasks/practice2_j)

## Reference

[https://atcoder.github.io/ac-library/production/document_en/segtree.html](https://atcoder.github.io/ac-library/production/document_en/segtree.html)
