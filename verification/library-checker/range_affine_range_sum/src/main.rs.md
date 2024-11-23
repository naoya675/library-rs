---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/lazy-segment-tree/src/lib.rs
    title: Lazy Segment Tree
  - icon: ':heavy_check_mark:'
    path: math/mod-int/src/lib.rs
    title: Mod Int
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/range_affine_range_sum
    links:
    - https://judge.yosupo.jp/problem/range_affine_range_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_range_sum\n\
    \nuse proconio::input;\n\nuse lazy_segment_tree::LazySegmentTree;\nuse mod_int::ModInt;\n\
    \ntype Mint = ModInt<998244353>;\n\nfn main() {\n    input! {\n        n: usize,\n\
    \        q: usize,\n        a: [u64; n],\n    }\n    let mut lst = LazySegmentTree::<(Mint,\
    \ Mint), (Mint, Mint)>::new(\n        n,\n        |a, b| (a.0 + b.0, a.1 + b.1),\n\
    \        (Mint::new(0), Mint::new(0)),\n        |a, b| (a.0 * b.0 + a.1 * b.1,\
    \ b.1),\n        |a, b| (a.0 * b.0, a.0 * b.1 + a.1),\n        (Mint::new(1),\
    \ Mint::new(0)),\n    );\n    let a = a\n        .iter()\n        .map(|&a| (Mint::new(a),\
    \ Mint::new(1)))\n        .collect::<Vec<_>>();\n    lst.build(a);\n    for _\
    \ in 0..q {\n        input! { query: usize, }\n        match query {\n       \
    \     0 => {\n                input! { l: usize, r: usize, b: u64, c: u64, }\n\
    \                lst.apply(l, r, (Mint::new(b), Mint::new(c)));\n            }\n\
    \            1 => {\n                input! { l: usize, r: usize, }\n        \
    \        println!(\"{}\", lst.prod(l, r).0);\n            }\n            _ =>\
    \ unreachable!(),\n        }\n    }\n}\n"
  dependsOn:
  - data-structure/lazy-segment-tree/src/lib.rs
  - math/mod-int/src/lib.rs
  isVerificationFile: true
  path: verification/library-checker/range_affine_range_sum/src/main.rs
  requiredBy: []
  timestamp: '2024-11-23 20:47:05+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/library-checker/range_affine_range_sum/src/main.rs
layout: document
redirect_from:
- /verify/verification/library-checker/range_affine_range_sum/src/main.rs
- /verify/verification/library-checker/range_affine_range_sum/src/main.rs.html
title: verification/library-checker/range_affine_range_sum/src/main.rs
---
