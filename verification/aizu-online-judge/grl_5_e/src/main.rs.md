---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/fenwick-tree-abstract/src/lib.rs
    title: Fenwick Tree (Abstract)
  - icon: ':heavy_check_mark:'
    path: macro/query/src/lib.rs
    title: macro/query/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: tree/heavy-light-decomposition/src/lib.rs
    title: Heavy-Light Decomposition
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_E
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_E
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_E\n\
    \nuse proconio::input;\n\nuse fenwick_tree_abstract::FenwickTreeAbstract;\nuse\
    \ heavy_light_decomposition::HeavyLightDecomposition;\n\nquery::define_query!\
    \ {\n    Query {\n        0 => Query0(v: usize, w: i64),\n        1 => Query1(v:\
    \ usize),\n    }\n}\n\nfn main() {\n    std::thread::Builder::new()\n        .stack_size(64\
    \ * 1024 * 1024)\n        .spawn(actual_main)\n        .unwrap()\n        .join()\n\
    \        .unwrap();\n}\n\nfn actual_main() {\n    input! {\n        n: usize,\n\
    \    }\n    let mut hld = HeavyLightDecomposition::new(n);\n    for i in 0..n\
    \ {\n        input! { k: usize, c: [usize; k], }\n        for c in c {\n     \
    \       hld.add_edge(i, c, 0);\n            hld.add_edge(c, i, 0);\n        }\n\
    \    }\n    hld.init(0);\n\n    let mut ft = vec![FenwickTreeAbstract::<i64>::new(n\
    \ + 1, |x, y| x + y, 0, |x| -x); 2];\n    input! {\n        q: usize,\n      \
    \  queries: [Query; q],\n    }\n    for query in queries {\n        match query\
    \ {\n            Query0(v, w) => {\n                hld.for_each_edge(0, v, |l,\
    \ r| {\n                    ft[0].add(l, -w * l as i64);\n                   \
    \ ft[0].add(r, w * r as i64);\n                    ft[1].add(l, w);\n        \
    \            ft[1].add(r, -w);\n                });\n            }\n         \
    \   Query1(v) => {\n                let mut res = 0;\n                hld.for_each_edge(0,\
    \ v, |l, r| {\n                    let sum_l = ft[0].sum(0, l) + ft[1].sum(0,\
    \ l) * l as i64;\n                    let sum_r = ft[0].sum(0, r) + ft[1].sum(0,\
    \ r) * r as i64;\n                    res += sum_r - sum_l;\n                });\n\
    \                println!(\"{}\", res);\n            }\n        }\n    }\n}\n"
  dependsOn:
  - data-structure/fenwick-tree-abstract/src/lib.rs
  - macro/query/src/lib.rs
  - tree/heavy-light-decomposition/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/grl_5_e/src/main.rs
  requiredBy: []
  timestamp: '2025-12-08 22:55:07+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/aizu-online-judge/grl_5_e/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/grl_5_e/src/main.rs
- /verify/verification/aizu-online-judge/grl_5_e/src/main.rs.html
title: verification/aizu-online-judge/grl_5_e/src/main.rs
---
