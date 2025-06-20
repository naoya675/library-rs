---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/union-find-with-potential/src/lib.rs
    title: data-structure/union-find-with-potential/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_B
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_B
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_B\n\
    \nuse proconio::input;\n\nuse union_find_with_potential::UnionFindWithPotential;\n\
    \nfn main() {\n    input! {\n        n: usize,\n        q: usize,\n    }\n   \
    \ let mut uf = UnionFindWithPotential::<i64>::new_default(n);\n    for _ in 0..q\
    \ {\n        input! { query: usize, }\n        match query {\n            0 =>\
    \ {\n                input! { x: usize, y: usize, z: i64, }\n                uf.merge(x,\
    \ y, z);\n            }\n            1 => {\n                input! { x: usize,\
    \ y: usize, }\n                if uf.same(x, y) {\n                    println!(\"\
    {}\", uf.diff(x, y));\n                } else {\n                    println!(\"\
    ?\");\n                }\n            }\n            _ => unreachable!(),\n  \
    \      }\n    }\n}\n"
  dependsOn:
  - data-structure/union-find-with-potential/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/dsl_1_b/src/main.rs
  requiredBy: []
  timestamp: '2025-05-29 20:17:30+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/aizu-online-judge/dsl_1_b/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/dsl_1_b/src/main.rs
- /verify/verification/aizu-online-judge/dsl_1_b/src/main.rs.html
title: verification/aizu-online-judge/dsl_1_b/src/main.rs
---
