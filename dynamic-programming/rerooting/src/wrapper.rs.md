---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: dynamic-programming/rerooting/src/lib.rs
    title: Rerooting
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: dynamic-programming/rerooting/src/lib.rs
    title: Rerooting
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/grl_5_a/src/main.rs
    title: verification/aizu-online-judge/grl_5_a/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://atcoder.jp/contests/abc222/editorial/2749
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use crate::Rerooting;\n\npub struct RerootingDiameter;\n\nimpl RerootingDiameter\
    \ {\n    pub fn new(\n        n: usize,\n    ) -> Rerooting<usize, usize, impl\
    \ Fn(usize, usize) -> usize, impl Fn() -> usize, impl Fn() -> usize, impl Fn(usize,\
    \ usize, usize, usize) -> usize> {\n        let merge = |a: usize, b: usize| std::cmp::max(a,\
    \ b);\n        let e = || 0;\n        let leaf = || 0;\n        let apply = |a:\
    \ usize, _: usize, _: usize, w: usize| a + w;\n        Rerooting::new(n, merge,\
    \ e, leaf, apply)\n    }\n}\n\n// reference: https://atcoder.jp/contests/abc222/editorial/2749\n"
  dependsOn:
  - dynamic-programming/rerooting/src/lib.rs
  isVerificationFile: false
  path: dynamic-programming/rerooting/src/wrapper.rs
  requiredBy:
  - dynamic-programming/rerooting/src/lib.rs
  timestamp: '2025-04-19 06:22:15+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/grl_5_a/src/main.rs
documentation_of: dynamic-programming/rerooting/src/wrapper.rs
layout: document
redirect_from:
- /library/dynamic-programming/rerooting/src/wrapper.rs
- /library/dynamic-programming/rerooting/src/wrapper.rs.html
title: dynamic-programming/rerooting/src/wrapper.rs
---
