---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/src/segment_tree.rs
    title: data-structure/src/segment_tree.rs
  - icon: ':heavy_check_mark:'
    path: data-structure/src/union_find.rs
    title: data-structure/src/union_find.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: data-structure/src/segment_tree.rs
    title: data-structure/src/segment_tree.rs
  - icon: ':heavy_check_mark:'
    path: data-structure/src/union_find.rs
    title: data-structure/src/union_find.rs
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
  code: 'pub mod segment_tree;

    pub use segment_tree::SegmentTree;


    pub mod union_find;

    pub use union_find::UnionFind;

    '
  dependsOn:
  - data-structure/src/segment_tree.rs
  - data-structure/src/union_find.rs
  isVerificationFile: false
  path: data-structure/src/lib.rs
  requiredBy:
  - data-structure/src/union_find.rs
  - data-structure/src/segment_tree.rs
  timestamp: '2024-03-11 21:06:54+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/library-checker/src/bin/static_range_sum.rs
  - verification/library-checker/src/bin/staticrmq.rs
  - verification/library-checker/src/bin/unionfind.rs
  - verification/aizu-online-judge/src/bin/grl_6_a.rs
  - verification/aizu-online-judge/src/bin/dsl_2_a.rs
documentation_of: data-structure/src/lib.rs
layout: document
redirect_from:
- /library/data-structure/src/lib.rs
- /library/data-structure/src/lib.rs.html
title: data-structure/src/lib.rs
---
