---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: tree/cartesian-tree/src/lib.rs
    title: Cartesian Tree
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/cartesian_tree
    links:
    - https://judge.yosupo.jp/problem/cartesian_tree
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/cartesian_tree\n\
    \nuse itertools::Itertools;\nuse proconio::input;\n\nuse cartesian_tree::CartesianTree;\n\
    \nfn main() {\n    input! {\n        n: usize,\n        a: [usize; n],\n    }\n\
    \    let mut ct = CartesianTree::new(a);\n\n    println!(\"{}\", ct.run(true).iter().enumerate().map(|(i,\
    \ &p)| if p == n { i } else { p }).join(\" \"));\n}\n"
  dependsOn:
  - tree/cartesian-tree/src/lib.rs
  isVerificationFile: true
  path: verification/library-checker/cartesian_tree/src/main.rs
  requiredBy: []
  timestamp: '2025-09-06 15:04:09+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/library-checker/cartesian_tree/src/main.rs
layout: document
redirect_from:
- /verify/verification/library-checker/cartesian_tree/src/main.rs
- /verify/verification/library-checker/cartesian_tree/src/main.rs.html
title: verification/library-checker/cartesian_tree/src/main.rs
---
