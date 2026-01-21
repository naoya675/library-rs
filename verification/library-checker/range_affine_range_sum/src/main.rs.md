---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/lazy-segment-tree/src/lib.rs
    title: Lazy Segment Tree
  - icon: ':heavy_check_mark:'
    path: data-structure/lazy-segment-tree/src/wrapper.rs
    title: Lazy Segment Tree (Wrapper)
  - icon: ':heavy_check_mark:'
    path: macro/query/src/lib.rs
    title: macro/query/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: math/modint/src/lib.rs
    title: Modint
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/range_affine_range_sum
    links:
    - https://judge.yosupo.jp/problem/range_affine_range_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_range_sum\n\
    \nuse proconio::input;\n\nuse lazy_segment_tree::LazySegmentTree;\nuse modint::StaticModint;\n\
    \ntype Mint = StaticModint<998244353>;\n\nquery::define_query! {\n    Query {\n\
    \        0 => Query0(l: usize, r: usize, b: u64, c: u64),\n        1 => Query1(l:\
    \ usize, r: usize),\n    }\n}\n\nfn main() {\n    input! {\n        n: usize,\n\
    \        q: usize,\n        a: [u64; n],\n        queries: [Query; q],\n    }\n\
    \    let mut lst = LazySegmentTree::<(Mint, Mint), (Mint, Mint)>::new(\n     \
    \   n,\n        |x, y| (x.0 + y.0, x.1 + y.1),\n        (Mint::new(0), Mint::new(0)),\n\
    \        |f, x| (f.0 * x.0 + f.1 * x.1, x.1),\n        |f, g| (f.0 * g.0, f.0\
    \ * g.1 + f.1),\n        (Mint::new(1), Mint::new(0)),\n    );\n    let a = a.iter().map(|&a|\
    \ (Mint::new(a), Mint::new(1))).collect::<Vec<_>>();\n    lst.build(&a);\n\n \
    \   for query in queries {\n        match query {\n            Query0(l, r, b,\
    \ c) => lst.apply(l, r, (Mint::new(b), Mint::new(c))),\n            Query1(l,\
    \ r) => {\n                println!(\"{}\", lst.prod(l, r).0);\n            }\n\
    \        }\n    }\n}\n"
  dependsOn:
  - data-structure/lazy-segment-tree/src/lib.rs
  - data-structure/lazy-segment-tree/src/wrapper.rs
  - macro/query/src/lib.rs
  - math/modint/src/lib.rs
  isVerificationFile: true
  path: verification/library-checker/range_affine_range_sum/src/main.rs
  requiredBy: []
  timestamp: '2026-01-01 00:11:18+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/library-checker/range_affine_range_sum/src/main.rs
layout: document
redirect_from:
- /verify/verification/library-checker/range_affine_range_sum/src/main.rs
- /verify/verification/library-checker/range_affine_range_sum/src/main.rs.html
title: verification/library-checker/range_affine_range_sum/src/main.rs
---
