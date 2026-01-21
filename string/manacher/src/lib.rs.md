---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/enumerate_palindromes/src/main.rs
    title: verification/library-checker/enumerate_palindromes/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "pub fn manacher<T: Copy + PartialEq>(s: &[T]) -> Vec<usize> {\n    if s.len()\
    \ == 0 {\n        return vec![];\n    }\n    let mut res = vec![0; s.len()];\n\
    \    let mut i = 0;\n    let mut j = 0;\n    while i < s.len() {\n        while\
    \ i >= j && i + j < s.len() && s[i - j] == s[i + j] {\n            j += 1;\n \
    \       }\n        res[i] = j;\n        let mut k = 1;\n        while i >= k &&\
    \ i + k < s.len() && k + res[i - k] < j {\n            res[i + k] = res[i - k];\n\
    \            k += 1;\n        }\n        i += k;\n        j -= k;\n    }\n   \
    \ res\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: string/manacher/src/lib.rs
  requiredBy: []
  timestamp: '2026-01-01 00:11:18+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/library-checker/enumerate_palindromes/src/main.rs
documentation_of: string/manacher/src/lib.rs
layout: document
title: Manacher
---

## Description

## Reference
- [https://snuke.hatenablog.com/entry/2014/12/02/235837](https://snuke.hatenablog.com/entry/2014/12/02/235837)
