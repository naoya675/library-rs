---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: math/mod-combinatorial/src/lib.rs
    title: Mod Combinatorial
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_5_D
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_5_D
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_5_D\n\
    \nuse proconio::input;\n\nuse mod_combinatorial::ModCombinatorial;\n\nconst MOD:\
    \ u64 = 1_000_000_007;\n\nfn main() {\n    input! {\n        n: usize,\n     \
    \   k: usize,\n    }\n    let mc = ModCombinatorial::<MOD>::new(n + k);\n    println!(\"\
    {}\", mc.homo(k, n));\n}\n"
  dependsOn:
  - math/mod-combinatorial/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/dpl_5_d/src/main.rs
  requiredBy: []
  timestamp: '2024-04-03 21:31:25+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/aizu-online-judge/dpl_5_d/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/dpl_5_d/src/main.rs
- /verify/verification/aizu-online-judge/dpl_5_d/src/main.rs.html
title: verification/aizu-online-judge/dpl_5_d/src/main.rs
---
