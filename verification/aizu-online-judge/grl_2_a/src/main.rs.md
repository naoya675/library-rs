---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: graph/kruskal/src/lib.rs
    title: Kruskal
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_2_A
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_2_A
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_2_A\n\
    \nuse proconio::input;\n\nuse kruskal::minimum_spanning_tree;\n\nfn main() {\n\
    \    input! {\n        v: usize,\n        e: usize,\n        mut stw: [(usize,\
    \ usize, i64); e],\n    }\n    println!(\"{}\", minimum_spanning_tree(v, &mut\
    \ stw).0);\n}\n"
  dependsOn:
  - graph/kruskal/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/grl_2_a/src/main.rs
  requiredBy: []
  timestamp: '2026-01-01 00:11:18+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/aizu-online-judge/grl_2_a/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/grl_2_a/src/main.rs
- /verify/verification/aizu-online-judge/grl_2_a/src/main.rs.html
title: verification/aizu-online-judge/grl_2_a/src/main.rs
---
