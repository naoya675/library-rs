---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/segment-tree/src/lib.rs
    title: Segment Tree
  - icon: ':heavy_check_mark:'
    path: data-structure/segment-tree/src/wrapper.rs
    title: data-structure/segment-tree/src/wrapper.rs
  - icon: ':heavy_check_mark:'
    path: math/modint/src/lib.rs
    title: Modint
  - icon: ':heavy_check_mark:'
    path: tree/euler-tour/src/lib.rs
    title: tree/euler-tour/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/vertex_set_path_composite
    links:
    - https://judge.yosupo.jp/problem/vertex_set_path_composite
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_set_path_composite\n\
    \nuse proconio::input;\n\nuse euler_tour::EulerTour;\nuse modint::StaticModint;\n\
    use segment_tree::SegmentTree;\n\ntype Mint = StaticModint<998244353>;\n\nfn main()\
    \ {\n    std::thread::Builder::new()\n        .stack_size(64 * 1024 * 1024)\n\
    \        .spawn(actual_main)\n        .unwrap()\n        .join()\n        .unwrap();\n\
    }\n\nfn actual_main() {\n    input! {\n        n: usize,\n        q: usize,\n\
    \        ab: [(u64, u64); n],\n        uv: [(usize, usize); n - 1],\n    }\n \
    \   let mut et = EulerTour::<usize>::new(n);\n    for (u, v) in uv {\n       \
    \ et.add_edge(u, v, 0);\n        et.add_edge(v, u, 0);\n    }\n    et.init(0);\n\
    \    let val = |a: u64, b: u64| (Mint::new(a), Mint::new(b));\n    let invval\
    \ = |a: u64, b: u64| (Mint::new(1) / Mint::new(a), -Mint::new(b) / Mint::new(a));\n\
    \    let mut st1 = SegmentTree::<(Mint, Mint)>::new(n + n, |a, b| (a.0 * b.0,\
    \ a.1 * b.0 + b.1), val(1, 0));\n    let mut st2 = SegmentTree::<(Mint, Mint)>::new(n\
    \ + n, |b, a| (a.0 * b.0, a.1 * b.0 + b.1), val(1, 0));\n    for i in 0..n {\n\
    \        let (a, b) = ab[i];\n        let index = et.index(i);\n        st1.set(index.0,\
    \ val(a, b));\n        st2.set(index.0, val(a, b));\n        st1.set(index.1,\
    \ invval(a, b));\n        st2.set(index.1, invval(a, b));\n    }\n    for _ in\
    \ 0..q {\n        input! { query: usize, }\n        match query {\n          \
    \  0 => {\n                input! { p: usize, c: u64, d: u64 }\n             \
    \   let index = et.index(p);\n                st1.set(index.0, val(c, d));\n \
    \               st2.set(index.0, val(c, d));\n                st1.set(index.1,\
    \ invval(c, d));\n                st2.set(index.1, invval(c, d));\n          \
    \  }\n            1 => {\n                input! { u: usize, v: usize, x: u64\
    \ }\n                let x = std::cell::RefCell::new(Mint::new(x));\n        \
    \        et.for_each_with(\n                    u,\n                    v,\n \
    \                   |l, r| {\n                        let (a, b) = st1.prod(l,\
    \ r);\n                        let tmp = *x.borrow() * a + b;\n              \
    \          *x.borrow_mut() = tmp;\n                    },\n                  \
    \  |l, r| {\n                        let (a, b) = st2.prod(l, r);\n          \
    \              let tmp = *x.borrow() * a + b;\n                        *x.borrow_mut()\
    \ = tmp;\n                    },\n                );\n                println!(\"\
    {}\", x.into_inner());\n            }\n            _ => unreachable!(),\n    \
    \    }\n    }\n}\n"
  dependsOn:
  - data-structure/segment-tree/src/lib.rs
  - data-structure/segment-tree/src/wrapper.rs
  - math/modint/src/lib.rs
  - tree/euler-tour/src/lib.rs
  isVerificationFile: true
  path: verification/library-checker/vertex_set_path_composite/src/main.rs
  requiredBy: []
  timestamp: '2025-06-21 02:45:26+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/library-checker/vertex_set_path_composite/src/main.rs
layout: document
redirect_from:
- /verify/verification/library-checker/vertex_set_path_composite/src/main.rs
- /verify/verification/library-checker/vertex_set_path_composite/src/main.rs.html
title: verification/library-checker/vertex_set_path_composite/src/main.rs
---
