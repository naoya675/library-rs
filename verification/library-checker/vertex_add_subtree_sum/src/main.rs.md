---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/fenwick-tree/src/lib.rs
    title: Fenwick Tree
  - icon: ':heavy_check_mark:'
    path: tree/euler-tour/src/lib.rs
    title: Euler Tour
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/vertex_add_subtree_sum
    links:
    - https://judge.yosupo.jp/problem/vertex_add_subtree_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_subtree_sum\n\
    \nuse proconio::input;\n\nuse euler_tour::EulerTour;\nuse fenwick_tree::FenwickTree;\n\
    \nfn main() {\n    std::thread::Builder::new()\n        .stack_size(64 * 1024\
    \ * 1024)\n        .spawn(actual_main)\n        .unwrap()\n        .join()\n \
    \       .unwrap();\n}\n\nfn actual_main() {\n    input! {\n        n: usize,\n\
    \        q: usize,\n        a: [i64; n],\n        p: [usize; n - 1],\n    }\n\
    \    let mut et = EulerTour::<usize>::new(n);\n    for (i, &p) in p.iter().enumerate()\
    \ {\n        et.add_edge(i + 1, p, 0);\n        et.add_edge(p, i + 1, 0);\n  \
    \  }\n    et.init(0);\n    let mut ft = FenwickTree::<i64>::new(n + n);\n    for\
    \ i in 0..n {\n        let index = et.index(i);\n        ft.add(index.0, a[i]);\n\
    \    }\n    for _ in 0..q {\n        input! { query: usize, }\n        match query\
    \ {\n            0 => {\n                input! { p: usize, x: i64, }\n      \
    \          let index = et.index(p);\n                ft.add(index.0, x);\n   \
    \         }\n            1 => {\n                input! { u: usize, }\n      \
    \          let mut res = 0;\n                et.for_each_subtree(u, |l, r| res\
    \ += ft.sum(l, r));\n                println!(\"{}\", res);\n            }\n \
    \           _ => unreachable!(),\n        }\n    }\n}\n"
  dependsOn:
  - data-structure/fenwick-tree/src/lib.rs
  - tree/euler-tour/src/lib.rs
  isVerificationFile: true
  path: verification/library-checker/vertex_add_subtree_sum/src/main.rs
  requiredBy: []
  timestamp: '2025-08-21 20:46:40+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/library-checker/vertex_add_subtree_sum/src/main.rs
layout: document
redirect_from:
- /verify/verification/library-checker/vertex_add_subtree_sum/src/main.rs
- /verify/verification/library-checker/vertex_add_subtree_sum/src/main.rs.html
title: verification/library-checker/vertex_add_subtree_sum/src/main.rs
---
