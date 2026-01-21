---
data:
  _extendedDependsOn:
  - icon: ':x:'
    path: data-structure/fenwick-tree/src/lib.rs
    title: Fenwick Tree
  - icon: ':question:'
    path: macro/query/src/lib.rs
    title: macro/query/src/lib.rs
  - icon: ':x:'
    path: tree/euler-tour/src/lib.rs
    title: Euler Tour
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/vertex_add_path_sum
    links:
    - https://judge.yosupo.jp/problem/vertex_add_path_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_path_sum\n\
    \nuse proconio::input;\n\nuse euler_tour::EulerTour;\nuse fenwick_tree::FenwickTree;\n\
    \nquery::define_query! {\n    Query {\n        0 => Query0(p: usize, x: i64),\n\
    \        1 => Query1(u: usize, v: usize),\n    }\n}\n\nfn main() {\n    std::thread::Builder::new()\n\
    \        .stack_size(64 * 1024 * 1024)\n        .spawn(actual_main)\n        .unwrap()\n\
    \        .join()\n        .unwrap();\n}\n\nfn actual_main() {\n    input! {\n\
    \        n: usize,\n        q: usize,\n        a: [i64; n],\n        uv: [(usize,\
    \ usize); n - 1],\n        queries: [Query; q],\n    }\n    let mut et = EulerTour::<usize>::new(n);\n\
    \    uv.iter().for_each(|&(u, v)| {\n        et.add_edge(u, v, 0);\n        et.add_edge(v,\
    \ u, 0);\n    });\n    et.init(0);\n\n    let mut ft = FenwickTree::<i64>::new(n\
    \ + n);\n    for i in 0..n {\n        let index = et.index(i);\n        ft.add(index.0,\
    \ a[i]);\n        ft.add(index.1, -a[i]);\n    }\n    for query in queries {\n\
    \        match query {\n            Query0(p, x) => {\n                let index\
    \ = et.index(p);\n                ft.add(index.0, x);\n                ft.add(index.1,\
    \ -x);\n            }\n            Query1(u, v) => {\n                let mut\
    \ res = 0;\n                et.for_each(u, v, |l, r| res += ft.sum(l, r));\n \
    \               println!(\"{}\", res);\n            }\n        }\n    }\n}\n"
  dependsOn:
  - data-structure/fenwick-tree/src/lib.rs
  - macro/query/src/lib.rs
  - tree/euler-tour/src/lib.rs
  isVerificationFile: true
  path: verification/library-checker/vertex_add_path_sum/src/main.rs
  requiredBy: []
  timestamp: '2025-12-14 01:36:04+09:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: verification/library-checker/vertex_add_path_sum/src/main.rs
layout: document
redirect_from:
- /verify/verification/library-checker/vertex_add_path_sum/src/main.rs
- /verify/verification/library-checker/vertex_add_path_sum/src/main.rs.html
title: verification/library-checker/vertex_add_path_sum/src/main.rs
---
