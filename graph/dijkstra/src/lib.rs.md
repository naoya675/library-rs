---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/grl_1_a/src/main.rs
    title: verification/aizu-online-judge/grl_1_a/src/main.rs
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
  code: "use std::collections::BinaryHeap;\n\npub fn dijkstra(size: usize, graph:\
    \ &[Vec<(usize, i64)>], s: usize) -> Vec<i64> {\n    let mut dist = vec![i64::MAX\
    \ / 4; size];\n    let mut heap: BinaryHeap<(i64, usize)> = BinaryHeap::new();\n\
    \    dist[s] = 0;\n    heap.push((-dist[s], s));\n    while let Some((d, from))\
    \ = heap.pop() {\n        if dist[from] < -d {\n            continue;\n      \
    \  }\n        for &(to, cost) in &graph[from] {\n            if dist[to] > dist[from]\
    \ + cost {\n                dist[to] = dist[from] + cost;\n                heap.push((-dist[to],\
    \ to));\n            }\n        }\n    }\n    dist\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: graph/dijkstra/src/lib.rs
  requiredBy: []
  timestamp: '2026-01-01 00:11:18+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/grl_1_a/src/main.rs
documentation_of: graph/dijkstra/src/lib.rs
layout: document
title: Dijkstra
---

## Description
