---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: graph/maxflow/src/lib.rs
    title: Maxflow
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_7_A
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_7_A
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_7_A\n\
    \nuse proconio::input;\n\nuse maxflow::Maxflow;\n\nfn main() {\n    input! {\n\
    \        x: usize,\n        y: usize,\n        e: usize,\n        xy: [(usize,\
    \ usize); e],\n    }\n    let xy = xy.iter().map(|&(xi, yi)| (xi, x + yi)).collect::<Vec<_>>();\n\
    \n    let source = x + y;\n    let sink = source + 1;\n    let mut mf = Maxflow::<i64>::new(sink\
    \ + 1);\n    (0..x).for_each(|i| {\n        mf.add_edge(source, i, 1);\n    });\n\
    \    xy.iter().for_each(|&(x, y)| {\n        mf.add_edge(x, y, 1);\n    });\n\
    \    (x..x + y).for_each(|i| {\n        mf.add_edge(i, sink, 1);\n    });\n\n\
    \    println!(\"{}\", mf.flow(source, sink));\n}\n"
  dependsOn:
  - graph/maxflow/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/grl_7_a_maxflow/src/main.rs
  requiredBy: []
  timestamp: '2026-01-01 00:11:18+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/aizu-online-judge/grl_7_a_maxflow/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/grl_7_a_maxflow/src/main.rs
- /verify/verification/aizu-online-judge/grl_7_a_maxflow/src/main.rs.html
title: verification/aizu-online-judge/grl_7_a_maxflow/src/main.rs
---
