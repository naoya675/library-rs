---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: graph/dijkstra/src/lib.rs
    title: Dijkstra
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_A
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_A
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_A\n\
    \nuse proconio::input;\n\nuse dijkstra::dijkstra;\nuse dijkstra::Edge;\n\nfn main()\
    \ {\n    input! {\n        v: usize,\n        e: usize,\n        r: usize,\n \
    \       std: [(usize, usize, i64); e],\n    }\n    let mut graph = vec![vec![];\
    \ v];\n    for (s, t, d) in std {\n        graph[s].push(Edge::new(t, d));\n \
    \   }\n    let res = dijkstra(v, &graph, r);\n    for i in 0..v {\n        if\
    \ res[i] < i64::MAX / 2 {\n            println!(\"{}\", res[i])\n        } else\
    \ {\n            println!(\"INF\");\n        }\n    }\n}\n"
  dependsOn:
  - graph/dijkstra/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/grl_1_a/src/main.rs
  requiredBy: []
  timestamp: '2024-09-23 04:04:04+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/aizu-online-judge/grl_1_a/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/grl_1_a/src/main.rs
- /verify/verification/aizu-online-judge/grl_1_a/src/main.rs.html
title: verification/aizu-online-judge/grl_1_a/src/main.rs
---
