---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/lazy-segment-tree/src/lib.rs
    title: Lazy Segment Tree
  - icon: ':heavy_check_mark:'
    path: math/modint_test/src/lib.rs
    title: math/modint_test/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/range_affine_point_get
    links:
    - https://judge.yosupo.jp/problem/range_affine_point_get
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_point_get\n\
    \nuse proconio::input;\n\nuse lazy_segment_tree::LazySegmentTree;\nuse modint_test::ModInt;\n\
    \ntype Mint = ModInt<998244353>;\n\nfn main() {\n    input! {\n        n: usize,\n\
    \        q: usize,\n        a: [u64; n],\n    }\n    let mut lst = LazySegmentTree::<(Mint,\
    \ Mint), (Mint, Mint)>::new(\n        n,\n        |a, b| (a.0 + b.0, a.1 + b.1),\n\
    \        (Mint::new(0), Mint::new(0)),\n        |a, b| (a.0 * b.0 + a.1 * b.1,\
    \ b.1),\n        |a, b| (a.0 * b.0, a.0 * b.1 + a.1),\n        (Mint::new(1),\
    \ Mint::new(0)),\n    );\n    let a = a\n        .iter()\n        .map(|&a| (Mint::new(a),\
    \ Mint::new(1)))\n        .collect::<Vec<_>>();\n    lst.build(a);\n    for _\
    \ in 0..q {\n        input! {\n            query: usize,\n        }\n        match\
    \ query {\n            0 => {\n                input! {\n                    l:\
    \ usize,\n                    r: usize,\n                    b: u64,\n       \
    \             c: u64,\n                }\n                lst.apply(l, r, (Mint::new(b),\
    \ Mint::new(c)));\n            }\n            1 => {\n                input! {\n\
    \                    i: usize,\n                }\n                println!(\"\
    {}\", lst.get(i).0);\n            }\n            _ => unreachable!(),\n      \
    \  }\n    }\n}\n"
  dependsOn:
  - data-structure/lazy-segment-tree/src/lib.rs
  - math/modint_test/src/lib.rs
  isVerificationFile: true
  path: verification/library-checker/range_affine_point_get/src/main.rs
  requiredBy: []
  timestamp: '2024-04-02 22:24:53+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/library-checker/range_affine_point_get/src/main.rs
layout: document
redirect_from:
- /verify/verification/library-checker/range_affine_point_get/src/main.rs
- /verify/verification/library-checker/range_affine_point_get/src/main.rs.html
title: verification/library-checker/range_affine_point_get/src/main.rs
---
