---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: data-structure/lazy-segment-tree/src/lib.rs
    title: Lazy Segment Tree
  _extendedRequiredBy:
  - icon: ':question:'
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
  - icon: ':x:'
    path: verification/library-checker/range_affine_range_sum/src/main.rs
    title: verification/library-checker/range_affine_range_sum/src/main.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use crate::LazySegmentTree;\n\npub struct RangeAddRangeMinimumQuery;\nimpl\
    \ RangeAddRangeMinimumQuery {\n    pub fn new(n: usize) -> LazySegmentTree<i64,\
    \ i64> {\n        LazySegmentTree::new(n, |x, y| std::cmp::min(x, y), i64::MAX,\
    \ |f, x| f + x, |f, g| f + g, 0)\n    }\n}\n\npub struct RangeAddRangeMaximumQuery;\n\
    impl RangeAddRangeMaximumQuery {\n    pub fn new(n: usize) -> LazySegmentTree<i64,\
    \ i64> {\n        LazySegmentTree::new(n, |x, y| std::cmp::max(x, y), i64::MIN,\
    \ |f, x| f + x, |f, g| f + g, 0)\n    }\n}\n\npub struct RangeAddRangeSumQuery;\n\
    impl RangeAddRangeSumQuery {\n    pub fn new(n: usize) -> LazySegmentTree<(i64,\
    \ i64), i64> {\n        LazySegmentTree::new(n, |x, y| (x.0 + y.0, x.1 + y.1),\
    \ (0, 0), |f, x| (x.0 + f * x.1, x.1), |f, g| f + g, 0)\n    }\n}\n\npub struct\
    \ RangeUpdateRangeMinimumQuery;\nimpl RangeUpdateRangeMinimumQuery {\n    pub\
    \ fn new(n: usize) -> LazySegmentTree<i64, i64> {\n        LazySegmentTree::new(\n\
    \            n,\n            |x, y| std::cmp::min(x, y),\n            i64::MAX,\n\
    \            |f, x| if f == i64::MAX { x } else { f },\n            |f, g| if\
    \ f == i64::MAX { g } else { f },\n            i64::MAX,\n        )\n    }\n}\n\
    \npub struct RangeUpdateRangeMaximumQuery;\nimpl RangeUpdateRangeMaximumQuery\
    \ {\n    pub fn new(n: usize) -> LazySegmentTree<i64, i64> {\n        LazySegmentTree::new(\n\
    \            n,\n            |x, y| std::cmp::max(x, y),\n            i64::MIN,\n\
    \            |f, x| if f == i64::MAX { x } else { f },\n            |f, g| if\
    \ f == i64::MAX { g } else { f },\n            i64::MAX,\n        )\n    }\n}\n\
    \npub struct RangeUpdateRangeSumQuery;\nimpl RangeUpdateRangeSumQuery {\n    pub\
    \ fn new(n: usize) -> LazySegmentTree<(i64, i64), i64> {\n        LazySegmentTree::new(\n\
    \            n,\n            |x, y| (x.0 + y.0, x.1 + y.1),\n            (0, 0),\n\
    \            |f, x| if f != i64::MAX { (f * x.1, x.1) } else { x },\n        \
    \    |f, g| if f == i64::MAX { g } else { f },\n            i64::MAX,\n      \
    \  )\n    }\n}\n\npub struct RangeAffineRangeSumQuery;\nimpl RangeAffineRangeSumQuery\
    \ {\n    pub fn new(n: usize) -> LazySegmentTree<(i64, i64), (i64, i64)> {\n \
    \       LazySegmentTree::new(\n            n,\n            |x, y| (x.0 + y.0,\
    \ x.1 + y.1),\n            (0, 0),\n            |f, x| (f.0 * x.0 + f.1 * x.1,\
    \ x.1),\n            |f, g| (f.0 * g.0, f.0 * g.1 + f.1),\n            (1, 0),\n\
    \        )\n    }\n}\n\npub mod range_arithmetic_sequence_add {\n    use crate::LazySegmentTree;\n\
    \n    #[derive(Debug, Clone, Copy)]\n    pub struct S {\n        value_sum: i64,\n\
    \        index_sum: i64,\n        len: i64,\n    }\n\n    #[derive(Debug, Clone,\
    \ Copy)]\n    pub struct F {\n        a: i64,\n        b: i64,\n    }\n\n    pub\
    \ struct RangeArithmeticSequenceAdd;\n    impl RangeArithmeticSequenceAdd {\n\
    \        pub fn new(n: usize) -> LazySegmentTree<S, F> {\n            LazySegmentTree::new(\n\
    \                n,\n                |x, y| S {\n                    value_sum:\
    \ x.value_sum + y.value_sum,\n                    index_sum: x.index_sum + y.index_sum,\n\
    \                    len: x.len + y.len,\n                },\n               \
    \ S {\n                    value_sum: 0,\n                    index_sum: 0,\n\
    \                    len: 0,\n                },\n                |f, x| S {\n\
    \                    value_sum: x.value_sum + x.index_sum * f.a + x.len * f.b,\n\
    \                    index_sum: x.index_sum,\n                    len: x.len,\n\
    \                },\n                |f, g| F { a: f.a + g.a, b: f.b + g.b },\n\
    \                F { a: 0, b: 0 },\n            )\n        }\n    }\n}\n"
  dependsOn:
  - data-structure/lazy-segment-tree/src/lib.rs
  isVerificationFile: false
  path: data-structure/lazy-segment-tree/src/wrapper.rs
  requiredBy:
  - data-structure/lazy-segment-tree/src/lib.rs
  timestamp: '2026-01-01 00:11:18+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - verification/aizu-online-judge/dsl_2_i/src/main.rs
  - verification/aizu-online-judge/dsl_2_f/src/main.rs
  - verification/aizu-online-judge/dsl_2_g/src/main.rs
  - verification/aizu-online-judge/dsl_2_h/src/main.rs
  - verification/library-checker/range_affine_range_sum/src/main.rs
documentation_of: data-structure/lazy-segment-tree/src/wrapper.rs
layout: document
title: Lazy Segment Tree (Wrapper)
---

<!--## Description-->

## Reference
- Range Update Range Query
    - [https://betrue12.hateblo.jp/entry/2020/09/22/194541](https://betrue12.hateblo.jp/entry/2020/09/22/194541)
    - [https://betrue12.hateblo.jp/entry/2020/09/23/005940](https://betrue12.hateblo.jp/entry/2020/09/23/005940)

- Range Arithmetic Sequence Add
    - [https://atcoder.jp/contests/abc407/editorial/13118](https://atcoder.jp/contests/abc407/editorial/13118)
