---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/lazy-segment-tree/src/lib.rs
    title: Lazy Segment Tree
  - icon: ':heavy_check_mark:'
    path: data-structure/lazy-segment-tree/src/wrapper.rs
    title: data-structure/lazy-segment-tree/src/wrapper.rs
  - icon: ':heavy_check_mark:'
    path: tree/heavy-light-decomposition/src/lib.rs
    title: tree/heavy-light-decomposition/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_E
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_E
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_E\n\
    \nuse proconio::input;\n\nuse heavy_light_decomposition::HeavyLightDecomposition;\n\
    use lazy_segment_tree::LazySegmentTree;\n\nfn main() {\n    std::thread::Builder::new()\n\
    \        .stack_size(64 * 1024 * 1024)\n        .spawn(actual_main)\n        .unwrap()\n\
    \        .join()\n        .unwrap();\n}\n\nfn actual_main() {\n    input! {\n\
    \        n: usize,\n    }\n    let mut hld = HeavyLightDecomposition::new(n);\n\
    \    for i in 0..n {\n        input! { k: usize, c: [usize; k], }\n        for\
    \ c in c {\n            hld.add_edge(i, c, 0);\n            hld.add_edge(c, i,\
    \ 0);\n        }\n    }\n    hld.init(0);\n    let mut lst = LazySegmentTree::<(i64,\
    \ i64), i64>::new(n + n, |a, b| (a.0 + b.0, a.1 + b.1), (0, 0), |f, x| (x.0 +\
    \ f * x.1, x.1), |f, x| f + x, 0);\n    lst.build(vec![(0, 1); n + n]);\n    input!\
    \ { q: usize, }\n    for _ in 0..q {\n        input! { query: usize, }\n     \
    \   match query {\n            0 => {\n                input! { v: usize, w: i64,\
    \ }\n                hld.for_each_edge(0, v, |l, r| {\n                    lst.apply(l,\
    \ r, w);\n                });\n            }\n            1 => {\n           \
    \     input! { v: usize, }\n                let mut res = 0;\n               \
    \ hld.for_each_edge(0, v, |l, r| {\n                    res += lst.prod(l, r).0;\n\
    \                });\n                println!(\"{}\", res);\n            }\n\
    \            _ => unreachable!(),\n        }\n    }\n}\n"
  dependsOn:
  - data-structure/lazy-segment-tree/src/lib.rs
  - data-structure/lazy-segment-tree/src/wrapper.rs
  - tree/heavy-light-decomposition/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/grl_5_e/src/main.rs
  requiredBy: []
  timestamp: '2025-06-21 02:45:26+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/aizu-online-judge/grl_5_e/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/grl_5_e/src/main.rs
- /verify/verification/aizu-online-judge/grl_5_e/src/main.rs.html
title: verification/aizu-online-judge/grl_5_e/src/main.rs
---
