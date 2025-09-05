---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: tree/heavy-light-decomposition/src/lib.rs
    title: Heavy-Light Decomposition
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_C
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_C
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_C\n\
    \nuse proconio::input;\n\nuse heavy_light_decomposition::HeavyLightDecomposition;\n\
    \nfn main() {\n    std::thread::Builder::new()\n        .stack_size(64 * 1024\
    \ * 1024)\n        .spawn(actual_main)\n        .unwrap()\n        .join()\n \
    \       .unwrap();\n}\n\nfn actual_main() {\n    input! {\n        n: usize,\n\
    \    }\n    let mut hld = HeavyLightDecomposition::<usize>::new(n);\n    for i\
    \ in 0..n {\n        input! { k: usize, c: [usize; k], }\n        for c in c {\n\
    \            hld.add_edge(i, c, 0);\n            hld.add_edge(c, i, 0);\n    \
    \    }\n    }\n    hld.init(0);\n    input! {\n        q: usize,\n        uv:\
    \ [(usize, usize); q],\n    }\n    for (u, v) in uv {\n        println!(\"{}\"\
    , hld.lca(u, v));\n    }\n}\n"
  dependsOn:
  - tree/heavy-light-decomposition/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/grl_5_c/src/main.rs
  requiredBy: []
  timestamp: '2025-09-05 20:18:54+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/aizu-online-judge/grl_5_c/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/grl_5_c/src/main.rs
- /verify/verification/aizu-online-judge/grl_5_c/src/main.rs.html
title: verification/aizu-online-judge/grl_5_c/src/main.rs
---
