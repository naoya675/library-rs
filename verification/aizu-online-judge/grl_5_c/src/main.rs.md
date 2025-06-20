---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: tree/euler-tour/src/lib.rs
    title: tree/euler-tour/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_C
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_C
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_C\n\
    \nuse proconio::input;\n\nuse euler_tour::EulerTour;\n\nfn main() {\n    std::thread::Builder::new()\n\
    \        .stack_size(64 * 1024 * 1024)\n        .spawn(actual_main)\n        .unwrap()\n\
    \        .join()\n        .unwrap();\n}\n\nfn actual_main() {\n    input! {\n\
    \        n: usize,\n    }\n    let mut et = EulerTour::<usize>::new(n);\n    for\
    \ i in 0..n {\n        input! { k: usize, c: [usize; k], }\n        for c in c\
    \ {\n            et.add_edge(i, c, 0);\n            et.add_edge(c, i, 0);\n  \
    \      }\n    }\n    et.init(0);\n    input! { q: usize, }\n    for _ in 0..q\
    \ {\n        input! { u: usize, v: usize, }\n        println!(\"{}\", et.lca(u,\
    \ v));\n    }\n}\n"
  dependsOn:
  - tree/euler-tour/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/grl_5_c/src/main.rs
  requiredBy: []
  timestamp: '2025-06-21 02:45:26+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/aizu-online-judge/grl_5_c/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/grl_5_c/src/main.rs
- /verify/verification/aizu-online-judge/grl_5_c/src/main.rs.html
title: verification/aizu-online-judge/grl_5_c/src/main.rs
---
