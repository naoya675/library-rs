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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "pub struct RerootingDiameter;\nimpl RerootingDiameter {\n    pub fn new(\n\
    \        n: usize,\n    ) -> Rerooting<usize, usize, impl Fn(usize, usize) ->\
    \ usize, impl Fn() -> usize, impl Fn() -> usize, impl Fn(usize, usize, usize,\
    \ usize) -> usize> {\n        Rerooting::new(\n            n,\n            |a:\
    \ usize, b: usize| std::cmp::max(a, b),\n            || 0,\n            || 0,\n\
    \            |a: usize, _: usize, _: usize, w: usize| a + w,\n        )\n    }\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: tree/rerooting/src/wrapper.rs
  requiredBy: []
  timestamp: '2025-09-05 20:18:54+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: tree/rerooting/src/wrapper.rs
layout: document
redirect_from:
- /library/tree/rerooting/src/wrapper.rs
- /library/tree/rerooting/src/wrapper.rs.html
title: tree/rerooting/src/wrapper.rs
---
