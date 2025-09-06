---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':x:'
    path: verification/aizu-online-judge/alds1_14_a/src/main.rs
    title: verification/aizu-online-judge/alds1_14_a/src/main.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "#[derive(Debug, Clone)]\npub struct RollingHash<T> {\n    base: T,\n    power:\
    \ Vec<T>,\n}\n\nimpl<T: Copy + From<u64>> RollingHash<T>\nwhere\n    T: std::ops::Add<T,\
    \ Output = T>,\n    T: std::ops::AddAssign,\n    T: std::ops::Sub<T, Output =\
    \ T>,\n    T: std::ops::SubAssign,\n    T: std::ops::Mul<T, Output = T>,\n   \
    \ T: std::ops::MulAssign,\n    T: std::ops::Div<T, Output = T>,\n    T: std::ops::DivAssign,\n\
    {\n    pub fn new(base: T) -> Self {\n        Self {\n            base,\n    \
    \        power: vec![T::from(1u64)],\n        }\n    }\n\n    pub fn build(&mut\
    \ self, s: &Vec<char>) -> Vec<T> {\n        let size = s.len();\n        let mut\
    \ hash = vec![T::from(0u64); size + 1];\n        for i in 0..size {\n        \
    \    hash[i + 1] = hash[i] * self.base + T::from(s[i] as u64);\n        }\n  \
    \      hash\n    }\n\n    fn build_power(&mut self, r: usize) {\n        while\
    \ self.power.len() <= r {\n            let val = *self.power.last().unwrap();\n\
    \            self.power.push(val * self.base);\n        }\n    }\n\n    // [l,\
    \ r)\n    pub fn rolling_hash(&mut self, hash: &Vec<T>, l: usize, r: usize) ->\
    \ T {\n        assert!(l <= r && r <= hash.len());\n        self.build_power(r\
    \ - l);\n        hash[r] - hash[l] * self.power[r - l]\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: string/rolling-hash/src/lib.rs
  requiredBy: []
  timestamp: '2025-05-26 17:25:50+09:00'
  verificationStatus: LIBRARY_ALL_WA
  verifiedWith:
  - verification/aizu-online-judge/alds1_14_a/src/main.rs
documentation_of: string/rolling-hash/src/lib.rs
layout: document
title: Rolling Hash
---

## Description
