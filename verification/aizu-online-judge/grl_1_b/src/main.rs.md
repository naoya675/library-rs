---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: graph/bellman-ford/src/lib.rs
    title: Bellman Ford
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_B
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_B
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_B\n\
    \nuse proconio::input;\n\nuse bellman_ford::bellman_ford;\nuse bellman_ford::Edge;\n\
    \nfn main() {\n    input! {\n        v: usize,\n        e: usize,\n        r:\
    \ usize,\n        std: [(usize, usize, i64); e],\n    }\n    let mut edge = vec![];\n\
    \    for (s, t, d) in std {\n        edge.push(Edge::new(s, t, d));\n    }\n \
    \   let (cycle, res) = bellman_ford(v, &edge, r);\n    if cycle {\n        println!(\"\
    NEGATIVE CYCLE\");\n    } else {\n        for i in 0..v {\n            if res[i]\
    \ < i64::MAX / 2 {\n                println!(\"{}\", res[i])\n            } else\
    \ {\n                println!(\"INF\");\n            }\n        }\n    }\n}\n"
  dependsOn:
  - graph/bellman-ford/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/grl_1_b/src/main.rs
  requiredBy: []
  timestamp: '2024-04-14 21:35:32+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/aizu-online-judge/grl_1_b/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/grl_1_b/src/main.rs
- /verify/verification/aizu-online-judge/grl_1_b/src/main.rs.html
title: verification/aizu-online-judge/grl_1_b/src/main.rs
---
