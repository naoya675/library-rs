---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/alds1_14_b_kmp/src/main.rs
    title: verification/aizu-online-judge/alds1_14_b_kmp/src/main.rs
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
  code: "pub fn kmp_table<T: Copy + PartialEq>(pattern: &[T]) -> Vec<usize> {\n  \
    \  if pattern.len() == 0 {\n        return vec![];\n    }\n    let mut i = 1;\n\
    \    let mut j = 0;\n    let mut failure = vec![0; pattern.len()];\n    while\
    \ i < pattern.len() {\n        if pattern[i] == pattern[j] {\n            failure[i]\
    \ = j + 1;\n            i += 1;\n            j += 1;\n        } else {\n     \
    \       if j == 0 {\n                failure[i] = 0;\n                i += 1;\n\
    \            } else {\n                j = failure[j - 1];\n            }\n  \
    \      }\n    }\n    failure\n}\n\npub fn kmp<T: Copy + PartialEq>(target: &[T],\
    \ pattern: &[T]) -> Vec<usize> {\n    let failure = kmp_table(pattern);\n    let\
    \ mut j = 0;\n    let mut res = vec![];\n    for i in 0..target.len() {\n    \
    \    while j > 0 && target[i] != pattern[j] {\n            j = failure[j - 1];\n\
    \        }\n        if target[i] == pattern[j] {\n            j += 1;\n      \
    \  }\n        if j == pattern.len() {\n            res.push(i + 1 - pattern.len());\n\
    \            j = failure[j - 1];\n        }\n    }\n    res\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: string/knuth-morris-pratt/src/lib.rs
  requiredBy: []
  timestamp: '2025-12-31 23:53:43+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/alds1_14_b_kmp/src/main.rs
documentation_of: string/knuth-morris-pratt/src/lib.rs
layout: document
title: Knuth-Morris-Pratt algorithm
---

## Description

## Reference
- [http://www-igm.univ-mlv.fr/~lecroq/string/node8.html](http://www-igm.univ-mlv.fr/~lecroq/string/node8.html)
- [https://snuke.hatenablog.com/entry/2014/12/01/235807](https://snuke.hatenablog.com/entry/2014/12/01/235807)
- [https://snuke.hatenablog.com/entry/2017/07/18/101026](https://snuke.hatenablog.com/entry/2017/07/18/101026)
- [https://snuke.hatenablog.com/entry/2015/04/05/184819](https://snuke.hatenablog.com/entry/2015/04/05/184819)
- [https://omochan.hatenablog.com/entry/2017/08/16/003840](https://omochan.hatenablog.com/entry/2017/08/16/003840)
- [https://potetisensei.hatenablog.com/entry/2017/07/10/174908](https://potetisensei.hatenablog.com/entry/2017/07/10/174908)
