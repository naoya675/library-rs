---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/segment-tree/src/lib.rs
    title: Segment Tree
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: data-structure/segment-tree/src/lib.rs
    title: Segment Tree
  - icon: ':heavy_check_mark:'
    path: string/rolling-hash-segment-tree/src/lib.rs
    title: Rolling Hash (Rabin-Karp)
  - icon: ':heavy_check_mark:'
    path: tree/euler-tour/src/lib.rs
    title: Euler Tour
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dsl_2_a/src/main.rs
    title: verification/aizu-online-judge/dsl_2_a/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dsl_2_b/src/main.rs
    title: verification/aizu-online-judge/dsl_2_b/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/point_set_range_composite/src/main.rs
    title: verification/library-checker/point_set_range_composite/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/vertex_set_path_composite/src/main.rs
    title: verification/library-checker/vertex_set_path_composite/src/main.rs
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
  code: "use crate::SegmentTree;\n\npub struct RangeMinimumQuery;\nimpl RangeMinimumQuery\
    \ {\n    pub fn new(n: usize) -> SegmentTree<i64> {\n        SegmentTree::new(n,\
    \ |x, y| std::cmp::min(x, y), i64::MAX)\n    }\n}\n\npub struct RangeMaximumQuery;\n\
    impl RangeMaximumQuery {\n    pub fn new(n: usize) -> SegmentTree<i64> {\n   \
    \     SegmentTree::new(n, |x, y| std::cmp::max(x, y), i64::MIN)\n    }\n}\n\n\
    pub struct RangeSumQuery;\nimpl RangeSumQuery {\n    pub fn new(n: usize) -> SegmentTree<i64>\
    \ {\n        SegmentTree::new(n, |x, y| x + y, 0)\n    }\n}\n\npub struct RangeCompositeQuery;\n\
    impl RangeCompositeQuery {\n    pub fn new(n: usize) -> SegmentTree<(i64, i64)>\
    \ {\n        SegmentTree::new(n, |x, y| (x.0 * y.0, x.1 * y.0 + y.1), (1, 0))\n\
    \    }\n}\n\npub struct ParenthesisCheckQuery;\nimpl ParenthesisCheckQuery {\n\
    \    pub fn new(n: usize) -> SegmentTree<(i64, i64)> {\n        SegmentTree::new(n,\
    \ |x, y| (x.0 + std::cmp::max(y.0 - x.1, 0), std::cmp::max(x.1 - y.0, 0) + y.1),\
    \ (0, 0))\n    }\n\n    pub fn new_build(n: usize, s: &Vec<char>) -> SegmentTree<(i64,\
    \ i64)> {\n        let mut st = Self::new(n);\n        let s = s\n           \
    \ .iter()\n            .map(|&s| match s {\n                '(' => (0, 1),\n \
    \               ')' => (1, 0),\n                _ => unreachable!(),\n       \
    \     })\n            .collect::<Vec<_>>();\n        st.build(&s);\n        st\n\
    \    }\n}\n\n/*\npub struct ParenthesisCheckQuery;\nimpl ParenthesisCheckQuery\
    \ {\n    pub fn new(n: usize) -> SegmentTree<(i64, i64)> {\n        SegmentTree::new(n,\
    \ |x, y| (std::cmp::min(x.0, x.1 + y.0), x.1 + y.1), (0, 0))\n    }\n\n    pub\
    \ fn new_build(n: usize, s: &Vec<char>) -> SegmentTree<(i64, i64)> {\n       \
    \ let mut st = Self::new(n);\n        let s = s\n            .iter()\n       \
    \     .map(|&s| match s {\n                '(' => (0, 1),\n                ')'\
    \ => (-1, -1),\n                _ => unreachable!(),\n            })\n       \
    \     .collect::<Vec<_>>();\n        st.build(&s);\n        st\n    }\n}\n*/\n\
    \npub struct IntervalCountQuery;\nimpl IntervalCountQuery {\n    pub fn new(n:\
    \ usize) -> SegmentTree<(usize, usize, usize)> {\n        SegmentTree::new(\n\
    \            n,\n            |x, y| {\n                if (x.0, x.1) == (2, 2)\
    \ {\n                    return y;\n                }\n                if (y.0,\
    \ y.1) == (2, 2) {\n                    return x;\n                }\n       \
    \         (x.0, y.1, x.2 + y.2 - if x.1 == y.0 && y.0 == 1 { 1 } else { 0 })\n\
    \            },\n            (2, 2, 0),\n        )\n    }\n\n    pub fn new_build(n:\
    \ usize, s: &Vec<usize>) -> SegmentTree<(usize, usize, usize)> {\n        let\
    \ mut st = Self::new(n);\n        let s = s\n            .iter()\n           \
    \ .map(|&s| match s {\n                0 => (0, 0, 0),      // Black\n       \
    \         1 => (1, 1, 1),      // White\n                _ => unreachable!(),\
    \ // Invalid\n            })\n            .collect::<Vec<_>>();\n        st.build(&s);\n\
    \        st\n    }\n}\n\npub mod range_maximum_subarray_sum_query {\n    use crate::SegmentTree;\n\
    \n    #[derive(Debug, Clone, Copy)]\n    pub struct S {\n        prefix: i64,\n\
    \        suffix: i64,\n        subarray: i64,\n        whole: i64,\n    }\n  \
    \  impl S {\n        fn new(x: i64) -> Self {\n            Self {\n          \
    \      prefix: x,\n                suffix: x,\n                subarray: x,\n\
    \                whole: x,\n            }\n        }\n        fn op(lhs: Self,\
    \ rhs: Self) -> Self {\n            Self {\n                prefix: lhs.prefix.max(rhs.prefix\
    \ + lhs.whole),\n                suffix: rhs.suffix.max(lhs.suffix + rhs.whole),\n\
    \                subarray: lhs.subarray.max(rhs.subarray).max(lhs.suffix + rhs.prefix),\n\
    \                whole: lhs.whole + rhs.whole,\n            }\n        }\n   \
    \ }\n\n    pub struct RangeMaximumSubarraySumQuery;\n    impl RangeMaximumSubarraySumQuery\
    \ {\n        pub fn new(n: usize) -> SegmentTree<S> {\n            SegmentTree::new(n,\
    \ S::op, S::new(0))\n        }\n    }\n}\n"
  dependsOn:
  - data-structure/segment-tree/src/lib.rs
  isVerificationFile: false
  path: data-structure/segment-tree/src/wrapper.rs
  requiredBy:
  - string/rolling-hash-segment-tree/src/lib.rs
  - data-structure/segment-tree/src/lib.rs
  - tree/euler-tour/src/lib.rs
  timestamp: '2026-01-01 00:11:18+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/dsl_2_b/src/main.rs
  - verification/aizu-online-judge/dsl_2_a/src/main.rs
  - verification/library-checker/point_set_range_composite/src/main.rs
  - verification/library-checker/vertex_set_path_composite/src/main.rs
documentation_of: data-structure/segment-tree/src/wrapper.rs
layout: document
title: Segment Tree (Wrapper)
---

<!--## Description-->

## Reference
- Parenthesis Check
    - [https://atcoder.jp/contests/abc223/editorial/2774](https://atcoder.jp/contests/abc223/editorial/2774)

- Range Maximum Subarray Sum (Prefix Sum, Suffix Sum)
    - [https://atcoder.jp/contests/abc175/editorial/4722](https://atcoder.jp/contests/abc175/editorial/4722)
    - [https://atcoder.jp/contests/abc175/submissions/34409729](https://atcoder.jp/contests/abc175/submissions/34409729)
    - [https://atcoder.jp/contests/abc415/editorial/13496](https://atcoder.jp/contests/abc415/editorial/13496)
    - [https://atcoder.jp/contests/abc415/submissions/67761786](https://atcoder.jp/contests/abc415/submissions/67761786)
    - Kadane's algorithm

- Interval Count
    - [https://atcoder.jp/contests/abc411/editorial/13379](https://atcoder.jp/contests/abc411/editorial/13379)
    - [https://atcoder.jp/contests/abc411/submissions/67026433](https://atcoder.jp/contests/abc411/submissions/67026433)
