---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/segment-tree/src/lib.rs
    title: Segment Tree
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_range_composite\n\
    \nuse proconio::input;\n\nuse modint::StaticModint;\nuse segment_tree::SegmentTree;\n\
    \ntype Mint = StaticModint<998244353>;\n\nfn main() {\n    input! {\n        n:\
    \ usize,\n        q: usize,\n        ab: [(u64, u64); n],\n    }\n    let mut\
    \ st = SegmentTree::<(Mint, Mint)>::new(n, |a, b| (a.0 * b.0, a.1 * b.0 + b.1),\
    \ (Mint::new(1), Mint::new(0)));\n    let ab = ab.iter().map(|&(a, b)| (Mint::new(a),\
    \ Mint::new(b))).collect::<Vec<_>>();\n    st.build(ab);\n    for _ in 0..q {\n\
    \        input! { query: usize, }\n        match query {\n            0 => {\n\
    \                input! { p: usize, c: u64, d: u64, }\n                st.set(p,\
    \ (Mint::new(c), Mint::new(d)));\n            }\n            1 => {\n        \
    \        input! { l: usize, r: usize, x: u64, }\n                let (a, b) =\
    \ st.prod(l, r);\n                println!(\"{}\", Mint::new(x) * a + b);\n  \
    \          }\n            _ => unreachable!(),\n        }\n    }\n}\n"
  dependsOn:
  - data-structure/segment-tree/src/lib.rs
  - math/modint/src/lib.rs
  isVerificationFile: true
  path: verification/library-checker/point_set_range_composite/src/main.rs
  requiredBy: []
  timestamp: '2025-08-21 20:46:40+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/library-checker/point_set_range_composite/src/main.rs
layout: document
redirect_from:
- /verify/verification/library-checker/point_set_range_composite/src/main.rs
- /verify/verification/library-checker/point_set_range_composite/src/main.rs.html
title: verification/library-checker/point_set_range_composite/src/main.rs
---
