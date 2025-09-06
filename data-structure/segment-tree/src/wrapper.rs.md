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
  code: "pub struct RangeMinimumQuery;\nimpl RangeMinimumQuery {\n    pub fn new(n:\
    \ usize) -> SegmentTree<i64> {\n        SegmentTree::new(n, |x, y| std::cmp::min(x,\
    \ y), i64::MAX)\n    }\n}\n\npub struct RangeMaximumQuery;\nimpl RangeMaximumQuery\
    \ {\n    pub fn new(n: usize) -> SegmentTree<i64> {\n        SegmentTree::new(n,\
    \ |x, y| std::cmp::max(x, y), i64::MIN)\n    }\n}\n\npub struct RangeSumQuery;\n\
    impl RangeSumQuery {\n    pub fn new(n: usize) -> SegmentTree<i64> {\n       \
    \ SegmentTree::new(n, |x, y| x + y, 0)\n    }\n}\n\npub struct RangeCompositeQuery;\n\
    impl RangeCompositeQuery {\n    pub fn new(n: usize) -> SegmentTree<(i64, i64)>\
    \ {\n        SegmentTree::new(n, |x, y| (x.0 * y.0, x.1 * y.0 + y.1), (1, 0))\n\
    \    }\n}\n\npub struct ParenthesisCheckQuery;\nimpl ParenthesisCheckQuery {\n\
    \    pub fn new(n: usize) -> SegmentTree<(i64, i64)> {\n        SegmentTree::new(n,\
    \ |x, y| (x.0 + std::cmp::max(y.0 - x.1, 0), std::cmp::max(x.1 - y.0, 0) + y.1),\
    \ (0, 0))\n    }\n\n    pub fn new_build(n: usize, s: &Vec<char>) -> SegmentTree<(i64,\
    \ i64)> {\n        let mut st = Self::new(n);\n        let s = s\n           \
    \ .iter()\n            .map(|&s| match s {\n                '(' => (0, 1),\n \
    \               ')' => (1, 0),\n                _ => unreachable!(),\n       \
    \     })\n            .collect::<Vec<_>>();\n        st.build(s);\n        st\n\
    \    }\n}\n\n/*\npub struct ParenthesisCheckQuery;\nimpl ParenthesisCheckQuery\
    \ {\n    pub fn new(n: usize) -> SegmentTree<(i64, i64)> {\n        SegmentTree::new(n,\
    \ |x, y| (std::cmp::min(x.0, x.1 + y.0), x.1 + y.1), (0, 0))\n    }\n\n    pub\
    \ fn new_build(n: usize, s: &Vec<char>) -> SegmentTree<(i64, i64)> {\n       \
    \ let mut st = Self::new(n);\n        let s = s\n            .iter()\n       \
    \     .map(|&s| match s {\n                '(' => (0, 1),\n                ')'\
    \ => (-1, -1),\n                _ => unreachable!(),\n            })\n       \
    \     .collect::<Vec<_>>();\n        st.build(s);\n        st\n    }\n}\n*/\n\n\
    pub struct IntervalCountQuery;\nimpl IntervalCountQuery {\n    pub fn new(n: usize)\
    \ -> SegmentTree<(usize, usize, usize)> {\n        SegmentTree::new(\n       \
    \     n,\n            |x, y| {\n                if (x.0, x.1) == (2, 2) {\n  \
    \                  return b;\n                }\n                if (y.0, y.1)\
    \ == (2, 2) {\n                    return a;\n                }\n            \
    \    (x.0, y.1, x.2 + y.2 - if x.1 == y.0 && y.0 == 1 { 1 } else { 0 })\n    \
    \        },\n            (2, 2, 0),\n        )\n    }\n\n    pub fn new_build(n:\
    \ usize, s: &Vec<usize>) -> SegmentTree<(usize, usize, usize)> {\n        let\
    \ mut st = Self::new(n);\n        let s = s\n            .iter()\n           \
    \ .map(|&s| match s {\n                0 => (0, 0, 0),      // Black\n       \
    \         1 => (1, 1, 1),      // White\n                _ => unreachable!(),\
    \ // Invalid\n            })\n            .collect::<Vec<_>>();\n        st.build(s);\n\
    \        st\n    }\n}\n\npub mod RangeMaximumSubarraySumQuery {\n    #[derive(Debug,\
    \ Clone, Copy)]\n    pub struct S {\n        prefix: i64,\n        suffix: i64,\n\
    \        subarray: i64,\n        whole: i64,\n    }\n    impl S {\n        fn\
    \ new(x: i64) -> Self {\n            Self {\n                prefix: x,\n    \
    \            suffix: x,\n                subarray: x,\n                whole:\
    \ x,\n            }\n        }\n        fn op(lhs: Self, rhs: Self) -> Self {\n\
    \            Self {\n                prefix: lhs.prefix.max(rhs.prefix + lhs.whole),\n\
    \                suffix: rhs.suffix.max(lhs.suffix + rhs.whole),\n           \
    \     subarray: lhs.subarray.max(rhs.subarray).max(lhs.suffix + rhs.prefix),\n\
    \                whole: lhs.whole + rhs.whole,\n            }\n        }\n   \
    \ }\n\n    pub struct RangeMaximumSubarraySumQuery;\n    impl RangeMaximumSubarraySumQuery\
    \ {\n        pub fn new(n: usize) -> SegmentTree<S> {\n            SegmentTree::new(n,\
    \ S::op, S::new(0))\n        }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: data-structure/segment-tree/src/wrapper.rs
  requiredBy: []
  timestamp: '2025-09-06 15:04:09+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: data-structure/segment-tree/src/wrapper.rs
layout: document
title: Segment Tree (Wrapper)
---

## Description
- Parenthesis Check
    - [https://atcoder.jp/contests/abc223/editorial/2774](https://atcoder.jp/contests/abc223/editorial/2774)

- Interval Count
    - [https://atcoder.jp/contests/abc411/editorial/13379](https://atcoder.jp/contests/abc411/editorial/13379)
    - [https://atcoder.jp/contests/abc411/submissions/67026433](https://atcoder.jp/contests/abc411/submissions/67026433)

- Range Maximum Subarray Sum (Prefix Sum, Suffix Sum)　
    - [https://atcoder.jp/contests/abc175/editorial/4722](https://atcoder.jp/contests/abc175/editorial/4722)
    - [https://atcoder.jp/contests/abc175/submissions/34409729](https://atcoder.jp/contests/abc175/submissions/34409729)
    - [https://atcoder.jp/contests/abc415/editorial/13496](https://atcoder.jp/contests/abc415/editorial/13496)
    - [https://atcoder.jp/contests/abc415/submissions/67761786](https://atcoder.jp/contests/abc415/submissions/67761786)
    - Kadane's algorithm
