---
data:
  _extendedDependsOn:
  - icon: ':x:'
    path: string/lcp-array/src/lib.rs
    title: LCP Array
  - icon: ':question:'
    path: string/suffix-array/src/lib.rs
    title: string/suffix-array/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/number_of_substrings
    links:
    - https://judge.yosupo.jp/problem/number_of_substrings
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/number_of_substrings\n\
    \nuse proconio::{input, marker::Chars};\n\nuse lcp_array::LCPArray;\nuse suffix_array::SuffixArray;\n\
    \nfn main() {\n    input! {\n        s: Chars,\n    }\n    let sa = SuffixArray::suffix_array(&s);\n\
    \    let lcp = LCPArray::lcp_array(&s, &sa);\n\n    println!(\"{}\", s.len() *\
    \ (s.len() + 1) / 2 - lcp.iter().sum::<usize>());\n}\n"
  dependsOn:
  - string/lcp-array/src/lib.rs
  - string/suffix-array/src/lib.rs
  isVerificationFile: true
  path: verification/library-checker/number_of_substrings/src/main.rs
  requiredBy: []
  timestamp: '2025-09-06 15:04:09+09:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: verification/library-checker/number_of_substrings/src/main.rs
layout: document
redirect_from:
- /verify/verification/library-checker/number_of_substrings/src/main.rs
- /verify/verification/library-checker/number_of_substrings/src/main.rs.html
title: verification/library-checker/number_of_substrings/src/main.rs
---
