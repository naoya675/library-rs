---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/segment-tree/src/lib.rs
    title: Segment Tree
  - icon: ':heavy_check_mark:'
    path: data-structure/segment-tree/src/wrapper.rs
    title: Segment Tree (Wrapper)
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
    PROBLEM: https://judge.yosupo.jp/problem/point_set_range_composite
    links:
    - https://judge.yosupo.jp/problem/point_set_range_composite
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_range_composite\n\
    \nuse proconio::input;\n\nuse modint::StaticModint;\nuse segment_tree::SegmentTree;\n\
    \ntype Mint = StaticModint<998244353>;\n\nquery::define_query! {\n    Query {\n\
    \        0 => Query0(p: usize, c: u64, d: u64),\n        1 => Query1(l: usize,\
    \ r: usize, x: u64),\n    }\n}\n\nfn main() {\n    input! {\n        n: usize,\n\
    \        q: usize,\n        ab: [(u64, u64); n],\n        queries: [Query; q],\n\
    \    }\n    let mut st = SegmentTree::<(Mint, Mint)>::new(n, |x, y| (x.0 * y.0,\
    \ x.1 * y.0 + y.1), (Mint::new(1), Mint::new(0)));\n    let ab = ab.iter().map(|&(a,\
    \ b)| (Mint::new(a), Mint::new(b))).collect::<Vec<_>>();\n    st.build(&ab);\n\
    \n    for query in queries {\n        match query {\n            Query0(p, c,\
    \ d) => st.set(p, (Mint::new(c), Mint::new(d))),\n            Query1(l, r, x)\
    \ => {\n                let (a, b) = st.prod(l, r);\n                println!(\"\
    {}\", Mint::new(x) * a + b);\n            }\n        }\n    }\n}\n"
  dependsOn:
  - data-structure/segment-tree/src/lib.rs
  - data-structure/segment-tree/src/wrapper.rs
  - macro/query/src/lib.rs
  - math/modint/src/lib.rs
  isVerificationFile: true
  path: verification/library-checker/point_set_range_composite/src/main.rs
  requiredBy: []
  timestamp: '2026-01-01 00:11:18+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/library-checker/point_set_range_composite/src/main.rs
layout: document
redirect_from:
- /verify/verification/library-checker/point_set_range_composite/src/main.rs
- /verify/verification/library-checker/point_set_range_composite/src/main.rs.html
title: verification/library-checker/point_set_range_composite/src/main.rs
---
