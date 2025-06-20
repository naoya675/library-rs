---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: tree/rerooting/src/lib.rs
    title: Rerooting
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: tree/rerooting/src/lib.rs
    title: Rerooting
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/grl_5_a/src/main.rs
    title: verification/aizu-online-judge/grl_5_a/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/grl_5_b/src/main.rs
    title: verification/aizu-online-judge/grl_5_b/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://atcoder.jp/contests/abc222/editorial/2749
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use crate::Rerooting;\n\n// reference: https://atcoder.jp/contests/abc222/editorial/2749\n\
    pub struct RerootingDiameter;\n\nimpl RerootingDiameter {\n    pub fn new(\n \
    \       n: usize,\n    ) -> Rerooting<usize, usize, impl Fn(usize, usize) -> usize,\
    \ impl Fn() -> usize, impl Fn() -> usize, impl Fn(usize, usize, usize, usize)\
    \ -> usize> {\n        Rerooting::new(\n            n,\n            |a: usize,\
    \ b: usize| std::cmp::max(a, b),\n            || 0,\n            || 0,\n     \
    \       |a: usize, _: usize, _: usize, w: usize| a + w,\n        )\n    }\n}\n"
  dependsOn:
  - tree/rerooting/src/lib.rs
  isVerificationFile: false
  path: tree/rerooting/src/wrapper.rs
  requiredBy:
  - tree/rerooting/src/lib.rs
  timestamp: '2025-05-26 15:54:30+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/grl_5_b/src/main.rs
  - verification/aizu-online-judge/grl_5_a/src/main.rs
documentation_of: tree/rerooting/src/wrapper.rs
layout: document
redirect_from:
- /library/tree/rerooting/src/wrapper.rs
- /library/tree/rerooting/src/wrapper.rs.html
title: tree/rerooting/src/wrapper.rs
---
