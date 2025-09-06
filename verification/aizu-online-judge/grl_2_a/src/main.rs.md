---
data:
  _extendedDependsOn:
  - icon: ':x:'
    path: graph/kruskal/src/lib.rs
    title: Kruskal
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_2_A
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_2_A
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_2_A\n\
    \nuse proconio::input;\n\nuse kruskal::Kruskal;\n\nfn main() {\n    input! {\n\
    \        v: usize,\n        e: usize,\n        stw: [(usize, usize, i64); e],\n\
    \    }\n    let mut krs = Kruskal::new(v);\n    stw.iter().for_each(|&(s, t, w)|\
    \ krs.add_edge(s, t, w));\n\n    println!(\"{}\", krs.minimum_spanning_tree().0);\n\
    }\n"
  dependsOn:
  - graph/kruskal/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/grl_2_a/src/main.rs
  requiredBy: []
  timestamp: '2025-09-06 15:04:09+09:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: verification/aizu-online-judge/grl_2_a/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/grl_2_a/src/main.rs
- /verify/verification/aizu-online-judge/grl_2_a/src/main.rs.html
title: verification/aizu-online-judge/grl_2_a/src/main.rs
---
