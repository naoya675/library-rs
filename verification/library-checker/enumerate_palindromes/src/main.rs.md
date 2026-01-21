---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: string/manacher/src/lib.rs
    title: Manacher
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/enumerate_palindromes
    links:
    - https://judge.yosupo.jp/problem/enumerate_palindromes
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/enumerate_palindromes\n\
    \nuse itertools::Itertools;\nuse proconio::{input, marker::Chars};\n\nuse manacher::manacher;\n\
    \nfn main() {\n    input! {\n        s: Chars,\n    }\n    let s = s.iter().intersperse(&'#').collect::<Vec<_>>();\n\
    \n    println!(\"{}\", manacher(&s).iter().enumerate().map(|(i, &k)| k - ((i ^\
    \ k ^ 1) & 1)).join(\" \"));\n}\n"
  dependsOn:
  - string/manacher/src/lib.rs
  isVerificationFile: true
  path: verification/library-checker/enumerate_palindromes/src/main.rs
  requiredBy: []
  timestamp: '2026-01-01 00:11:18+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/library-checker/enumerate_palindromes/src/main.rs
layout: document
redirect_from:
- /verify/verification/library-checker/enumerate_palindromes/src/main.rs
- /verify/verification/library-checker/enumerate_palindromes/src/main.rs.html
title: verification/library-checker/enumerate_palindromes/src/main.rs
---
