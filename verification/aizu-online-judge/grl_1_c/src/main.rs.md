---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: graph/warshall-floyd/src/lib.rs
    title: Warshall Floyd
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_C
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_C
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_C\n\
    \nuse itertools::Itertools;\nuse proconio::input;\n\nuse warshall_floyd::WarshallFloyd;\n\
    \nfn main() {\n    input! {\n        v: usize,\n        e: usize,\n        std:\
    \ [(usize, usize, i64); e],\n    }\n    let mut wf = WarshallFloyd::new(v);\n\
    \    for (s, t, d) in std {\n        wf.add_edge(s, t, d);\n    }\n    let (cycle,\
    \ res) = wf.warshall_floyd();\n    if cycle {\n        println!(\"NEGATIVE CYCLE\"\
    );\n    } else {\n        for i in 0..v {\n            let res = res[i]\n    \
    \            .iter()\n                .map(|&res| if res < i64::MAX / 8 { res.to_string()\
    \ } else { \"INF\".to_string() })\n                .join(\" \");\n           \
    \ println!(\"{}\", res);\n        }\n    }\n}\n"
  dependsOn:
  - graph/warshall-floyd/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/grl_1_c/src/main.rs
  requiredBy: []
  timestamp: '2025-04-19 04:57:54+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/aizu-online-judge/grl_1_c/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/grl_1_c/src/main.rs
- /verify/verification/aizu-online-judge/grl_1_c/src/main.rs.html
title: verification/aizu-online-judge/grl_1_c/src/main.rs
---
