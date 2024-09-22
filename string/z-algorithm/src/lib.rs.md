---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "pub fn z_algorithm(s: &Vec<char>) -> Vec<usize> {\n    let mut res = vec![0;\
    \ s.len()];\n    res[0] = s.len();\n    let mut i = 1;\n    let mut j = 0;\n \
    \   while i < s.len() {\n        while i + j < s.len() && s[i + j] == s[j] {\n\
    \            j += 1;\n        }\n        res[i] = j;\n        if j == 0 {\n  \
    \          i += 1;\n            continue;\n        }\n        let mut k = 1;\n\
    \        while i + k < s.len() && k + res[k] < j {\n            res[i + k] = res[k];\n\
    \            k += 1;\n        }\n        i += k;\n        j -= k;\n    }\n   \
    \ res\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: string/z-algorithm/src/lib.rs
  requiredBy: []
  timestamp: '2024-05-01 01:43:10+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: string/z-algorithm/src/lib.rs
layout: document
redirect_from:
- /library/string/z-algorithm/src/lib.rs
- /library/string/z-algorithm/src/lib.rs.html
title: string/z-algorithm/src/lib.rs
---
