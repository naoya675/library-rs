---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: dynamic-programming/rerooting/src/lib.rs
    title: Rerooting
  - icon: ':heavy_check_mark:'
    path: dynamic-programming/rerooting/src/wrapper.rs
    title: dynamic-programming/rerooting/src/wrapper.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_A
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_A
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_A\n\
    \nuse rerooting::Rerooting;\n\nfn main() {\n    std::thread::Builder::new()\n\
    \        .stack_size(64 * 1024 * 1024)\n        .spawn(actual_main)\n        .unwrap()\n\
    \        .join()\n        .unwrap();\n}\n\nfn actual_main() {\n    proconio::input!\
    \ {\n        n: usize,\n        stw: [(usize, usize, usize); n - 1],\n    }\n\
    \    let merge = |a: usize, b: usize| std::cmp::max(a, b);\n    let e = || 0_usize;\n\
    \    let leaf = || 0_usize;\n    let apply = |a: usize, _: usize, _: usize, w:\
    \ usize| -> usize { a + w };\n    let mut g = Rerooting::<usize, usize, _, _,\
    \ _, _>::new(n, merge, e, leaf, apply);\n    for (s, t, w) in stw {\n        g.add_edge(s,\
    \ t, w);\n        g.add_edge(t, s, w);\n    }\n    let res = g.run();\n    println!(\"\
    {}\", res.iter().max().unwrap());\n}\n"
  dependsOn:
  - dynamic-programming/rerooting/src/lib.rs
  - dynamic-programming/rerooting/src/wrapper.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/grl_5_a/src/main.rs
  requiredBy: []
  timestamp: '2025-05-01 00:33:42+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/aizu-online-judge/grl_5_a/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/grl_5_a/src/main.rs
- /verify/verification/aizu-online-judge/grl_5_a/src/main.rs.html
title: verification/aizu-online-judge/grl_5_a/src/main.rs
---
