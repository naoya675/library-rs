---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/lazy-segment-tree/src/lib.rs
    title: Lazy Segment Tree
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: data-structure/lazy-segment-tree/src/lib.rs
    title: Lazy Segment Tree
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dsl_2_f/src/main.rs
    title: verification/aizu-online-judge/dsl_2_f/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dsl_2_g/src/main.rs
    title: verification/aizu-online-judge/dsl_2_g/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dsl_2_h/src/main.rs
    title: verification/aizu-online-judge/dsl_2_h/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dsl_2_i/src/main.rs
    title: verification/aizu-online-judge/dsl_2_i/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/range_affine_range_sum/src/main.rs
    title: verification/library-checker/range_affine_range_sum/src/main.rs
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
  code: "use crate::LazySegmentTree;\n\npub struct RangeAddRangeMinimumQuery;\n\n\
    impl RangeAddRangeMinimumQuery {\n    pub fn new(n: usize) -> LazySegmentTree<i64,\
    \ i64> {\n        LazySegmentTree::new(n, |a, b| std::cmp::min(a, b), i64::MAX,\
    \ |f, x| f + x, |f, g| f + g, 0)\n    }\n}\n\npub struct RangeAddRangeMaximumQuery;\n\
    \nimpl RangeAddRangeMaximumQuery {\n    pub fn new(n: usize) -> LazySegmentTree<i64,\
    \ i64> {\n        LazySegmentTree::new(n, |a, b| std::cmp::max(a, b), i64::MIN,\
    \ |f, x| f + x, |f, g| f + g, 0)\n    }\n}\n\npub struct RangeAddRangeSumQuery;\n\
    \nimpl RangeAddRangeSumQuery {\n    pub fn new(n: usize) -> LazySegmentTree<(i64,\
    \ i64), i64> {\n        LazySegmentTree::new(n, |a, b| (a.0 + b.0, a.1 + b.1),\
    \ (0, 0), |f, x| (x.0 + f * x.1, x.1), |f, x| f + x, 0)\n    }\n}\n\npub struct\
    \ RangeUpdateRangeMinimumQuery;\n\nimpl RangeUpdateRangeMinimumQuery {\n    pub\
    \ fn new(n: usize) -> LazySegmentTree<i64, i64> {\n        LazySegmentTree::new(\n\
    \            n,\n            |a, b| std::cmp::min(a, b),\n            i64::MAX,\n\
    \            |f, x| if f == i64::MAX { x } else { f },\n            |f, g| if\
    \ f == i64::MAX { g } else { f },\n            i64::MAX,\n        )\n    }\n}\n\
    \npub struct RangeUpdateRangeMaximumQuery;\n\nimpl RangeUpdateRangeMaximumQuery\
    \ {\n    pub fn new(n: usize) -> LazySegmentTree<i64, i64> {\n        LazySegmentTree::new(\n\
    \            n,\n            |a, b| std::cmp::max(a, b),\n            i64::MIN,\n\
    \            |f, x| if f == i64::MAX { x } else { f },\n            |f, g| if\
    \ f == i64::MAX { g } else { f },\n            i64::MAX,\n        )\n    }\n}\n\
    \npub struct RangeUpdateRangeSumQuery;\n\nimpl RangeUpdateRangeSumQuery {\n  \
    \  pub fn new(n: usize) -> LazySegmentTree<(i64, i64), i64> {\n        LazySegmentTree::new(\n\
    \            n,\n            |a, b| (a.0 + b.0, a.1 + b.1),\n            (0, 0),\n\
    \            |f, x| if f != i64::MAX { (f * x.1, x.1) } else { x },\n        \
    \    |f, g| if f == i64::MAX { g } else { f },\n            i64::MAX,\n      \
    \  )\n    }\n}\n\npub struct RangeAffineRangeSumQuery;\n\nimpl RangeAffineRangeSumQuery\
    \ {\n    pub fn new(n: usize) -> LazySegmentTree<(i64, i64), (i64, i64)> {\n \
    \       LazySegmentTree::new(\n            n,\n            |a, b| (a.0 + b.0,\
    \ a.1 + b.1),\n            (0, 0),\n            |a, b| (a.0 * b.0 + a.1 * b.1,\
    \ b.1),\n            |a, b| (a.0 * b.0, a.0 * b.1 + a.1),\n            (1, 0),\n\
    \        )\n    }\n}\n"
  dependsOn:
  - data-structure/lazy-segment-tree/src/lib.rs
  isVerificationFile: false
  path: data-structure/lazy-segment-tree/src/wrapper.rs
  requiredBy:
  - data-structure/lazy-segment-tree/src/lib.rs
  timestamp: '2025-04-19 06:22:15+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/library-checker/range_affine_range_sum/src/main.rs
  - verification/aizu-online-judge/dsl_2_i/src/main.rs
  - verification/aizu-online-judge/dsl_2_g/src/main.rs
  - verification/aizu-online-judge/dsl_2_h/src/main.rs
  - verification/aizu-online-judge/dsl_2_f/src/main.rs
documentation_of: data-structure/lazy-segment-tree/src/wrapper.rs
layout: document
redirect_from:
- /library/data-structure/lazy-segment-tree/src/wrapper.rs
- /library/data-structure/lazy-segment-tree/src/wrapper.rs.html
title: data-structure/lazy-segment-tree/src/wrapper.rs
---
