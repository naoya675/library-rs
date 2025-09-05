---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/fenwick-tree-abstract/src/lib.rs
    title: Fenwick Tree (Abstract)
  - icon: ':heavy_check_mark:'
    path: tree/heavy-light-decomposition/src/lib.rs
    title: Heavy-Light Decomposition
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_D
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_D
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_D\n\
    \nuse proconio::input;\n\nuse fenwick_tree_abstract::FenwickTreeAbstract;\nuse\
    \ heavy_light_decomposition::HeavyLightDecomposition;\n\nfn main() {\n    std::thread::Builder::new()\n\
    \        .stack_size(64 * 1024 * 1024)\n        .spawn(actual_main)\n        .unwrap()\n\
    \        .join()\n        .unwrap();\n}\n\nfn actual_main() {\n    input! {\n\
    \        n: usize,\n    }\n    let mut hld = HeavyLightDecomposition::<usize>::new(n);\n\
    \    for i in 0..n {\n        input! { k: usize, c: [usize; k], }\n        for\
    \ c in c {\n            hld.add_edge(i, c, 0);\n            hld.add_edge(c, i,\
    \ 0);\n        }\n    }\n    hld.init(0);\n    let mut ft = FenwickTreeAbstract::<i64>::new(n,\
    \ |a, b| a + b, 0, |a| -a);\n    input! { q: usize, }\n    for _ in 0..q {\n \
    \       input! { query: usize, }\n        match query {\n            0 => {\n\
    \                input! { v: usize, w:i64, }\n                let index = hld.index(v);\n\
    \                ft.add(index.0, w);\n            }\n            1 => {\n    \
    \            input! { v: usize, }\n                let mut res = 0;\n        \
    \        hld.for_each_edge(0, v, |l, r| res += ft.sum(l, r));\n              \
    \  println!(\"{}\", res);\n            }\n            _ => unreachable!(),\n \
    \       }\n    }\n}\n"
  dependsOn:
  - data-structure/fenwick-tree-abstract/src/lib.rs
  - tree/heavy-light-decomposition/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/grl_5_d/src/main.rs
  requiredBy: []
  timestamp: '2025-08-21 20:48:10+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/aizu-online-judge/grl_5_d/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/grl_5_d/src/main.rs
- /verify/verification/aizu-online-judge/grl_5_d/src/main.rs.html
title: verification/aizu-online-judge/grl_5_d/src/main.rs
---
