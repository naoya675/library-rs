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
    \ Clone)]\npub struct WarshallFloyd {\n    size: usize,\n    edge: Vec<Edge>,\n\
    }\n\nimpl WarshallFloyd {\n    pub fn new(size: usize) -> Self {\n        Self\
    \ { size, edge: vec![] }\n    }\n\n    pub fn add_edge(&mut self, from: usize,\
    \ to: usize, cost: i64) {\n        self.edge.push(Edge::new(from, to, cost));\n\
    \    }\n\n    pub fn warshall_floyd(&mut self) -> (bool, Vec<Vec<i64>>) {\n  \
    \      let mut dist = vec![vec![i64::MAX / 4; self.size]; self.size];\n      \
    \  for i in 0..self.size {\n            dist[i][i] = 0;\n        }\n        for\
    \ edge in &self.edge {\n            dist[edge.from][edge.to] = edge.cost;\n  \
    \      }\n        for k in 0..self.size {\n            for i in 0..self.size {\n\
    \                for j in 0..self.size {\n                    dist[i][j] = dist[i][j].min(dist[i][k]\
    \ + dist[k][j])\n                }\n            }\n        }\n        for i in\
    \ 0..self.size {\n            if dist[i][i] < 0 {\n                return (true,\
    \ dist);\n            }\n        }\n        (false, dist)\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: graph/warshall-floyd/src/struct.rs
  requiredBy: []
  timestamp: '2025-12-08 23:17:07+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: graph/warshall-floyd/src/struct.rs
layout: document
redirect_from:
- /library/graph/warshall-floyd/src/struct.rs
- /library/graph/warshall-floyd/src/struct.rs.html
title: graph/warshall-floyd/src/struct.rs
---
