---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/union-find/src/lib.rs
    title: Union Find
  - icon: ':heavy_check_mark:'
    path: macro/query/src/lib.rs
    title: macro/query/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/unionfind
    links:
    - https://judge.yosupo.jp/problem/unionfind
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind\n\
    \nuse proconio::input;\n\nuse union_find::UnionFind;\n\nquery::define_query! {\n\
    \    Query {\n        0 => Query0(u: usize, v: usize),\n        1 => Query1(u:\
    \ usize, v: usize),\n    }\n}\n\nfn main() {\n    input! {\n        n: usize,\n\
    \        q: usize,\n        queries: [Query; q],\n    }\n    let mut uf = UnionFind::new(n);\n\
    \n    for query in queries {\n        match query {\n            Query0(u, v)\
    \ => {\n                uf.merge(u, v);\n            }\n            Query1(u,\
    \ v) => {\n                println!(\"{}\", if uf.same(u, v) { 1 } else { 0 });\n\
    \            }\n        }\n    }\n}\n"
  dependsOn:
  - data-structure/union-find/src/lib.rs
  - macro/query/src/lib.rs
  isVerificationFile: true
  path: verification/library-checker/unionfind/src/main.rs
  requiredBy: []
  timestamp: '2025-09-06 15:04:09+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/library-checker/unionfind/src/main.rs
layout: document
redirect_from:
- /verify/verification/library-checker/unionfind/src/main.rs
- /verify/verification/library-checker/unionfind/src/main.rs.html
title: verification/library-checker/unionfind/src/main.rs
---
