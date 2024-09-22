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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "pub type Cost = i64;\n\n#[derive(Debug, Clone, Copy)]\npub struct Edge {\n\
    \    from: usize,\n    to: usize,\n    cost: Cost,\n}\n\nimpl Edge {\n    pub\
    \ fn new(from: usize, to: usize, cost: Cost) -> Self {\n        Self { from, to,\
    \ cost }\n    }\n}\n\n#[derive(Debug, Clone)]\npub struct BellmanFord {\n    size:\
    \ usize,\n    edge: Vec<Edge>,\n}\n\nimpl BellmanFord {\n    pub const INF: Cost\
    \ = Cost::MAX / 2;\n\n    pub fn new(size: usize) -> Self {\n        Self { size,\
    \ edge: vec![] }\n    }\n\n    pub fn add_edge(&mut self, from: usize, to: usize,\
    \ cost: Cost) {\n        self.edge.push(Edge::new(from, to, cost));\n    }\n\n\
    \    pub fn bellman_ford(&mut self, s: usize) -> (bool, Vec<Cost>) {\n       \
    \ let mut dist = vec![Self::INF; self.size];\n        dist[s] = 0;\n        for\
    \ _ in 0..self.size {\n            let mut update = false;\n            for edge\
    \ in &self.edge {\n                if dist[edge.from] == Self::INF {\n       \
    \             continue;\n                }\n                if dist[edge.from]\
    \ + edge.cost < dist[edge.to] {\n                    dist[edge.to] = dist[edge.from]\
    \ + edge.cost;\n                    update = true;\n                }\n      \
    \      }\n            if !update {\n                return (false, dist);\n  \
    \          }\n        }\n        for _ in 0..self.size {\n            for edge\
    \ in &self.edge {\n                if dist[edge.from] == Self::INF {\n       \
    \             continue;\n                }\n                if dist[edge.from]\
    \ + edge.cost < dist[edge.to] {\n                    dist[edge.to] = -Self::INF;\n\
    \                }\n            }\n        }\n        (true, dist)\n    }\n}\n\
    \npub fn bellman_ford(size: usize, edge: &Vec<Edge>, s: usize) -> (bool, Vec<Cost>)\
    \ {\n    let mut dist = vec![Cost::MAX / 2; size];\n    dist[s] = 0;\n    for\
    \ _ in 0..size {\n        let mut update = false;\n        for edge in edge {\n\
    \            if dist[edge.from] == Cost::MAX / 2 {\n                continue;\n\
    \            }\n            if dist[edge.from] + edge.cost < dist[edge.to] {\n\
    \                dist[edge.to] = dist[edge.from] + edge.cost;\n              \
    \  update = true;\n            }\n        }\n        if !update {\n          \
    \  return (false, dist);\n        }\n    }\n    for _ in 0..size {\n        for\
    \ edge in edge {\n            if dist[edge.from] == Cost::MAX / 2 {\n        \
    \        continue;\n            }\n            if dist[edge.from] + edge.cost\
    \ < dist[edge.to] {\n                dist[edge.to] = -(Cost::MAX / 2);\n     \
    \       }\n        }\n    }\n    (true, dist)\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: graph/bellman-ford/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-14 21:35:32+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/grl_1_b/src/main.rs
documentation_of: graph/bellman-ford/src/lib.rs
layout: document
title: Bellman Ford
---

## Description
