---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/lazy-segment-tree/src/lib.rs
    title: Lazy Segment Tree
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_F
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_F
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_F\n\
    \nuse proconio::input;\nuse std::cmp::min;\n\nuse lazy_segment_tree::LazySegmentTree;\n\
    \nfn main() {\n    input! {\n        n: usize,\n        q: usize,\n    }\n   \
    \ let mut lst = LazySegmentTree::<i64, i64>::new(\n        n,\n        |a, b|\
    \ min(a, b),\n        1 << 31,\n        |a, b| if a == -1 { b } else { a },\n\
    \        |a, b| if a == -1 { b } else { a },\n        -1,\n    );\n    lst.build(vec![(1\
    \ << 31) - 1; n]);\n    for _ in 0..q {\n        input! {\n            query:\
    \ usize,\n        }\n        match query {\n            0 => {\n             \
    \   input! {\n                    s: usize,\n                    t: usize,\n \
    \                   x: i64,\n                }\n                lst.apply(s, t\
    \ + 1, x);\n            }\n            1 => {\n                input! {\n    \
    \                s: usize,\n                    t: usize,\n                }\n\
    \                println!(\"{}\", lst.prod(s, t + 1));\n            }\n      \
    \      _ => unreachable!(),\n        }\n    }\n}\n"
  dependsOn:
  - data-structure/lazy-segment-tree/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/dsl_2_f/src/main.rs
  requiredBy: []
  timestamp: '2024-03-29 23:59:42+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/aizu-online-judge/dsl_2_f/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/dsl_2_f/src/main.rs
- /verify/verification/aizu-online-judge/dsl_2_f/src/main.rs.html
title: verification/aizu-online-judge/dsl_2_f/src/main.rs
---
