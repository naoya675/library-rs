---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/segment-tree/src/lib.rs
    title: Segment Tree
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_B
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_B
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_B\n\
    \nuse proconio::input;\n\nuse segment_tree::SegmentTree;\n\nfn main() {\n    input!\
    \ {\n        n: usize,\n        q: usize,\n    }\n    let mut st = SegmentTree::<i64>::new(n,\
    \ |a, b| a + b, 0);\n    for _ in 0..q {\n        input! { query: usize, }\n \
    \       match query {\n            0 => {\n                input! { x: usize,\
    \ y: i64, }\n                st.apply(x - 1, y);\n            }\n            1\
    \ => {\n                input! { x: usize, y: usize, }\n                println!(\"\
    {}\", st.prod(x - 1, y));\n            }\n            _ => unreachable!(),\n \
    \       }\n    }\n}\n"
  dependsOn:
  - data-structure/segment-tree/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/dsl_2_b/src/main.rs
  requiredBy: []
  timestamp: '2025-08-21 20:46:40+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/aizu-online-judge/dsl_2_b/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/dsl_2_b/src/main.rs
- /verify/verification/aizu-online-judge/dsl_2_b/src/main.rs.html
title: verification/aizu-online-judge/dsl_2_b/src/main.rs
---
