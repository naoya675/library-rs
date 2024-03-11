---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: graph/src/ford_fulkerson.rs
    title: graph/src/ford_fulkerson.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: graph/src/ford_fulkerson.rs
    title: graph/src/ford_fulkerson.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/src/bin/dsl_2_a.rs
    title: verification/aizu-online-judge/src/bin/dsl_2_a.rs
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/src/bin/grl_6_a.rs
    title: verification/aizu-online-judge/src/bin/grl_6_a.rs
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/src/bin/static_range_sum.rs
    title: verification/library-checker/src/bin/static_range_sum.rs
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/src/bin/staticrmq.rs
    title: verification/library-checker/src/bin/staticrmq.rs
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/src/bin/unionfind.rs
    title: verification/library-checker/src/bin/unionfind.rs
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
  code: 'pub mod ford_fulkerson;

    pub use ford_fulkerson::{Edge, FordFulkerson};

    '
  dependsOn:
  - graph/src/ford_fulkerson.rs
  isVerificationFile: false
  path: graph/src/lib.rs
  requiredBy:
  - graph/src/ford_fulkerson.rs
  timestamp: '2024-03-11 21:07:02+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/library-checker/src/bin/static_range_sum.rs
  - verification/library-checker/src/bin/staticrmq.rs
  - verification/library-checker/src/bin/unionfind.rs
  - verification/aizu-online-judge/src/bin/grl_6_a.rs
  - verification/aizu-online-judge/src/bin/dsl_2_a.rs
documentation_of: graph/src/lib.rs
layout: document
redirect_from:
- /library/graph/src/lib.rs
- /library/graph/src/lib.rs.html
title: graph/src/lib.rs
---
