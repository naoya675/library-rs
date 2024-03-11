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
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A\n\
    \nuse proconio::input;\nuse std::cmp::{max, min};\n\nuse segment_tree::SegmentTree;\n\
    \nfn main() {\n    input! {\n        n: usize,\n        q: usize,\n        com:\
    \ [(usize, usize, usize); q],\n    }\n    let mut st = SegmentTree::new(n, |a,\
    \ b| min(a, b), (1 << 31) - 1);\n    for (com, x, y) in com {\n        match com\
    \ {\n            0 => st.set(x, y),\n            1 => {\n                println!(\"\
    {}\", st.prod(x, y + 1));\n            }\n            _ => unreachable!(),\n \
    \       }\n    }\n}\n"
  dependsOn:
  - rust/structure/segment-tree/src/lib.rs
  - rust/structure/segment-tree/src/segment_tree.rs
  - rust/structure/union-find/src/lib.rs
  - rust/structure/union-find/src/union_find.rs
  isVerificationFile: true
  path: rust/verification/aizu-online-judge/src/bin/dsl_2_a.rs
  requiredBy: []
  timestamp: '2024-03-11 18:53:50+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: rust/verification/aizu-online-judge/src/bin/dsl_2_a.rs
layout: document
redirect_from:
- /verify/rust/verification/aizu-online-judge/src/bin/dsl_2_a.rs
- /verify/rust/verification/aizu-online-judge/src/bin/dsl_2_a.rs.html
title: rust/verification/aizu-online-judge/src/bin/dsl_2_a.rs
---
