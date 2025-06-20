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
    title: Rolling Hash + Segment Tree
  - icon: ':heavy_check_mark:'
    path: tree/euler-tour/src/lib.rs
    title: tree/euler-tour/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dsl_2_a/src/main.rs
    title: verification/aizu-online-judge/dsl_2_a/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dsl_2_b/src/main.rs
    title: verification/aizu-online-judge/dsl_2_b/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/vertex_set_path_composite/src/main.rs
    title: verification/library-checker/vertex_set_path_composite/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://atcoder.jp/contests/abc223/editorial/2774
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use crate::SegmentTree;\n\npub struct RangeMinimumQuery;\n\nimpl RangeMinimumQuery\
    \ {\n    pub fn new(n: usize) -> SegmentTree<i64> {\n        SegmentTree::new(n,\
    \ |a, b| std::cmp::min(a, b), i64::MAX)\n    }\n}\n\npub struct RangeMaximumQuery;\n\
    \nimpl RangeMaximumQuery {\n    pub fn new(n: usize) -> SegmentTree<i64> {\n \
    \       SegmentTree::new(n, |a, b| std::cmp::max(a, b), i64::MIN)\n    }\n}\n\n\
    pub struct RangeSumQuery;\n\nimpl RangeSumQuery {\n    pub fn new(n: usize) ->\
    \ SegmentTree<i64> {\n        SegmentTree::new(n, |a, b| a + b, 0)\n    }\n}\n\
    \npub struct ParenthesisCheckQuery;\n\nimpl ParenthesisCheckQuery {\n    pub fn\
    \ new(n: usize) -> SegmentTree<(i64, i64)> {\n        SegmentTree::new(n, |a,\
    \ b| (a.0 + std::cmp::max(b.0 - a.1, 0), std::cmp::max(a.1 - b.0, 0) + b.1), (0,\
    \ 0))\n    }\n\n    pub fn new_build(n: usize, s: &Vec<char>) -> SegmentTree<(i64,\
    \ i64)> {\n        let mut st = SegmentTree::new(n, |a, b| (a.0 + std::cmp::max(b.0\
    \ - a.1, 0), std::cmp::max(a.1 - b.0, 0) + b.1), (0, 0));\n        for i in 0..s.len()\
    \ {\n            st.set(\n                i,\n                match s[i] {\n \
    \                   '(' => (0, 1),\n                    ')' => (1, 0),\n     \
    \               _ => unreachable!(),\n                },\n            );\n   \
    \     }\n        st\n    }\n}\n\n/*\n// reference: https://atcoder.jp/contests/abc223/editorial/2774\n\
    pub struct ParenthesisCheckQuery;\n\nimpl ParenthesisCheckQuery {\n    pub fn\
    \ new(n: usize) -> SegmentTree<(i64, i64)> {\n        SegmentTree::new(n, |a,\
    \ b| (std::cmp::min(a.0, a.1 + b.0), a.1 + b.1), (0, 0))\n    }\n\n    pub fn\
    \ new_build(n: usize, s: &Vec<char>) -> SegmentTree<(i64, i64)> {\n        let\
    \ mut st = SegmentTree::new(n, |a, b| (a.0 + std::cmp::max(b.0 - a.1, 0), std::cmp::max(a.1\
    \ - b.0, 0) + b.1), (0, 0));\n        for i in 0..s.len() {\n            st.set(\n\
    \                i,\n                match s[i] {\n                    '(' =>\
    \ (0, 0),\n                    ')' => (-1, -1),\n                    _ => unreachable!(),\n\
    \                },\n            );\n        }\n        st\n    }\n}\n*/\n\npub\
    \ struct RangeCompositeQuery;\n\nimpl RangeCompositeQuery {\n    pub fn new(n:\
    \ usize) -> SegmentTree<(i64, i64)> {\n        SegmentTree::new(n, |a, b| (a.0\
    \ * b.0, a.1 * b.0 + b.1), (1, 0))\n    }\n}\n"
  dependsOn:
  - data-structure/segment-tree/src/lib.rs
  isVerificationFile: false
  path: data-structure/segment-tree/src/wrapper.rs
  requiredBy:
  - string/rolling-hash-segment-tree/src/lib.rs
  - data-structure/segment-tree/src/lib.rs
  - tree/euler-tour/src/lib.rs
  timestamp: '2025-06-07 00:31:50+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/library-checker/vertex_set_path_composite/src/main.rs
  - verification/aizu-online-judge/dsl_2_b/src/main.rs
  - verification/aizu-online-judge/dsl_2_a/src/main.rs
documentation_of: data-structure/segment-tree/src/wrapper.rs
layout: document
redirect_from:
- /library/data-structure/segment-tree/src/wrapper.rs
- /library/data-structure/segment-tree/src/wrapper.rs.html
title: data-structure/segment-tree/src/wrapper.rs
---
