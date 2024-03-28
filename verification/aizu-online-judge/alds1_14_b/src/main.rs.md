---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: string/rolling-hash/src/lib.rs
    title: Rolling Hash
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_14_B
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_14_B
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_14_B\n\
    \nuse proconio::{input, marker::Chars};\n\nuse rolling_hash::RollingHash;\n\n\
    fn main() {\n    input! {\n        t: Chars,\n        p: Chars,\n    }\n    let\
    \ mut rh = RollingHash::new();\n    let ht = rh.build(&t);\n    let hp = rh.build(&p);\n\
    \    for i in 0.. {\n        if i + p.len() > t.len() {\n            break;\n\
    \        }\n        if rh.rolling_hash(&ht, i, i + p.len()) == rh.rolling_hash(&hp,\
    \ 0, p.len()) {\n            println!(\"{}\", i);\n        }\n    }\n}\n"
  dependsOn:
  - string/rolling-hash/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/alds1_14_b/src/main.rs
  requiredBy: []
  timestamp: '2024-03-28 17:53:35+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/aizu-online-judge/alds1_14_b/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/alds1_14_b/src/main.rs
- /verify/verification/aizu-online-judge/alds1_14_b/src/main.rs.html
title: verification/aizu-online-judge/alds1_14_b/src/main.rs
---
