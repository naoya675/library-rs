---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/interval-set/src/lib.rs
    title: Interval Set
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=2880
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=2880
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=2880\n\
    \nuse proconio::input;\n\nuse interval_set::IntervalSet;\n\nfn main() {\n    input!\
    \ {\n        _n: usize,\n        m: usize,\n        q: usize,\n        dab: [(usize,\
    \ usize, usize); m],\n        est: [(usize, usize, usize); q],\n    }\n    let\
    \ mut query = vec![];\n    for (_, &(d, a, b)) in dab.iter().enumerate() {\n \
    \       query.push((d * 2 + 1, 0, a * 2, b * 2 + 1));\n    }\n    for (i, &(e,\
    \ s, t)) in est.iter().enumerate() {\n        query.push((e * 2, i + 1, s * 2,\
    \ t * 2));\n    }\n    query.sort();\n    let mut set = IntervalSet::<usize, usize>::new(0);\n\
    \    let mut res = vec![false; q + 1];\n    for &(_, q, s, t) in &query {\n  \
    \      match q {\n            0 => {\n                set.insert(s, t);\n    \
    \        }\n            _ => {\n                res[q] = s >= t || set.same(s,\
    \ t);\n            }\n        }\n    }\n    for i in 1..=q {\n        println!(\"\
    {}\", if res[i] { \"Yes\" } else { \"No\" });\n    }\n}\n"
  dependsOn:
  - data-structure/interval-set/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/volume28-2880/src/main.rs
  requiredBy: []
  timestamp: '2025-12-31 21:47:15+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/aizu-online-judge/volume28-2880/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/volume28-2880/src/main.rs
- /verify/verification/aizu-online-judge/volume28-2880/src/main.rs.html
title: verification/aizu-online-judge/volume28-2880/src/main.rs
---
