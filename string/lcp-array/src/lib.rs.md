---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':x:'
    path: verification/library-checker/number_of_substrings/src/main.rs
    title: verification/library-checker/number_of_substrings/src/main.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    links:
    - https://atcoder.github.io/ac-library/production/document_en/string.html
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// reference: https://atcoder.github.io/ac-library/production/document_en/string.html\n\
    \n#[derive(Debug)]\npub struct LCPArray;\n\nimpl LCPArray {\n    pub fn lcp_array<T:\
    \ Copy + Ord + PartialOrd>(s: &Vec<T>, sa: &Vec<usize>) -> Vec<usize> {\n    \
    \    assert!(s.len() == sa.len());\n        let n = s.len();\n        let mut\
    \ rank = vec![0; n];\n        for i in 0..n {\n            assert!(sa[i] < n);\n\
    \            rank[sa[i]] = i;\n        }\n        let mut lcp = vec![0; n - 1];\n\
    \        let mut h = 0;\n        for i in 0..n {\n            if h > 0 {\n   \
    \             h -= 1;\n            }\n            if rank[i] == 0 {\n        \
    \        continue;\n            }\n            let j = sa[rank[i] - 1];\n    \
    \        while j + h < n && i + h < n {\n                if s[j + h] != s[i +\
    \ h] {\n                    break;\n                }\n                h += 1;\n\
    \            }\n            lcp[rank[i] - 1] = h;\n        }\n        lcp\n  \
    \  }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: string/lcp-array/src/lib.rs
  requiredBy: []
  timestamp: '2025-06-21 17:54:09+09:00'
  verificationStatus: LIBRARY_ALL_WA
  verifiedWith:
  - verification/library-checker/number_of_substrings/src/main.rs
documentation_of: string/lcp-array/src/lib.rs
layout: document
title: LCP Array
---

## Description
