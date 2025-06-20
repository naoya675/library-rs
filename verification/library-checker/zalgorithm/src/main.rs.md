---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: string/z-algorithm/src/lib.rs
    title: Z Algorithm
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/zalgorithm
    links:
    - https://judge.yosupo.jp/problem/zalgorithm
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/zalgorithm\n\
    \nuse itertools::Itertools;\nuse proconio::{input, marker::Chars};\n\nuse z_algorithm::ZAlgorithm;\n\
    \nfn main() {\n    input! {\n        s: Chars,\n    }\n    let mut za = ZAlgorithm::<char>::new();\n\
    \    za.build(&s);\n    println!(\"{}\", za.get().iter().join(\" \"));\n}\n"
  dependsOn:
  - string/z-algorithm/src/lib.rs
  isVerificationFile: true
  path: verification/library-checker/zalgorithm/src/main.rs
  requiredBy: []
  timestamp: '2025-06-07 00:31:50+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/library-checker/zalgorithm/src/main.rs
layout: document
redirect_from:
- /verify/verification/library-checker/zalgorithm/src/main.rs
- /verify/verification/library-checker/zalgorithm/src/main.rs.html
title: verification/library-checker/zalgorithm/src/main.rs
---
