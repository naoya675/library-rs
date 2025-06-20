---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/union-find-with-potential/src/lib.rs
    title: data-structure/union-find-with-potential/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: math/modint/src/lib.rs
    title: Modint
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/unionfind_with_potential_non_commutative_group
    links:
    - https://judge.yosupo.jp/problem/unionfind_with_potential_non_commutative_group
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind_with_potential_non_commutative_group\n\
    \nuse proconio::input;\n\nuse modint::StaticModint;\nuse union_find_with_potential::UnionFindWithPotential;\n\
    \ntype Mint = StaticModint<998244353>;\n\n#[derive(Debug, Clone, Copy, PartialEq,\
    \ Eq)]\npub struct Matrix {\n    mat: (Mint, Mint, Mint, Mint),\n}\n\nimpl Matrix\
    \ {\n    pub fn new(x: u64, y: u64, z: u64, w: u64) -> Self {\n        Self {\n\
    \            mat: (Mint::new(x), Mint::new(y), Mint::new(z), Mint::new(w)),\n\
    \        }\n    }\n\n    pub fn add(a: Matrix, b: Matrix) -> Self {\n        Self\
    \ {\n            mat: (a.mat.0 + b.mat.0, a.mat.1 + b.mat.1, a.mat.2 + b.mat.2,\
    \ a.mat.3 + b.mat.3),\n        }\n    }\n\n    pub fn mul(a: Matrix, b: Matrix)\
    \ -> Self {\n        Self {\n            mat: (\n                a.mat.0 * b.mat.0\
    \ + a.mat.1 * b.mat.2,\n                a.mat.0 * b.mat.1 + a.mat.1 * b.mat.3,\n\
    \                a.mat.2 * b.mat.0 + a.mat.3 * b.mat.2,\n                a.mat.2\
    \ * b.mat.1 + a.mat.3 * b.mat.3,\n            ),\n        }\n    }\n\n    pub\
    \ fn inv(a: Matrix) -> Self {\n        Self {\n            mat: (a.mat.3, -a.mat.1,\
    \ -a.mat.2, a.mat.0),\n        }\n    }\n}\n\nfn main() {\n    input! {\n    \
    \    n: usize,\n        q: usize,\n    }\n    let mut uf = UnionFindWithPotential::<Matrix>::new(n,\
    \ |a, b| Matrix::mul(a, b), Matrix::new(1, 0, 0, 1), |a| Matrix::inv(a));\n  \
    \  for _ in 0..q {\n        input! { query: usize, }\n        match query {\n\
    \            0 => {\n                input! { u: usize, v: usize, x: u64, y: u64,\
    \ z: u64, w: u64 }\n                println!(\"{}\", if let Some(_) = uf.merge(u,\
    \ v, Matrix::new(x, y, z, w)) { 1 } else { 0 });\n            }\n            1\
    \ => {\n                input! { u: usize, v: usize, }\n                if uf.same(u,\
    \ v) {\n                    let (x, y, z, w) = uf.diff(u, v).mat;\n          \
    \          println!(\"{} {} {} {}\", x, y, z, w);\n                } else {\n\
    \                    println!(\"-1\");\n                }\n            }\n   \
    \         _ => unreachable!(),\n        }\n    }\n}\n"
  dependsOn:
  - data-structure/union-find-with-potential/src/lib.rs
  - math/modint/src/lib.rs
  isVerificationFile: true
  path: verification/library-checker/unionfind_with_potential_non_commutative_group/src/main.rs
  requiredBy: []
  timestamp: '2025-05-29 20:17:30+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/library-checker/unionfind_with_potential_non_commutative_group/src/main.rs
layout: document
redirect_from:
- /verify/verification/library-checker/unionfind_with_potential_non_commutative_group/src/main.rs
- /verify/verification/library-checker/unionfind_with_potential_non_commutative_group/src/main.rs.html
title: verification/library-checker/unionfind_with_potential_non_commutative_group/src/main.rs
---
