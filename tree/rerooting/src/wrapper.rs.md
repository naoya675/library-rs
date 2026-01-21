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
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "pub mod rerooting_diameter {\n    use crate::Rerooting;\n\n    type Cost\
    \ = usize;\n    type Data = usize;\n\n    pub struct RerootingDiameter;\n    impl\
    \ RerootingDiameter {\n        pub fn new(\n            n: usize,\n        ) ->\
    \ Rerooting<Cost, Data, impl Fn(Data, Data) -> Data, impl Fn() -> Data, impl Fn(usize)\
    \ -> Data, impl Fn(Data, usize, usize, Cost) -> Data> {\n            Rerooting::new(\n\
    \                n,\n                |x: Data, y: Data| std::cmp::max(x, y),\n\
    \                || 0,\n                |_: usize| 0,\n                |x: Data,\
    \ _: usize, _: usize, w: Cost| x + w,\n            )\n        }\n    }\n}\n\n\
    /*\n *\npub mod rerooting_diameter {\n    use crate::Rerooting;\n\n    type Cost\
    \ = usize;\n    type Data = (usize, usize);\n\n    pub struct RerootingDiameter;\n\
    \    impl RerootingDiameter {\n        pub fn new(\n            n: usize,\n  \
    \      ) -> Rerooting<Cost, Data, impl Fn(Data, Data) -> Data, impl Fn() -> Data,\
    \ impl Fn(usize) -> Data, impl Fn(Data, usize, usize, Cost) -> Data> {\n     \
    \       Rerooting::new(\n                n,\n                |x: Data, y: Data|\
    \ std::cmp::max(x, y),\n                || (0, 0),\n                |c: usize|\
    \ (0, c),\n                |x: Data, _: usize, _: usize, w: Cost| (x.0 + w, x.1),\n\
    \            )\n        }\n    }\n}\n */\n"
  dependsOn:
  - tree/rerooting/src/lib.rs
  isVerificationFile: false
  path: tree/rerooting/src/wrapper.rs
  requiredBy:
  - tree/rerooting/src/lib.rs
  timestamp: '2025-12-08 22:55:07+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/grl_5_a/src/main.rs
  - verification/aizu-online-judge/grl_5_b/src/main.rs
documentation_of: tree/rerooting/src/wrapper.rs
layout: document
title: Rerooting (Wrapper)
---

<!--## Description-->

## Reference
- Diameter
    - [https://atcoder.jp/contests/abc428/editorial/14240](https://atcoder.jp/contests/abc428/editorial/14240)
