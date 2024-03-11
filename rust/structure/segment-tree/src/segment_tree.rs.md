---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: rust/structure/segment-tree/src/lib.rs
    title: rust/structure/segment-tree/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: rust/structure/segment-tree/src/lib.rs
    title: rust/structure/segment-tree/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: rust/verification/aizu-online-judge/src/bin/dsl_2_a.rs
    title: rust/verification/aizu-online-judge/src/bin/dsl_2_a.rs
  - icon: ':heavy_check_mark:'
    path: rust/verification/library-checker/src/bin/staticrmq.rs
    title: rust/verification/library-checker/src/bin/staticrmq.rs
  - icon: ':heavy_check_mark:'
    path: rust/verification/library-checker/src/bin/unionfind.rs
    title: rust/verification/library-checker/src/bin/unionfind.rs
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
  code: "pub struct SegmentTree<T> {\n    tree: Vec<T>,\n    size: usize,\n    op:\
    \ fn(T, T) -> T, // evaluation funciton\n    e: T,              // identity element\n\
    }\n\nimpl<T: Copy> SegmentTree<T> {\n    pub fn new(n: usize, op: fn(T, T) ->\
    \ T, e: T) -> Self {\n        let size = n.next_power_of_two();\n        Self\
    \ {\n            tree: vec![e; 2 * size],\n            size,\n            op,\n\
    \            e,\n        }\n    }\n\n    pub fn set(&mut self, mut k: usize, x:\
    \ T) {\n        k += self.size;\n        self.tree[k] = x;\n        while k >\
    \ 0 {\n            k /= 2;\n            self.tree[k] = (self.op)(self.tree[2 *\
    \ k], self.tree[2 * k + 1]);\n        }\n    }\n\n    pub fn prod(&mut self, mut\
    \ l: usize, mut r: usize) -> T {\n        l += self.size;\n        r += self.size;\n\
    \        let mut res = self.e;\n        while l < r {\n            if l % 2 ==\
    \ 1 {\n                res = (self.op)(res, self.tree[l]);\n                l\
    \ += 1;\n            }\n            l /= 2;\n            if r % 2 == 1 {\n   \
    \             res = (self.op)(res, self.tree[r - 1]);\n                r -= 1;\n\
    \            }\n            r /= 2;\n        }\n        res\n    }\n}\n"
  dependsOn:
  - rust/structure/segment-tree/src/lib.rs
  isVerificationFile: false
  path: rust/structure/segment-tree/src/segment_tree.rs
  requiredBy:
  - rust/structure/segment-tree/src/lib.rs
  timestamp: '2024-03-11 14:31:54+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - rust/verification/library-checker/src/bin/staticrmq.rs
  - rust/verification/library-checker/src/bin/unionfind.rs
  - rust/verification/aizu-online-judge/src/bin/dsl_2_a.rs
documentation_of: rust/structure/segment-tree/src/segment_tree.rs
layout: document
redirect_from:
- /library/rust/structure/segment-tree/src/segment_tree.rs
- /library/rust/structure/segment-tree/src/segment_tree.rs.html
title: rust/structure/segment-tree/src/segment_tree.rs
---
