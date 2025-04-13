---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/union-find/src/lib.rs
    title: Union Find
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/grl_2_a/src/main.rs
    title: verification/aizu-online-judge/grl_2_a/src/main.rs
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
  code: "use union_find::UnionFind;\n\npub type Cost = i64;\n\n#[derive(Debug, Clone,\
    \ Copy)]\npub struct Edge {\n    from: usize,\n    to: usize,\n    cost: Cost,\n\
    }\n\nimpl Edge {\n    pub fn new(from: usize, to: usize, cost: Cost) -> Self {\n\
    \        Self { from, to, cost }\n    }\n}\n\n#[derive(Debug, Clone)]\npub struct\
    \ Kruskal {\n    size: usize,\n    edge: Vec<Edge>,\n}\n\nimpl Kruskal {\n   \
    \ pub fn new(size: usize) -> Self {\n        Self { size, edge: vec![] }\n   \
    \ }\n\n    pub fn add_edge(&mut self, from: usize, to: usize, cost: Cost) {\n\
    \        self.edge.push(Edge::new(from, to, cost));\n    }\n\n    pub fn minimum_spanning_tree(&mut\
    \ self) -> (Cost, Vec<Edge>) {\n        self.edge.sort_by(|a, b| a.cost.cmp(&b.cost));\n\
    \        let mut uf = UnionFind::new(self.size);\n        let mut res = 0;\n \
    \       let mut res_edge = vec![];\n        for edge in &self.edge {\n       \
    \     if uf.merge(edge.to, edge.from) {\n                res += edge.cost;\n \
    \               res_edge.push(edge.clone());\n            }\n        }\n     \
    \   (res, res_edge)\n    }\n\n    pub fn maximum_spanning_tree(&mut self) -> (Cost,\
    \ Vec<Edge>) {\n        self.edge.sort_by(|a, b| b.cost.cmp(&a.cost));\n     \
    \   let mut uf = UnionFind::new(self.size);\n        let mut res = 0;\n      \
    \  let mut res_edge = vec![];\n        for edge in &self.edge {\n            if\
    \ uf.merge(edge.to, edge.from) {\n                res += edge.cost;\n        \
    \        res_edge.push(edge.clone());\n            }\n        }\n        (res,\
    \ res_edge)\n    }\n}\n\n/*\npub fn minimum_spanning_tree(\n    size: usize,\n\
    \    edge: &mut Vec<(usize, usize, i64)>,\n) -> (Cost, Vec<(usize, usize, i64)>)\
    \ {\n    edge.sort_by(|a, b| a.2.cmp(&b.2));\n    let mut uf = UnionFind::new(size);\n\
    \    let mut res = 0;\n    let mut res_edge = vec![];\n    for &mut (from, to,\
    \ cost) in edge {\n        if uf.merge(to, from) {\n            res += cost;\n\
    \            res_edge.push((from, to, cost));\n        }\n    }\n    (res, res_edge)\n\
    }\n\npub fn maximum_spanning_tree(\n    size: usize,\n    edge: &mut Vec<(usize,\
    \ usize, i64)>,\n) -> (Cost, Vec<(usize, usize, i64)>) {\n    edge.sort_by(|a,\
    \ b| b.2.cmp(&a.2));\n    let mut uf = UnionFind::new(size);\n    let mut res\
    \ = 0;\n    let mut res_edge = vec![];\n    for &mut (from, to, cost) in edge\
    \ {\n        if uf.merge(to, from) {\n            res += cost;\n            res_edge.push((from,\
    \ to, cost));\n        }\n    }\n    (res, res_edge)\n}\n*/\n"
  dependsOn:
  - data-structure/union-find/src/lib.rs
  isVerificationFile: false
  path: graph/kruskal/src/lib.rs
  requiredBy: []
  timestamp: '2025-04-13 18:21:44+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/grl_2_a/src/main.rs
documentation_of: graph/kruskal/src/lib.rs
layout: document
title: Kruskal
---

## Description
