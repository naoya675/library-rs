---
data:
  _extendedDependsOn:
  - icon: ':x:'
    path: tree/rerooting/src/lib.rs
    title: Rerooting
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_A
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_A
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_A\n\
    \nuse proconio::input;\n\nuse rerooting::Rerooting;\n\nfn main() {\n    std::thread::Builder::new()\n\
    \        .stack_size(64 * 1024 * 1024)\n        .spawn(actual_main)\n        .unwrap()\n\
    \        .join()\n        .unwrap();\n}\n\nfn actual_main() {\n    input! {\n\
    \        n: usize,\n        stw: [(usize, usize, usize); n - 1],\n    }\n    let\
    \ mut g = Rerooting::<usize, usize, _, _, _, _>::new(\n        n,\n        |a:\
    \ usize, b: usize| std::cmp::max(a, b),\n        || 0,\n        || 0,\n      \
    \  |a: usize, _: usize, _: usize, w: usize| a + w,\n    );\n    stw.iter().for_each(|&(s,\
    \ t, w)| {\n        g.add_edge(s, t, w);\n        g.add_edge(t, s, w);\n    });\n\
    \n    let res = g.run();\n    println!(\"{}\", res.iter().max().unwrap());\n}\n"
  dependsOn:
  - tree/rerooting/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/grl_5_a/src/main.rs
  requiredBy: []
  timestamp: '2025-09-06 15:04:09+09:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: verification/aizu-online-judge/grl_5_a/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/grl_5_a/src/main.rs
- /verify/verification/aizu-online-judge/grl_5_a/src/main.rs.html
title: verification/aizu-online-judge/grl_5_a/src/main.rs
---
