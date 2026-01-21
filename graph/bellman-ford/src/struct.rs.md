---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "#[derive(Debug, Clone, Copy)]\npub struct Edge {\n    from: usize,\n    to:\
    \ usize,\n    cost: i64,\n}\n\nimpl Edge {\n    pub fn new(from: usize, to: usize,\
    \ cost: i64) -> Self {\n        Self { from, to, cost }\n    }\n}\n\n#[derive(Debug,\
    \ Clone)]\npub struct BellmanFord {\n    size: usize,\n    edge: Vec<Edge>,\n\
    }\n\nimpl BellmanFord {\n    pub fn new(size: usize) -> Self {\n        Self {\
    \ size, edge: vec![] }\n    }\n\n    pub fn add_edge(&mut self, from: usize, to:\
    \ usize, cost: i64) {\n        self.edge.push(Edge::new(from, to, cost));\n  \
    \  }\n\n    pub fn bellman_ford(&mut self, s: usize) -> (bool, Vec<i64>) {\n \
    \       let mut dist = vec![i64::MAX / 4; self.size];\n        dist[s] = 0;\n\
    \        for _ in 0..self.size {\n            let mut update = false;\n      \
    \      for edge in &self.edge {\n                if dist[edge.from] == i64::MAX\
    \ / 4 {\n                    continue;\n                }\n                if\
    \ dist[edge.from] + edge.cost < dist[edge.to] {\n                    dist[edge.to]\
    \ = dist[edge.from] + edge.cost;\n                    update = true;\n       \
    \         }\n            }\n            if !update {\n                return (false,\
    \ dist);\n            }\n        }\n        for _ in 0..self.size {\n        \
    \    for edge in &self.edge {\n                if dist[edge.from] == i64::MAX\
    \ / 4 {\n                    continue;\n                }\n                if\
    \ dist[edge.from] + edge.cost < dist[edge.to] {\n                    dist[edge.to]\
    \ = -(i64::MAX / 4);\n                }\n            }\n        }\n        (true,\
    \ dist)\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: graph/bellman-ford/src/struct.rs
  requiredBy: []
  timestamp: '2025-12-08 23:17:07+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: graph/bellman-ford/src/struct.rs
layout: document
redirect_from:
- /library/graph/bellman-ford/src/struct.rs
- /library/graph/bellman-ford/src/struct.rs.html
title: graph/bellman-ford/src/struct.rs
---
