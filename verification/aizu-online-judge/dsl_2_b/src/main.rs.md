---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/fenwick-tree/src/lib.rs
    title: data-structure/fenwick-tree/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_B
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_B
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_B\n\
    \nuse proconio::input;\n\n// use segment_tree::SegmentTree;\nuse fenwick_tree::FenwickTree;\n\
    \nfn main() {\n    input! {\n        n: usize,\n        q: usize,\n    }\n   \
    \ // let mut st = SegmentTree::<i64>::new(n, |a, b| a + b, 0);\n    let mut ft\
    \ = FenwickTree::<i64>::new(n);\n    for _ in 0..q {\n        input! { query:\
    \ usize, }\n        match query {\n            0 => {\n                input!\
    \ {\n                    x: usize,\n                    y: i64,\n            \
    \    }\n                ft.add(x - 1, y);\n            }\n            1 => {\n\
    \                input! {\n                    x: usize,\n                   \
    \ y: usize,\n                }\n                println!(\"{}\", ft.sum(x - 1,\
    \ y));\n            }\n            _ => unreachable!(),\n        }\n    }\n}\n"
  dependsOn:
  - data-structure/fenwick-tree/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/dsl_2_b/src/main.rs
  requiredBy: []
  timestamp: '2024-09-23 04:03:48+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/aizu-online-judge/dsl_2_b/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/dsl_2_b/src/main.rs
- /verify/verification/aizu-online-judge/dsl_2_b/src/main.rs.html
title: verification/aizu-online-judge/dsl_2_b/src/main.rs
---
