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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "#[derive(Debug)]\npub struct RunLendth {}\n\nimpl RunLendth {\n    pub fn\
    \ encode<T: Copy + PartialEq>(s: &[T]) -> Vec<(T, usize)> {\n        let mut res\
    \ = vec![];\n        if s.len() == 0 {\n            return res;\n        }\n \
    \       let mut i = 0;\n        while i < s.len() {\n            let mut j = i;\n\
    \            while j < s.len() && s[i] == s[j] {\n                j += 1;\n  \
    \          }\n            res.push((s[i], j - i));\n            i = j;\n     \
    \   }\n        res\n    }\n\n    pub fn decode<T: Copy>(s: &[(T, usize)]) -> Vec<T>\
    \ {\n        let mut res = vec![];\n        if s.len() == 0 {\n            return\
    \ res;\n        }\n        for &(value, c) in s {\n            res.extend(vec![value;\
    \ c]);\n        }\n        res\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: string/run-length/src/struct.rs
  requiredBy: []
  timestamp: '2026-01-01 00:11:18+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: string/run-length/src/struct.rs
layout: document
redirect_from:
- /library/string/run-length/src/struct.rs
- /library/string/run-length/src/struct.rs.html
title: string/run-length/src/struct.rs
---
