---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: math/mersenne-modint/src/lib.rs
    title: Mersenne Modint
  - icon: ':heavy_check_mark:'
    path: string/rolling-hash-segment-tree/src/lib.rs
    title: Rolling Hash (Rabin-Karp)
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_14_B
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_14_B
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_14_B\n\
    \nuse proconio::{input, marker::Chars};\n\nuse mersenne_modint::MersenneModint;\n\
    use rolling_hash_segment_tree::RollingHash;\n\nfn main() {\n    input! {\n   \
    \     t: Chars,\n        p: Chars,\n    }\n    let mut rh = RollingHash::<MersenneModint>::new(MersenneModint::rand());\n\
    \    let mut ht = rh.build_segment_tree(&t);\n    let mut hp = rh.build_segment_tree(&p);\n\
    \n    for i in 0.. {\n        if i + p.len() > t.len() {\n            break;\n\
    \        }\n        if ht.prod(i, i + p.len()) == hp.prod(0, p.len()) {\n    \
    \        println!(\"{}\", i);\n        }\n    }\n}\n"
  dependsOn:
  - math/mersenne-modint/src/lib.rs
  - string/rolling-hash-segment-tree/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/alds1_14_b/src/main.rs
  requiredBy: []
  timestamp: '2026-01-21 18:49:44+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/aizu-online-judge/alds1_14_b/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/alds1_14_b/src/main.rs
- /verify/verification/aizu-online-judge/alds1_14_b/src/main.rs.html
title: verification/aizu-online-judge/alds1_14_b/src/main.rs
---
