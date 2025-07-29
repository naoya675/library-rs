---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: graph/ford-fulkerson/src/lib.rs
    title: Ford Fulkerson
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_6_A
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_6_A
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_6_A\n\
    \nuse proconio::input;\n\nuse ford_fulkerson::FordFulkerson;\n\nfn main() {\n\
    \    input! {\n        n: usize,\n        m: usize,\n        uvc: [(usize, usize,\
    \ i64); m],\n    }\n    let mut ff = FordFulkerson::new(n);\n    for (u, v, c)\
    \ in uvc {\n        ff.add_edge(u, v, c);\n    }\n    println!(\"{}\", ff.flow(0,\
    \ n - 1));\n}\n"
  dependsOn:
  - graph/ford-fulkerson/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/grl_6_a/src/main.rs
  requiredBy: []
  timestamp: '2025-05-26 22:58:00+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/aizu-online-judge/grl_6_a/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/grl_6_a/src/main.rs
- /verify/verification/aizu-online-judge/grl_6_a/src/main.rs.html
title: verification/aizu-online-judge/grl_6_a/src/main.rs
---
