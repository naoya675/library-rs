---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: string/suffix-array/src/lib.rs
    title: string/suffix-array/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/suffixarray
    links:
    - https://judge.yosupo.jp/problem/suffixarray
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/suffixarray\n\
    \nuse itertools::Itertools;\nuse proconio::{input, marker::Chars};\n\nuse suffix_array::SuffixArray;\n\
    \nfn main() {\n    input! {\n        s: Chars,\n    }\n    let sa = SuffixArray::suffix_array(&s);\n\
    \    println!(\"{}\", sa.iter().join(\" \"));\n}\n"
  dependsOn:
  - string/suffix-array/src/lib.rs
  isVerificationFile: true
  path: verification/library-checker/suffixarray/src/main.rs
  requiredBy: []
  timestamp: '2025-06-21 17:54:09+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/library-checker/suffixarray/src/main.rs
layout: document
redirect_from:
- /verify/verification/library-checker/suffixarray/src/main.rs
- /verify/verification/library-checker/suffixarray/src/main.rs.html
title: verification/library-checker/suffixarray/src/main.rs
---
