---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/grl_1_b/src/main.rs
    title: verification/aizu-online-judge/grl_1_b/src/main.rs
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
  code: "pub fn bellman_ford(size: usize, edge: &[(usize, usize, i64)], s: usize)\
    \ -> (bool, Vec<i64>) {\n    let mut dist = vec![i64::MAX / 4; size];\n    dist[s]\
    \ = 0;\n    for _ in 0..size {\n        let mut update = false;\n        for &(from,\
    \ to, cost) in edge {\n            if dist[from] == i64::MAX / 4 {\n         \
    \       continue;\n            }\n            if dist[from] + cost < dist[to]\
    \ {\n                dist[to] = dist[from] + cost;\n                update = true;\n\
    \            }\n        }\n        if !update {\n            return (false, dist);\n\
    \        }\n    }\n    for _ in 0..size {\n        for &(from, to, cost) in edge\
    \ {\n            if dist[from] == i64::MAX / 4 {\n                continue;\n\
    \            }\n            if dist[from] + cost < dist[to] {\n              \
    \  dist[to] = -(i64::MAX / 4);\n            }\n        }\n    }\n    (true, dist)\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: graph/bellman-ford/src/lib.rs
  requiredBy: []
  timestamp: '2026-01-01 00:11:18+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/grl_1_b/src/main.rs
documentation_of: graph/bellman-ford/src/lib.rs
layout: document
title: Bellman Ford
---

## Description
