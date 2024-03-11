---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/src/lib.rs
    title: data-structure/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: data-structure/src/segment_tree.rs
    title: data-structure/src/segment_tree.rs
  - icon: ':heavy_check_mark:'
    path: data-structure/src/union_find.rs
    title: data-structure/src/union_find.rs
  - icon: ':heavy_check_mark:'
    path: graph/src/ford_fulkerson.rs
    title: graph/src/ford_fulkerson.rs
  - icon: ':heavy_check_mark:'
    path: graph/src/lib.rs
    title: graph/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/staticrmq
    links:
    - https://judge.yosupo.jp/problem/staticrmq
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq\n\
    \nuse proconio::input;\nuse std::cmp::min;\n\nuse data_structure::SegmentTree;\n\
    \nfn main() {\n    input! {\n        n: usize,\n        q: usize,\n        a:\
    \ [i32; n],\n        lr: [(usize, usize); q],\n    }\n    let mut st = SegmentTree::new(n,\
    \ |a, b| min(a, b), i32::MAX);\n    for i in 0..n {\n        st.set(i, a[i]);\n\
    \    }\n    for (l, r) in lr {\n        println!(\"{}\", st.prod(l, r));\n   \
    \ }\n}\n"
  dependsOn:
  - data-structure/src/lib.rs
  - data-structure/src/segment_tree.rs
  - data-structure/src/union_find.rs
  - graph/src/ford_fulkerson.rs
  - graph/src/lib.rs
  isVerificationFile: true
  path: verification/library-checker/src/bin/staticrmq.rs
  requiredBy: []
  timestamp: '2024-03-11 21:07:23+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/library-checker/src/bin/staticrmq.rs
layout: document
redirect_from:
- /verify/verification/library-checker/src/bin/staticrmq.rs
- /verify/verification/library-checker/src/bin/staticrmq.rs.html
title: verification/library-checker/src/bin/staticrmq.rs
---
