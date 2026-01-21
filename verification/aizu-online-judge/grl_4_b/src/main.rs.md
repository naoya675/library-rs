---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: graph/topological-sort/src/lib.rs
    title: graph/topological-sort/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    IGNORE: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_4_B
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_4_B
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: IGNORE https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_4_B\n\
    \nuse itertools::Itertools;\nuse proconio::input;\n\nuse topological_sort::topological_sort;\n\
    \nfn main() {\n    input! {\n        v: usize,\n        e: usize,\n        st:\
    \ [(usize, usize); e],\n    }\n    let mut graph = vec![vec![]; v];\n    st.iter().for_each(|&(s,\
    \ t)| graph[s].push(t));\n\n    println!(\"{}\", topological_sort(v, &graph).iter().join(\"\
    \\n\"));\n}\n"
  dependsOn:
  - graph/topological-sort/src/lib.rs
  isVerificationFile: false
  path: verification/aizu-online-judge/grl_4_b/src/main.rs
  requiredBy: []
  timestamp: '2026-01-01 00:11:18+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: verification/aizu-online-judge/grl_4_b/src/main.rs
layout: document
redirect_from:
- /library/verification/aizu-online-judge/grl_4_b/src/main.rs
- /library/verification/aizu-online-judge/grl_4_b/src/main.rs.html
title: verification/aizu-online-judge/grl_4_b/src/main.rs
---
