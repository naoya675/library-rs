---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dpl_5_d/src/main.rs
    title: verification/aizu-online-judge/dpl_5_d/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "#[derive(Debug, Clone)]\npub struct Combinatorics<T> {\n    fact: Vec<T>,\n\
    \    finv: Vec<T>,\n    inv: Vec<T>,\n}\n\nimpl<T: Copy + From<u64>> Combinatorics<T>\n\
    where\n    T: std::ops::Add<T, Output = T>,\n    T: std::ops::AddAssign,\n   \
    \ T: std::ops::Sub<T, Output = T>,\n    T: std::ops::SubAssign,\n    T: std::ops::Mul<T,\
    \ Output = T>,\n    T: std::ops::MulAssign,\n    T: std::ops::Div<T, Output =\
    \ T>,\n    T: std::ops::DivAssign,\n{\n    pub fn new(n: usize) -> Self {\n  \
    \      let mut fact = vec![T::from(1u64); n + 1];\n        let mut finv = vec![T::from(1u64);\
    \ n + 1];\n        let mut inv = vec![T::from(1u64); n + 1];\n        for i in\
    \ 0..n {\n            fact[i + 1] = fact[i] * T::from((i + 1) as u64);\n     \
    \   }\n        finv[n] = T::from(1u64) / fact[n];\n        for i in (0..n).rev()\
    \ {\n            finv[i] = finv[i + 1] * T::from((i + 1) as u64);\n        }\n\
    \        for i in 0..n {\n            inv[i + 1] = finv[i + 1] * fact[i];\n  \
    \      }\n        Self { fact, finv, inv }\n    }\n\n    pub fn fact(&mut self,\
    \ n: usize) -> T {\n        assert!(n <= self.fact.len());\n        self.fact[n]\n\
    \    }\n\n    pub fn finv(&mut self, n: usize) -> T {\n        assert!(n <= self.finv.len());\n\
    \        self.finv[n]\n    }\n\n    pub fn inv(&mut self, n: usize) -> T {\n \
    \       assert!(n <= self.inv.len());\n        self.inv[n]\n    }\n\n    // permutation\n\
    \    pub fn perm(&mut self, n: usize, r: usize) -> T {\n        // assert!(r <=\
    \ n);\n        if n < r {\n            return T::from(0u64);\n        }\n    \
    \    self.fact(n) * self.finv(n - r)\n    }\n\n    // combination\n    pub fn\
    \ comb(&mut self, n: usize, r: usize) -> T {\n        // assert!(r <= n);\n  \
    \      if n < r {\n            return T::from(0u64);\n        }\n        self.fact(n)\
    \ * self.finv(r) * self.finv(n - r)\n    }\n\n    // combinations with replacement\
    \ (homogeneous product)\n    pub fn homo(&mut self, n: usize, r: usize) -> T {\n\
    \        self.comb(n + r - 1, r)\n    }\n\n    // catalan number\n    pub fn catalan(&mut\
    \ self, n: usize) -> T {\n        self.comb(2 * n, n) - self.comb(2 * n, n - 1)\n\
    \    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: math/combinatorics/src/lib.rs
  requiredBy: []
  timestamp: '2025-05-26 17:25:50+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/dpl_5_d/src/main.rs
documentation_of: math/combinatorics/src/lib.rs
layout: document
title: Combinatorics
---

## Description
