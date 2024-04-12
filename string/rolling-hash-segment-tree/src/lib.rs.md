---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/segment-tree/src/lib.rs
    title: Segment Tree
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/alds1_14_a/src/main.rs
    title: verification/aizu-online-judge/alds1_14_a/src/main.rs
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
  code: "use rand::Rng;\n\nuse segment_tree::SegmentTree;\n\n#[derive(Debug, Clone)]\n\
    pub struct RollingHash {\n    base: u64,\n    power: Vec<u64>,\n}\n\nimpl RollingHash\
    \ {\n    const MOD: u64 = (1_u64 << 61) - 1;\n    const MASK30: u64 = (1_u64 <<\
    \ 30) - 1;\n    const MASK31: u64 = (1_u64 << 31) - 1;\n    const MASK61: u64\
    \ = Self::MOD;\n    const POSITIVIZER: u64 = Self::MOD * 4;\n\n    pub fn new()\
    \ -> Self {\n        let mut rng = rand::thread_rng();\n        let base = rng.gen_range(1..Self::MOD);\n\
    \        Self {\n            base,\n            power: vec![1],\n        }\n \
    \   }\n\n    pub fn build_segment_tree(&mut self, s: &Vec<char>) -> SegmentTree<(u64,\
    \ u64)> {\n        let size = s.len();\n        let mut st = SegmentTree::<(u64,\
    \ u64)>::new(\n            size,\n            |a, b| {\n                (\n  \
    \                  Self::calc_mod(Self::calc_mul(a.1, b.0) + a.0),\n         \
    \           Self::calc_mod(Self::calc_mul(a.1, b.1)),\n                )\n   \
    \         },\n            (0, 1),\n        );\n        st.build(s.into_iter().map(|&f|\
    \ (f as u64, self.base)).collect());\n        st\n    }\n\n    pub fn build(&mut\
    \ self, s: &Vec<char>) -> Vec<u64> {\n        let size = s.len();\n        let\
    \ mut hash = vec![0; size + 1];\n        for i in 0..size {\n            hash[i\
    \ + 1] = Self::calc_mod(Self::calc_mul(hash[i], self.base) + s[i] as u64);\n \
    \       }\n        hash\n    }\n\n    fn build_power(&mut self, r: usize) {\n\
    \        while self.power.len() <= r {\n            let val = *self.power.last().unwrap();\n\
    \            self.power\n                .push(Self::calc_mod(Self::calc_mul(val,\
    \ self.base)));\n        }\n    }\n\n    // [l, r)\n    pub fn rolling_hash(&mut\
    \ self, hash: &Vec<u64>, l: usize, r: usize) -> u64 {\n        assert!(l <= r\
    \ && r <= hash.len());\n        self.build_power(r - l);\n        Self::calc_mod(hash[r]\
    \ + Self::POSITIVIZER - Self::calc_mul(hash[l], self.power[r - l]))\n    }\n\n\
    \    // fn calc_add(a: u64, b: u64) -> u64 {\n    //     let mut res = a + b;\n\
    \    //     if res >= Self::MOD {\n    //         res -= Self::MOD;\n    //  \
    \   }\n    //     res\n    // }\n\n    fn calc_mul(a: u64, b: u64) -> u64 {\n\
    \        let au = a >> 31;\n        let ad = a & Self::MASK31;\n        let bu\
    \ = b >> 31;\n        let bd = b & Self::MASK31;\n        let mid = ad * bu +\
    \ au * bd;\n        let midu = mid >> 30;\n        let midd = mid & Self::MASK30;\n\
    \        ((au * bu) << 1) + midu + (midd << 31) + ad * bd\n        // Self::calc_mod(((au\
    \ * bu) << 1) + midu + (midd << 31) + ad * bd)\n    }\n\n    fn calc_mod(a: u64)\
    \ -> u64 {\n        let au = a >> 61;\n        let ad = a & Self::MASK61;\n  \
    \      let mut res = au + ad;\n        if res >= Self::MOD {\n            res\
    \ -= Self::MOD;\n        }\n        res\n        // Self::calc_add(au, ad)\n \
    \   }\n}\n"
  dependsOn:
  - data-structure/segment-tree/src/lib.rs
  isVerificationFile: false
  path: string/rolling-hash-segment-tree/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-09 23:57:06+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/alds1_14_a/src/main.rs
documentation_of: string/rolling-hash-segment-tree/src/lib.rs
layout: document
title: Rolling Hash + Segment Tree
---

## Description
