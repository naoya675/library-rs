---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/fenwick-tree-abstract/src/lib.rs
    title: data-structure/fenwick-tree-abstract/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: tree/euler-tour/src/lib.rs
    title: tree/euler-tour/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_D
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_D
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_D\n\
    \nuse proconio::input;\n\nuse euler_tour::EulerTour;\nuse fenwick_tree_abstract::FenwickTreeAbstract;\n\
    \nfn main() {\n    std::thread::Builder::new()\n        .stack_size(64 * 1024\
    \ * 1024)\n        .spawn(actual_main)\n        .unwrap()\n        .join()\n \
    \       .unwrap();\n}\n\nfn actual_main() {\n    input! {\n        n: usize,\n\
    \    }\n    let mut et = EulerTour::<usize>::new(n);\n    for i in 0..n {\n  \
    \      input! { k: usize, c: [usize; k], }\n        for c in c {\n           \
    \ et.add_edge(i, c, 0);\n            et.add_edge(c, i, 0);\n        }\n    }\n\
    \    et.init(0);\n    let mut ft = FenwickTreeAbstract::<i64>::new(n + n, |a,\
    \ b| a + b, 0, |a| -a);\n    input! { q: usize, }\n    for _ in 0..q {\n     \
    \   input! { query: usize, }\n        match query {\n            0 => {\n    \
    \            input! { v: usize, w:i64, }\n                let index = et.index(v);\n\
    \                ft.add(index.0, w);\n                ft.add(index.1, -w);\n \
    \           }\n            1 => {\n                input! { v: usize, }\n    \
    \            let mut res = 0;\n                et.for_each_edge(0, v, |l, r| res\
    \ += ft.sum(l, r));\n                println!(\"{}\", res);\n            }\n \
    \           _ => unreachable!(),\n        }\n    }\n}\n"
  dependsOn:
  - data-structure/fenwick-tree-abstract/src/lib.rs
  - tree/euler-tour/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/grl_5_d/src/main.rs
  requiredBy: []
  timestamp: '2025-06-21 02:45:26+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/aizu-online-judge/grl_5_d/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/grl_5_d/src/main.rs
- /verify/verification/aizu-online-judge/grl_5_d/src/main.rs.html
title: verification/aizu-online-judge/grl_5_d/src/main.rs
---
