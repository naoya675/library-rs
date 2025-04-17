---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/segment-tree/src/lib.rs
    title: Segment Tree
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/alds1_14_b/src/main.rs
    title: verification/aizu-online-judge/alds1_14_b/src/main.rs
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
  code: "use segment_tree::SegmentTree;\n\n#[derive(Debug, Clone)]\npub struct RollingHash<T>\
    \ {\n    base: T,\n    power: Vec<T>,\n}\n\nimpl<T: Copy + From<u64>> RollingHash<T>\n\
    where\n    T: std::ops::Neg<Output = T>,\n    T: std::ops::Add<T, Output = T>,\n\
    \    T: std::ops::AddAssign,\n    T: std::ops::Sub<T, Output = T>,\n    T: std::ops::SubAssign,\n\
    \    T: std::ops::Mul<T, Output = T>,\n    T: std::ops::MulAssign,\n    T: std::ops::Div<T,\
    \ Output = T>,\n    T: std::ops::DivAssign,\n{\n    pub fn new(base: T) -> Self\
    \ {\n        Self {\n            base,\n            power: vec![T::from(1u64)],\n\
    \        }\n    }\n\n    pub fn build_segment_tree(&mut self, s: &Vec<char>) ->\
    \ SegmentTree<(T, T)> {\n        let size = s.len();\n        let mut st = SegmentTree::<(T,\
    \ T)>::new(\n            size,\n            |a, b| (a.0 + (a.1 * b.0), a.1 * b.1),\n\
    \            (T::from(0u64), T::from(1u64)),\n        );\n        st.build(\n\
    \            s.into_iter()\n                .map(|&f| (T::from(f as u64), self.base))\n\
    \                .collect(),\n        );\n        st\n    }\n\n    pub fn build(&mut\
    \ self, s: &Vec<char>) -> Vec<T> {\n        let size = s.len();\n        let mut\
    \ hash = vec![T::from(0u64); size + 1];\n        for i in 0..size {\n        \
    \    hash[i + 1] = hash[i] * self.base + T::from(s[i] as u64);\n        }\n  \
    \      hash\n    }\n\n    fn build_power(&mut self, r: usize) {\n        while\
    \ self.power.len() <= r {\n            let val = *self.power.last().unwrap();\n\
    \            self.power.push(val * self.base);\n        }\n    }\n\n    // [l,\
    \ r)\n    pub fn rolling_hash(&mut self, hash: &Vec<T>, l: usize, r: usize) ->\
    \ T {\n        assert!(l <= r && r <= hash.len());\n        self.build_power(r\
    \ - l);\n        hash[r] - hash[l] * self.power[r - l]\n    }\n}\n"
  dependsOn:
  - data-structure/segment-tree/src/lib.rs
  isVerificationFile: false
  path: string/rolling-hash-segment-tree/src/lib.rs
  requiredBy: []
  timestamp: '2025-04-18 00:17:29+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/alds1_14_b/src/main.rs
documentation_of: string/rolling-hash-segment-tree/src/lib.rs
layout: document
title: Rolling Hash + Segment Tree
---

## Description
