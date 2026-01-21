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
  code: "use union_find::UnionFind;\n\n#[derive(Debug, Clone, Copy)]\npub struct Edge\
    \ {\n    from: usize,\n    to: usize,\n    cost: i64,\n}\n\nimpl Edge {\n    pub\
    \ fn new(from: usize, to: usize, cost: i64) -> Self {\n        Self { from, to,\
    \ cost }\n    }\n}\n\n#[derive(Debug, Clone)]\npub struct Kruskal {\n    size:\
    \ usize,\n    edge: Vec<Edge>,\n}\n\nimpl Kruskal {\n    pub fn new(size: usize)\
    \ -> Self {\n        Self { size, edge: vec![] }\n    }\n\n    pub fn add_edge(&mut\
    \ self, from: usize, to: usize, cost: i64) {\n        self.edge.push(Edge::new(from,\
    \ to, cost));\n    }\n\n    pub fn minimum_spanning_tree(&mut self) -> (i64, Vec<Edge>)\
    \ {\n        self.edge.sort_by(|a, b| a.cost.cmp(&b.cost));\n        let mut uf\
    \ = UnionFind::new(self.size);\n        let mut res = 0;\n        let mut res_edge\
    \ = vec![];\n        for edge in &self.edge {\n            if uf.same(edge.from,\
    \ edge.to) {\n                continue;\n            }\n            uf.merge(edge.from,\
    \ edge.to);\n            res += edge.cost;\n            res_edge.push(edge.clone());\n\
    \        }\n        (res, res_edge)\n    }\n\n    pub fn maximum_spanning_tree(&mut\
    \ self) -> (i64, Vec<Edge>) {\n        self.edge.sort_by(|a, b| b.cost.cmp(&a.cost));\n\
    \        let mut uf = UnionFind::new(self.size);\n        let mut res = 0;\n \
    \       let mut res_edge = vec![];\n        for edge in &self.edge {\n       \
    \     if uf.same(edge.from, edge.to) {\n                continue;\n          \
    \  }\n            uf.merge(edge.from, edge.to);\n            res += edge.cost;\n\
    \            res_edge.push(edge.clone());\n        }\n        (res, res_edge)\n\
    \    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: graph/kruskal/src/struct.rs
  requiredBy: []
  timestamp: '2025-12-08 23:17:07+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: graph/kruskal/src/struct.rs
layout: document
redirect_from:
- /library/graph/kruskal/src/struct.rs
- /library/graph/kruskal/src/struct.rs.html
title: graph/kruskal/src/struct.rs
---
