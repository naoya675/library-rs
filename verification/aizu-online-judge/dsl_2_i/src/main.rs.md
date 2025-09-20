---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/lazy-segment-tree/src/lib.rs
    title: Lazy Segment Tree
  - icon: ':heavy_check_mark:'
    path: macro/query/src/lib.rs
    title: macro/query/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_I
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_I
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_I\n\
    \nuse proconio::input;\n\nuse lazy_segment_tree::LazySegmentTree;\n\nquery::define_query!\
    \ {\n    Query {\n        0 => Query0(s: usize, t: usize, x: i64),\n        1\
    \ => Query1(s: usize, t: usize),\n    }\n}\n\nfn main() {\n    input! {\n    \
    \    n: usize,\n        q: usize,\n        queries: [Query; q],\n    }\n    let\
    \ mut lst = LazySegmentTree::<(i64, i64), i64>::new(\n        n,\n        |x,\
    \ y| (x.0 + y.0, x.1 + y.1),\n        (0, 0),\n        |f, x| if f != i64::MAX\
    \ { (f * x.1, x.1) } else { x },\n        |f, g| if f == i64::MAX { g } else {\
    \ f },\n        i64::MAX,\n    );\n    lst.build(vec![(0, 1); n]);\n\n    for\
    \ query in queries {\n        match query {\n            Query0(s, t, x) => lst.apply(s,\
    \ t + 1, x),\n            Query1(s, t) => {\n                println!(\"{}\",\
    \ lst.prod(s, t + 1).0);\n            }\n        }\n    }\n}\n"
  dependsOn:
  - data-structure/lazy-segment-tree/src/lib.rs
  - macro/query/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/dsl_2_i/src/main.rs
  requiredBy: []
  timestamp: '2025-09-06 15:04:09+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/aizu-online-judge/dsl_2_i/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/dsl_2_i/src/main.rs
- /verify/verification/aizu-online-judge/dsl_2_i/src/main.rs.html
title: verification/aizu-online-judge/dsl_2_i/src/main.rs
---
