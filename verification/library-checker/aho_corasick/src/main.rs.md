---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: string/aho-corasick/src/lib.rs
    title: Aho-Corasick
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/aho_corasick
    links:
    - https://judge.yosupo.jp/problem/aho_corasick
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/aho_corasick\n\
    \nuse itertools::Itertools;\nuse proconio::{input, marker::Chars};\n\nuse aho_corasick::AhoCorasick;\n\
    \nfn main() {\n    input! {\n        n: usize,\n        s: [Chars; n],\n    }\n\
    \    let mut ac = AhoCorasick::new(26, 'a');\n    (0..n).for_each(|i| ac.insert(&s[i]));\n\
    \    ac.build(true);\n\n    let mut index = vec![None; s.iter().map(|s| s.len()).sum::<usize>()\
    \ + 1];\n    index[0] = Some(0);\n    let mut ps = vec![(0, 0)];\n    let mut\
    \ v = vec![];\n    for i in 0..n {\n        let mut now = 0;\n        for &c in\
    \ &s[i] {\n            let (_, next) = ac.next(c, now);\n            let (_, fail)\
    \ = ac.next((26 + 'a' as u8) as char, next);\n            if index[next].is_none()\
    \ {\n                index[next] = Some(ps.len());\n                ps.push((now,\
    \ fail));\n            }\n            now = next;\n        }\n        v.push(now);\n\
    \    }\n\n    println!(\"{}\", ps.len());\n    for &(p, s) in &ps[1..] {\n   \
    \     let p = index[p].unwrap();\n        let s = index[s].unwrap();\n       \
    \ println!(\"{} {}\", p, s);\n    }\n    println!(\"{}\", v.iter().map(|&v| index[v].unwrap()).join(\"\
    \ \"));\n}\n"
  dependsOn:
  - string/aho-corasick/src/lib.rs
  isVerificationFile: true
  path: verification/library-checker/aho_corasick/src/main.rs
  requiredBy: []
  timestamp: '2025-09-05 02:21:41+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/library-checker/aho_corasick/src/main.rs
layout: document
redirect_from:
- /verify/verification/library-checker/aho_corasick/src/main.rs
- /verify/verification/library-checker/aho_corasick/src/main.rs.html
title: verification/library-checker/aho_corasick/src/main.rs
---
