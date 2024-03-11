---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dsl_2_a/src/main.rs
    title: verification/aizu-online-judge/dsl_2_a/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/static_range_sum/src/main.rs
    title: verification/library-checker/static_range_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/staticrmq/src/main.rs
    title: verification/library-checker/staticrmq/src/main.rs
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
    \         op,\n            e,\n        }\n    }\n\n    pub fn set(&mut self, mut\
    \ k: usize, x: T) {\n        k += self.size;\n        self.tree[k] = x;\n    \
    \    while k > 0 {\n            k /= 2;\n            self.tree[k] = (self.op)(self.tree[2\
    \ * k], self.tree[2 * k + 1]);\n        }\n    }\n\n    pub fn prod(&mut self,\
    \ mut l: usize, mut r: usize) -> T {\n        l += self.size;\n        r += self.size;\n\
    \        let mut res = self.e;\n        while l < r {\n            if l % 2 ==\
    \ 1 {\n                res = (self.op)(res, self.tree[l]);\n                l\
    \ += 1;\n            }\n            l /= 2;\n            if r % 2 == 1 {\n   \
    \             res = (self.op)(res, self.tree[r - 1]);\n                r -= 1;\n\
    \            }\n            r /= 2;\n        }\n        res\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: data-structure/segment-tree/src/lib.rs
  requiredBy: []
  timestamp: '2024-03-11 21:49:40+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/library-checker/staticrmq/src/main.rs
  - verification/library-checker/static_range_sum/src/main.rs
  - verification/aizu-online-judge/dsl_2_a/src/main.rs
documentation_of: data-structure/segment-tree/src/lib.rs
layout: document
redirect_from:
- /library/data-structure/segment-tree/src/lib.rs
- /library/data-structure/segment-tree/src/lib.rs.html
title: data-structure/segment-tree/src/lib.rs
---
