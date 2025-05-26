---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: math/combinatorics/src/lib.rs
    title: Combinatorics
  - icon: ':heavy_check_mark:'
    path: math/modint/src/lib.rs
    title: Modint
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_5_D
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_5_D
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_5_D\n\
    \nuse proconio::input;\n\nuse combinatorics::Combinatorics;\nuse modint::StaticModint;\n\
    \ntype Mint = StaticModint<1000000007>;\n\nfn main() {\n    input! {\n       \
    \ n: usize,\n        k: usize,\n    }\n    let mut e = Combinatorics::<Mint>::new(n\
    \ + k);\n    println!(\"{}\", e.homo(k, n));\n}\n"
  dependsOn:
  - math/combinatorics/src/lib.rs
  - math/modint/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/dpl_5_d/src/main.rs
  requiredBy: []
  timestamp: '2025-05-26 15:54:30+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/aizu-online-judge/dpl_5_d/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/dpl_5_d/src/main.rs
- /verify/verification/aizu-online-judge/dpl_5_d/src/main.rs.html
title: verification/aizu-online-judge/dpl_5_d/src/main.rs
---
