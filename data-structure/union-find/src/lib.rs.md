---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: graph/kruskal/src/lib.rs
    title: Kruskal
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dsl_1_a/src/main.rs
    title: verification/aizu-online-judge/dsl_1_a/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/unionfind/src/main.rs
    title: verification/library-checker/unionfind/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://atcoder.github.io/ac-library/production/document_en/dsu.html
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// reference: https://atcoder.github.io/ac-library/production/document_en/dsu.html\n\
    \n#[derive(Debug, Clone)]\npub struct UnionFind {\n    n: usize,\n    par: Vec<usize>,\n\
    \    siz: Vec<usize>,\n}\n\nimpl UnionFind {\n    pub fn new(n: usize) -> Self\
    \ {\n        Self {\n            n,\n            par: (0..n).collect::<Vec<usize>>(),\n\
    \            siz: vec![1; n],\n        }\n    }\n\n    pub fn merge(&mut self,\
    \ x: usize, y: usize) -> usize {\n        assert!(x < self.n);\n        assert!(y\
    \ < self.n);\n        let mut x = self.leader(x);\n        let mut y = self.leader(y);\n\
    \        if x == y {\n            return x;\n        }\n        if self.siz[x]\
    \ < self.siz[y] {\n            std::mem::swap(&mut x, &mut y);\n        }\n  \
    \      self.siz[x] += self.siz[y];\n        self.par[y] = x;\n        x\n    }\n\
    \n    pub fn same(&mut self, x: usize, y: usize) -> bool {\n        assert!(x\
    \ < self.n);\n        assert!(y < self.n);\n        self.leader(x) == self.leader(y)\n\
    \    }\n\n    pub fn leader(&mut self, x: usize) -> usize {\n        assert!(x\
    \ < self.n);\n        if self.par[x] == x {\n            return x;\n        }\n\
    \        let leader = self.leader(self.par[x]);\n        self.par[x] = leader;\n\
    \        self.par[x]\n    }\n\n    pub fn size(&mut self, x: usize) -> usize {\n\
    \        assert!(x < self.n);\n        let x = self.leader(x);\n        self.siz[x]\n\
    \    }\n\n    pub fn groups(&mut self) -> Vec<Vec<usize>> {\n        let mut res\
    \ = vec![vec![]; self.n];\n        for i in 0..self.n {\n            res[self.leader(i)].push(i);\n\
    \        }\n        res.into_iter().filter(|f| !f.is_empty()).collect::<Vec<_>>()\n\
    \    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: data-structure/union-find/src/lib.rs
  requiredBy:
  - graph/kruskal/src/lib.rs
  timestamp: '2025-06-21 17:54:09+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/library-checker/unionfind/src/main.rs
  - verification/aizu-online-judge/dsl_1_a/src/main.rs
documentation_of: data-structure/union-find/src/lib.rs
layout: document
title: Union Find
---

## Description
