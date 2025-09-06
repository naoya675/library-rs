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
  code: "pub struct RangeAddRangeMinimumQuery;\nimpl RangeAddRangeMinimumQuery {\n\
    \    pub fn new(n: usize) -> LazySegmentTree<i64, i64> {\n        LazySegmentTree::new(n,\
    \ |x, y| std::cmp::min(x, y), i64::MAX, |f, x| f + x, |f, g| f + g, 0)\n    }\n\
    }\n\npub struct RangeAddRangeMaximumQuery;\nimpl RangeAddRangeMaximumQuery {\n\
    \    pub fn new(n: usize) -> LazySegmentTree<i64, i64> {\n        LazySegmentTree::new(n,\
    \ |x, y| std::cmp::max(x, y), i64::MIN, |f, x| f + x, |f, g| f + g, 0)\n    }\n\
    }\n\npub struct RangeAddRangeSumQuery;\nimpl RangeAddRangeSumQuery {\n    pub\
    \ fn new(n: usize) -> LazySegmentTree<(i64, i64), i64> {\n        LazySegmentTree::new(n,\
    \ |x, y| (x.0 + y.0, x.1 + y.1), (0, 0), |f, x| (x.0 + f * x.1, x.1), |f, g| f\
    \ + g, 0)\n    }\n}\n\npub struct RangeUpdateRangeMinimumQuery;\nimpl RangeUpdateRangeMinimumQuery\
    \ {\n    pub fn new(n: usize) -> LazySegmentTree<i64, i64> {\n        LazySegmentTree::new(\n\
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
    \        )\n    }\n}\n\npub mod RangeArithmeticSequenceAddRangeSumQuery {\n  \
    \  #[derive(Debug, Clone, Copy)]\n    pub struct S {\n        value_sum: i64,\n\
    \        index_sum: i64,\n        len: i64,\n    }\n    impl S {\n        pub\
    \ fn new(value_sum: i64, index_sum: i64, len: i64) -> Self {\n            Self\
    \ { value_sum, index_sum, len }\n        }\n    }\n\n    #[derive(Debug, Clone,\
    \ Copy)]\n    pub struct F {\n        a: i64,\n        b: i64,\n    }\n    impl\
    \ F {\n        pub fn new(a: i64, b: i64) -> Self {\n            Self { a, b }\n\
    \        }\n    }\n\n    pub struct RangeArithmeticSequenceAddRangeSumQuery;\n\
    \    impl RangeArithmeticSequenceAddRangeSumQuery {\n        pub fn new(n: usize)\
    \ -> LazySegmentTree<S, F> {\n            LazySegmentTree::new(\n            \
    \    n,\n                |x, y| S::new(x.value_sum + y.value_sum, x.index_sum\
    \ + y.index_sum, x.len + y.len),\n                S::new(0, 0, 0),\n         \
    \       |f, x| S::new(x.value_sum + x.index_sum * f.a + x.len * f.b, x.index_sum,\
    \ x.len),\n                |f, g| F::new(f.a + g.a, f.b + g.b),\n            \
    \    F::new(0, 0),\n            )\n        }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: data-structure/lazy-segment-tree/src/wrapper.rs
  requiredBy: []
  timestamp: '2025-09-06 15:04:09+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: data-structure/lazy-segment-tree/src/wrapper.rs
layout: document
title: Lazy Segment Tree (Wrapper)
---

## Description

## Reference
- 区間変更区間取得
    - [https://betrue12.hateblo.jp/entry/2020/09/22/194541](https://betrue12.hateblo.jp/entry/2020/09/22/194541)
    - [https://betrue12.hateblo.jp/entry/2020/09/23/005940](https://betrue12.hateblo.jp/entry/2020/09/23/005940)

- 区間等差数列加算
    - [https://atcoder.jp/contests/abc407/editorial/13118](https://atcoder.jp/contests/abc407/editorial/13118)
