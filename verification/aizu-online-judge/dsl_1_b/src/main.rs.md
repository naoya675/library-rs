---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/weighted-union-find/src/lib.rs
    title: Weighted Union Find
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_B
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_B
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_B\n\
    \nuse proconio::input;\n\nuse weighted_union_find::WeightedUnionFind;\n\nfn main()\
    \ {\n    input! {\n        n: usize,\n        q: usize,\n    }\n    let mut wuf\
    \ = WeightedUnionFind::<i64>::new(n);\n    for _ in 0..q {\n        input! { query:\
    \ usize, }\n        match query {\n            0 => {\n                input!\
    \ { x: usize, y: usize, z: i64, }\n                wuf.merge(x, y, z);\n     \
    \       }\n            1 => {\n                input! { x: usize, y: usize, }\n\
    \                if wuf.same(x, y) {\n                    println!(\"{}\", wuf.diff(x,\
    \ y));\n                } else {\n                    println!(\"?\");\n     \
    \           }\n            }\n            _ => unreachable!(),\n        }\n  \
    \  }\n}\n"
  dependsOn:
  - data-structure/weighted-union-find/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/dsl_1_b/src/main.rs
  requiredBy: []
  timestamp: '2024-04-12 20:06:42+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/aizu-online-judge/dsl_1_b/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/dsl_1_b/src/main.rs
- /verify/verification/aizu-online-judge/dsl_1_b/src/main.rs.html
title: verification/aizu-online-judge/dsl_1_b/src/main.rs
---
