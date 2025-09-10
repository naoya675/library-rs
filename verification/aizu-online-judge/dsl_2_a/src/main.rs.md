---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: data-structure/segment-tree/src/lib.rs
    title: Segment Tree
  - icon: ':question:'
    path: macro/query/src/lib.rs
    title: macro/query/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A\n\
    \nuse proconio::input;\n\nuse segment_tree::SegmentTree;\n\nquery::define_query!\
    \ {\n    Query {\n        0 => Q0(x: usize, y: i64),\n        1 => Q1(x: usize,\
    \ y: usize),\n    }\n}\n\nfn main() {\n    input! {\n        n: usize,\n     \
    \   q: usize,\n        queries: [Query; q],\n    }\n    let mut st = SegmentTree::<i64>::new(n,\
    \ |x, y| std::cmp::min(x, y), i64::MAX);\n    st.build(vec![(1 << 31) - 1; n]);\n\
    \n    for query in queries {\n        match query {\n            Q0(x, y) => st.set(x,\
    \ y),\n            Q1(x, y) => {\n                println!(\"{}\", st.prod(x,\
    \ y + 1));\n            }\n        }\n    }\n}\n"
  dependsOn:
  - data-structure/segment-tree/src/lib.rs
  - macro/query/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/dsl_2_a/src/main.rs
  requiredBy: []
  timestamp: '2025-09-06 15:04:09+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/aizu-online-judge/dsl_2_a/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/dsl_2_a/src/main.rs
- /verify/verification/aizu-online-judge/dsl_2_a/src/main.rs.html
title: verification/aizu-online-judge/dsl_2_a/src/main.rs
---
