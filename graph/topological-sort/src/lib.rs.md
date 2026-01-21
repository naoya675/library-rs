---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: verification/aizu-online-judge/grl_4_b/src/main.rs
    title: verification/aizu-online-judge/grl_4_b/src/main.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/grl_4_a/src/main.rs
    title: verification/aizu-online-judge/grl_4_a/src/main.rs
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
  code: "use std::{cmp::Reverse, collections::BinaryHeap};\n\npub fn topological_sort(size:\
    \ usize, graph: &[Vec<usize>]) -> Vec<usize> {\n    let mut indegree = vec![0;\
    \ size];\n    for from in 0..size {\n        for i in 0..graph[from].len() {\n\
    \            let to = graph[from][i];\n            indegree[to] += 1;\n      \
    \  }\n    }\n    let mut heap = BinaryHeap::new();\n    for from in 0..size {\n\
    \        if indegree[from] == 0 {\n            heap.push(Reverse(from));\n   \
    \     }\n    }\n    let mut res = vec![];\n    while let Some(Reverse(from)) =\
    \ heap.pop() {\n        res.push(from);\n        for i in 0..graph[from].len()\
    \ {\n            let to = graph[from][i];\n            indegree[to] -= 1;\n  \
    \          if indegree[to] == 0 {\n                heap.push(Reverse(to));\n \
    \           }\n        }\n    }\n    if res.len() != size {\n        return vec![];\n\
    \    }\n    res\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: graph/topological-sort/src/lib.rs
  requiredBy:
  - verification/aizu-online-judge/grl_4_b/src/main.rs
  timestamp: '2026-01-01 00:11:18+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/grl_4_a/src/main.rs
documentation_of: graph/topological-sort/src/lib.rs
layout: document
redirect_from:
- /library/graph/topological-sort/src/lib.rs
- /library/graph/topological-sort/src/lib.rs.html
title: graph/topological-sort/src/lib.rs
---
