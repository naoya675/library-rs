---
data:
  _extendedDependsOn:
  - icon: ':x:'
    path: string/z-algorithm/src/lib.rs
    title: Z Algorithm
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/zalgorithm
    links:
    - https://judge.yosupo.jp/problem/zalgorithm
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/zalgorithm\n\
    \nuse itertools::Itertools;\nuse proconio::{input, marker::Chars};\n\nuse z_algorithm::z_algorithm;\n\
    \nfn main() {\n    input! {\n        s: Chars,\n    }\n    println!(\"{}\", z_algorithm(&s).iter().join(\"\
    \ \"));\n}\n"
  dependsOn:
  - string/z-algorithm/src/lib.rs
  isVerificationFile: true
  path: verification/library-checker/zalgorithm/src/main.rs
  requiredBy: []
  timestamp: '2025-12-31 23:53:43+09:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: verification/library-checker/zalgorithm/src/main.rs
layout: document
redirect_from:
- /verify/verification/library-checker/zalgorithm/src/main.rs
- /verify/verification/library-checker/zalgorithm/src/main.rs.html
title: verification/library-checker/zalgorithm/src/main.rs
---
