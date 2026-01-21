---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "#[derive(Debug, Clone)]\npub struct Doubling<T> {\n    n: usize,\n    m:\
    \ usize,\n    dpf: Vec<Vec<usize>>,\n    dpg: Vec<Vec<T>>,\n    op: fn(T, T) ->\
    \ T,\n    e: T,\n}\n\nimpl<T: Copy + Default> Doubling<T>\nwhere\n    T: std::ops::Add<T,\
    \ Output = T>,\n    T: std::ops::AddAssign,\n{\n    pub fn new_default(n: usize,\
    \ m: usize) -> Self {\n        fn op<T>(x: T, y: T) -> T\n        where\n    \
    \        T: std::ops::Add<T, Output = T>,\n            T: std::ops::AddAssign,\n\
    \        {\n            x + y\n        }\n\n        Self::new(n, m, op, T::default())\n\
    \    }\n}\n\nimpl<T: Copy> Doubling<T> {\n    pub fn new(n: usize, m: usize, op:\
    \ fn(T, T) -> T, e: T) -> Self {\n        let m = (m.next_power_of_two().ilog2()\
    \ + 1) as usize;\n        Self {\n            n,\n            m,\n           \
    \ dpf: vec![vec![0; n]; m],\n            dpg: vec![vec![e; n]; m],\n         \
    \   op,\n            e,\n        }\n    }\n\n    pub fn doubling(&mut self, f:\
    \ &[usize], g: &[T]) {\n        assert!(f.len() == self.n);\n        assert!(g.len()\
    \ == self.n);\n        for i in 0..self.n {\n            self.dpf[0][i] = f[i];\n\
    \            self.dpg[0][i] = g[i];\n        }\n        for i in 1..self.m {\n\
    \            for j in 0..self.n {\n                self.dpf[i][j] = self.dpf[i\
    \ - 1][self.dpf[i - 1][j]];\n                self.dpg[i][j] = (self.op)(self.dpg[i\
    \ - 1][j], self.dpg[i - 1][self.dpf[i - 1][j]]);\n            }\n        }\n \
    \   }\n\n    pub fn kth(&self, x: usize, k: usize) -> (usize, T) {\n        assert!(x\
    \ < self.n);\n        assert!(k < (1 << self.m));\n        if k == 0 {\n     \
    \       return (x, self.e);\n        }\n        (0..self.m)\n            .zip(self.dpf.iter())\n\
    \            .zip(self.dpg.iter())\n            .map(|((i, f), g)| (i, f, g))\n\
    \            .filter(|(i, _, _)| (k >> i) & 1 == 1)\n            .fold((x, self.e),\
    \ |(x, v), (_, f, g)| (f[x], (self.op)(v, g[x])))\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: data-structure/doubling/src/lib.rs
  requiredBy: []
  timestamp: '2026-01-21 18:49:44+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: data-structure/doubling/src/lib.rs
layout: document
title: Doubling
---

## Description

## Reference
- Verification
    - [https://atcoder.jp/contests/typical90/tasks/typical90_bf](https://atcoder.jp/contests/typical90/tasks/typical90_bf)
    - [https://atcoder.jp/contests/typical90/submissions/72124166](https://atcoder.jp/contests/typical90/submissions/72124166)
