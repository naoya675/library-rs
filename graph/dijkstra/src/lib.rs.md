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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::collections::BinaryHeap;\n\npub type Cost = i64;\n\n#[derive(Debug,\
    \ Clone, Copy)]\npub struct Edge {\n    to: usize,\n    cost: Cost,\n}\n\nimpl\
    \ Edge {\n    pub fn new(to: usize, cost: Cost) -> Self {\n        Self { to,\
    \ cost }\n    }\n}\n\n#[derive(Debug, Clone)]\npub struct Dijkstra {\n    size:\
    \ usize,\n    graph: Vec<Vec<Edge>>,\n}\n\nimpl Dijkstra {\n    pub const INF:\
    \ Cost = Cost::MAX / 2;\n\n    pub fn new(size: usize) -> Self {\n        Self\
    \ {\n            size,\n            graph: vec![vec![]; size],\n        }\n  \
    \  }\n\n    pub fn add_edge(&mut self, from: usize, to: usize, cost: Cost) {\n\
    \        self.graph[from].push(Edge::new(to, cost));\n    }\n\n    pub fn dijkstra(&mut\
    \ self, s: usize) -> Vec<Cost> {\n        let mut dist = vec![Self::INF; self.size];\n\
    \        dist[s] = 0;\n        let mut heap = BinaryHeap::new();\n        heap.push((-dist[s],\
    \ s));\n        while let Some((d, from)) = heap.pop() {\n            if dist[from]\
    \ < -d {\n                continue;\n            }\n            for edge in &self.graph[from]\
    \ {\n                if dist[edge.to] > dist[from] + edge.cost {\n           \
    \         dist[edge.to] = dist[from] + edge.cost;\n                    heap.push((-dist[edge.to],\
    \ edge.to));\n                }\n            }\n        }\n        dist\n    }\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: graph/dijkstra/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-12 23:47:43+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/grl_1_a/src/main.rs
documentation_of: graph/dijkstra/src/lib.rs
layout: document
title: Dijkstra
---

## Description
