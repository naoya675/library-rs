---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: math/mod-int/src/lib.rs
    title: Mod Int
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use mod_int::ModInt;\n\n#[derive(Debug, Clone)]\npub struct ModCombinatorial<const\
    \ MOD: u64> {\n    fact: Vec<ModInt<MOD>>,\n    finv: Vec<ModInt<MOD>>,\n}\n\n\
    impl<const MOD: u64> ModCombinatorial<MOD> {\n    pub fn new(n: usize) -> Self\
    \ {\n        let mut fact = vec![ModInt::<MOD>::new(1); n + 1];\n        let mut\
    \ finv = vec![ModInt::<MOD>::new(1); n + 1];\n        for i in 0..n {\n      \
    \      fact[i + 1] = fact[i] * ModInt::<MOD>::new((i + 1) as u64);\n        }\n\
    \        finv[n] = fact[n].inv();\n        for i in (0..n).rev() {\n         \
    \   finv[i] = finv[i + 1] * ModInt::<MOD>::new((i + 1) as u64);\n        }\n \
    \       Self { fact, finv }\n    }\n\n    pub fn fact(&self, n: usize) -> ModInt<MOD>\
    \ {\n        assert!(n <= self.fact.len());\n        self.fact[n]\n    }\n\n \
    \   pub fn finv(&self, n: usize) -> ModInt<MOD> {\n        assert!(n <= self.finv.len());\n\
    \        self.finv[n]\n    }\n\n    // permutation\n    pub fn perm(&self, n:\
    \ usize, r: usize) -> ModInt<MOD> {\n        // assert!(r <= n);\n        if r\
    \ > n {\n            return ModInt::<MOD>::new(0);\n        }\n        self.fact[n]\
    \ * self.finv[n - r]\n    }\n\n    // combination\n    pub fn comb(&self, n: usize,\
    \ r: usize) -> ModInt<MOD> {\n        // assert!(r <= n);\n        if r > n {\n\
    \            return ModInt::<MOD>::new(0);\n        }\n        self.fact[n] *\
    \ self.finv[r] * self.finv[n - r]\n    }\n\n    // combinations with replacement\
    \ (homogeneous product)\n    pub fn homo(&self, n: usize, r: usize) -> ModInt<MOD>\
    \ {\n        self.comb(n + r - 1, r)\n    }\n}\n"
  dependsOn:
  - math/mod-int/src/lib.rs
  isVerificationFile: false
  path: math/mod-combinatorial/src/lib.rs
  requiredBy: []
  timestamp: '2025-04-04 10:54:03+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/dpl_5_d/src/main.rs
documentation_of: math/mod-combinatorial/src/lib.rs
layout: document
title: Mod Combinatorial
---

## Description
