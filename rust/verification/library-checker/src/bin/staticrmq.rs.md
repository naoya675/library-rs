---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: rust/structure/segment-tree/src/lib.rs
    title: rust/structure/segment-tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: rust/structure/segment-tree/src/segment_tree.rs
    title: rust/structure/segment-tree/src/segment_tree.rs
  - icon: ':heavy_check_mark:'
    path: rust/structure/union-find/src/lib.rs
    title: rust/structure/union-find/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: rust/structure/union-find/src/union_find.rs
    title: rust/structure/union-find/src/union_find.rs
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
    \nuse proconio::input;\nuse std::cmp::{max, min};\n\nuse segment_tree::SegmentTree;\n\
    \nfn main() {\n    input! {\n        n: usize,\n        q: usize,\n        a:\
    \ [i32; n],\n        lr: [(usize, usize); q],\n    }\n    let mut st = SegmentTree::new(n,\
    \ |a, b| min(a, b), i32::MAX);\n    for i in 0..n {\n        st.set(i, a[i]);\n\
    \    }\n    for (l, r) in lr {\n        println!(\"{}\", st.prod(l, r));\n   \
    \ }\n}\n"
  dependsOn:
  - rust/structure/segment-tree/src/lib.rs
  - rust/structure/segment-tree/src/segment_tree.rs
  - rust/structure/union-find/src/lib.rs
  - rust/structure/union-find/src/union_find.rs
  isVerificationFile: true
  path: rust/verification/library-checker/src/bin/staticrmq.rs
  requiredBy: []
  timestamp: '2024-03-11 15:30:14+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: rust/verification/library-checker/src/bin/staticrmq.rs
layout: document
redirect_from:
- /verify/rust/verification/library-checker/src/bin/staticrmq.rs
- /verify/rust/verification/library-checker/src/bin/staticrmq.rs.html
title: rust/verification/library-checker/src/bin/staticrmq.rs
---
