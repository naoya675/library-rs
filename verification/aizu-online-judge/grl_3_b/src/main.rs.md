---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: graph/low-link/src/lib.rs
    title: Low Link
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_3_B
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_3_B
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_3_B\n\
    \nuse proconio::input;\n\nuse low_link::LowLink;\n\nfn main() {\n    input! {\n\
    \        v: usize,\n        e: usize,\n        st: [(usize, usize); e],\n    }\n\
    \    let mut ll = LowLink::new(v);\n    st.iter().for_each(|&(s, t)| {\n     \
    \   ll.add_edge(s, t);\n        ll.add_edge(t, s);\n    });\n    ll.build();\n\
    \    let mut b = ll.bridge().iter().map(|&(s, t)| if s < t { (s, t) } else { (t,\
    \ s) }).collect::<Vec<_>>();\n    b.sort();\n\n    for (s, t) in &b {\n      \
    \  println!(\"{} {}\", s, t);\n    }\n}\n"
  dependsOn:
  - graph/low-link/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/grl_3_b/src/main.rs
  requiredBy: []
  timestamp: '2026-01-01 18:54:13+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/aizu-online-judge/grl_3_b/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/grl_3_b/src/main.rs
- /verify/verification/aizu-online-judge/grl_3_b/src/main.rs.html
title: verification/aizu-online-judge/grl_3_b/src/main.rs
---
