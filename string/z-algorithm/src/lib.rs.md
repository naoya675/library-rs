---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':x:'
    path: verification/library-checker/zalgorithm/src/main.rs
    title: verification/library-checker/zalgorithm/src/main.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "pub fn z_algorithm<T: Copy + PartialEq>(s: &[T]) -> Vec<usize> {\n    if\
    \ s.len() == 0 {\n        return vec![];\n    }\n    let mut res = vec![0; s.len()];\n\
    \    let mut i = 1;\n    let mut j = 0;\n    while i < s.len() {\n        while\
    \ i + j < s.len() && s[i + j] == s[j] {\n            j += 1;\n        }\n    \
    \    res[i] = j;\n        if j == 0 {\n            i += 1;\n            continue;\n\
    \        }\n        let mut k = 1;\n        while i + k < s.len() && k + res[k]\
    \ < j {\n            res[i + k] = res[k];\n            k += 1;\n        }\n  \
    \      i += k;\n        j -= k;\n    }\n    res[0] = s.len();\n    res\n}\n\n\
    /*\npub fn z_algorithm<T: Copy + PartialEq>(s: &[T]) -> Vec<usize> {\n    if s.len()\
    \ == 0 {\n        return vec![];\n    }\n    let mut res = vec![0; s.len()];\n\
    \    let mut i = 1;\n    let mut j = 0;\n    while i < s.len() {\n        res[i]\
    \ = if res[j] + j <= i { 0 } else { std::cmp::min(res[j] + j - i, res[i - j])\
    \ };\n        while i + res[i] < s.len() && s[res[i]] == s[i + res[i]] {\n   \
    \         res[i] += 1;\n        }\n        if res[j] + j < res[i] + i {\n    \
    \        j = i;\n        }\n        i += 1;\n    }\n    res[0] = s.len();\n  \
    \  res\n}\n*/\n"
  dependsOn: []
  isVerificationFile: false
  path: string/z-algorithm/src/lib.rs
  requiredBy: []
  timestamp: '2025-12-31 23:53:43+09:00'
  verificationStatus: LIBRARY_ALL_WA
  verifiedWith:
  - verification/library-checker/zalgorithm/src/main.rs
documentation_of: string/z-algorithm/src/lib.rs
layout: document
title: Z Algorithm
---

## Description

## Reference
- [https://atcoder.github.io/ac-library/production/document_en/string.html](https://atcoder.github.io/ac-library/production/document_en/string.html)

- String Algorithm
    - [https://snuke.hatenablog.com/entry/2015/04/05/184819](https://snuke.hatenablog.com/entry/2015/04/05/184819)
    - [https://snuke.hatenablog.com/entry/2017/07/18/101026](https://snuke.hatenablog.com/entry/2017/07/18/101026)
    - [https://snuke.hatenablog.com/entry/2014/12/01/235807](https://snuke.hatenablog.com/entry/2014/12/01/235807)
    - [https://snuke.hatenablog.com/entry/2014/12/02/235837](https://snuke.hatenablog.com/entry/2014/12/02/235837)
    - [https://snuke.hatenablog.com/entry/2014/12/03/214243](https://snuke.hatenablog.com/entry/2014/12/03/214243)
    - [https://qiita.com/t_fuki/items/f32406da0233ed51ec86](https://qiita.com/t_fuki/items/f32406da0233ed51ec86)
    - [https://qiita.com/t_fuki/items/e682238dda6ad832ce05](https://qiita.com/t_fuki/items/e682238dda6ad832ce05)
    - [https://qiita.com/t_fuki/items/408fe87dceb4c88bd036](https://qiita.com/t_fuki/items/408fe87dceb4c88bd036)
    - [https://blog.hamayanhamayan.com/entry/2017/03/25/005452](https://blog.hamayanhamayan.com/entry/2017/03/25/005452)
    - [https://everplay.jp/column/33278](https://everplay.jp/column/33278)
