---
data:
  _extendedDependsOn:
  - icon: ':x:'
    path: data-structure/union-find-with-potential/src/lib.rs
    title: Union Find with Potential
  - icon: ':x:'
    path: macro/query/src/lib.rs
    title: macro/query/src/lib.rs
  - icon: ':question:'
    path: math/modint/src/lib.rs
    title: Modint
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/unionfind_with_potential
    links:
    - https://judge.yosupo.jp/problem/unionfind_with_potential
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind_with_potential\n\
    \nuse proconio::input;\n\nuse modint::StaticModint;\nuse union_find_with_potential::UnionFindWithPotential;\n\
    \ntype Mint = StaticModint<998244353>;\n\nquery::define_query! {\n    Query {\n\
    \        0 => Query0(u: usize, v: usize, x: u64),\n        1 => Query1(u: usize,\
    \ v: usize),\n    }\n}\n\nfn main() {\n    input! {\n        n: usize,\n     \
    \   q: usize,\n        queries: [Query; q],\n    }\n    let mut uf = UnionFindWithPotential::<Mint>::new_default(n);\n\
    \n    for query in queries {\n        match query {\n            Query0(u, v,\
    \ x) => {\n                println!(\"{}\", if let Some(_) = uf.merge(u, v, Mint::new(x))\
    \ { 1 } else { 0 })\n            }\n            Query1(u, v) => {\n          \
    \      println!(\"{}\", if uf.same(u, v) { uf.diff(u, v).to_string() } else {\
    \ \"-1\".to_string() });\n            }\n        }\n    }\n}\n"
  dependsOn:
  - data-structure/union-find-with-potential/src/lib.rs
  - macro/query/src/lib.rs
  - math/modint/src/lib.rs
  isVerificationFile: true
  path: verification/library-checker/unionfind_with_potential/src/main.rs
  requiredBy: []
  timestamp: '2025-09-06 15:04:09+09:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: verification/library-checker/unionfind_with_potential/src/main.rs
layout: document
redirect_from:
- /verify/verification/library-checker/unionfind_with_potential/src/main.rs
- /verify/verification/library-checker/unionfind_with_potential/src/main.rs.html
title: verification/library-checker/unionfind_with_potential/src/main.rs
---
