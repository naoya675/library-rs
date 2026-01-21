---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: data-structure/union-find-with-potential/src/lib.rs
    title: Union Find with Potential
  - icon: ':question:'
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
    PROBLEM: https://judge.yosupo.jp/problem/unionfind_with_potential_non_commutative_group
    links:
    - https://judge.yosupo.jp/problem/unionfind_with_potential_non_commutative_group
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind_with_potential_non_commutative_group\n\
    \nuse proconio::input;\n\nuse modint::StaticModint;\nuse union_find_with_potential::UnionFindWithPotential;\n\
    \ntype Mint = StaticModint<998244353>;\n\nquery::define_query! {\n    Query {\n\
    \        0 => Query0(u: usize, v: usize, x: u64, y: u64, z: u64, w: u64),\n  \
    \      1 => Query1(u: usize, v: usize),\n    }\n}\n\n#[derive(Debug, Clone, Copy,\
    \ PartialEq, Eq)]\npub struct Matrix {\n    mat: (Mint, Mint, Mint, Mint),\n}\n\
    \nimpl Matrix {\n    pub fn new(x: u64, y: u64, z: u64, w: u64) -> Self {\n  \
    \      Self {\n            mat: (Mint::new(x), Mint::new(y), Mint::new(z), Mint::new(w)),\n\
    \        }\n    }\n\n    pub fn inv(x: Matrix) -> Self {\n        Self {\n   \
    \         mat: (x.mat.3, -x.mat.1, -x.mat.2, x.mat.0),\n        }\n    }\n\n \
    \   pub fn add(x: Matrix, y: Matrix) -> Self {\n        Self {\n            mat:\
    \ (x.mat.0 + y.mat.0, x.mat.1 + y.mat.1, x.mat.2 + y.mat.2, x.mat.3 + y.mat.3),\n\
    \        }\n    }\n\n    pub fn mul(x: Matrix, y: Matrix) -> Self {\n        Self\
    \ {\n            mat: (\n                x.mat.0 * y.mat.0 + x.mat.1 * y.mat.2,\n\
    \                x.mat.0 * y.mat.1 + x.mat.1 * y.mat.3,\n                x.mat.2\
    \ * y.mat.0 + x.mat.3 * y.mat.2,\n                x.mat.2 * y.mat.1 + x.mat.3\
    \ * y.mat.3,\n            ),\n        }\n    }\n}\n\nfn main() {\n    input! {\n\
    \        n: usize,\n        q: usize,\n        queries: [Query; q],\n    }\n \
    \   let mut uf = UnionFindWithPotential::<Matrix>::new(n, |x, y| Matrix::mul(x,\
    \ y), Matrix::new(1, 0, 0, 1), |x| Matrix::inv(x));\n\n    for query in queries\
    \ {\n        match query {\n            Query0(u, v, x, y, z, w) => {\n      \
    \          println!(\"{}\", if uf.merge(u, v, Matrix::new(x, y, z, w)).is_some()\
    \ { 1 } else { 0 });\n            }\n            Query1(u, v) => {\n         \
    \       if uf.same(u, v) {\n                    let (x, y, z, w) = uf.diff(u,\
    \ v).mat;\n                    println!(\"{} {} {} {}\", x, y, z, w);\n      \
    \          } else {\n                    println!(\"-1\");\n                }\n\
    \            }\n        }\n    }\n}\n"
  dependsOn:
  - data-structure/union-find-with-potential/src/lib.rs
  - macro/query/src/lib.rs
  - math/modint/src/lib.rs
  isVerificationFile: true
  path: verification/library-checker/unionfind_with_potential_non_commutative_group/src/main.rs
  requiredBy: []
  timestamp: '2025-12-08 22:55:07+09:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: verification/library-checker/unionfind_with_potential_non_commutative_group/src/main.rs
layout: document
redirect_from:
- /verify/verification/library-checker/unionfind_with_potential_non_commutative_group/src/main.rs
- /verify/verification/library-checker/unionfind_with_potential_non_commutative_group/src/main.rs.html
title: verification/library-checker/unionfind_with_potential_non_commutative_group/src/main.rs
---
