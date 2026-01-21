---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/grl_3_a/src/main.rs
    title: verification/aizu-online-judge/grl_3_a/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/grl_3_b/src/main.rs
    title: verification/aizu-online-judge/grl_3_b/src/main.rs
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
  code: "#[derive(Debug, Clone, Copy)]\npub struct Edge {\n    from: usize,\n    to:\
    \ usize,\n}\n\nimpl Edge {\n    pub fn new(from: usize, to: usize) -> Self {\n\
    \        Self { from, to }\n    }\n}\n\n#[derive(Debug, Clone)]\npub struct LowLink\
    \ {\n    size: usize,\n    graph: Vec<Vec<Edge>>,\n    ord: Vec<usize>,\n    low:\
    \ Vec<usize>,\n    used: Vec<bool>,\n    articulation: Vec<usize>, // articulation\
    \ points\n    bridge: Vec<Edge>,        // bridges\n}\n\nimpl LowLink {\n    pub\
    \ fn new(size: usize) -> Self {\n        Self {\n            size,\n         \
    \   graph: vec![vec![]; size],\n            ord: vec![0; size],\n            low:\
    \ vec![0; size],\n            used: vec![false; size],\n            articulation:\
    \ vec![],\n            bridge: vec![],\n        }\n    }\n\n    pub fn add_edge(&mut\
    \ self, from: usize, to: usize) {\n        assert!(from < self.size);\n      \
    \  assert!(to < self.size);\n        self.graph[from].push(Edge::new(from, to));\n\
    \    }\n\n    pub fn build(&mut self) {\n        let mut k = 0;\n        for i\
    \ in 0..self.size {\n            if !self.used[i] {\n                k = self.dfs(i,\
    \ k, None);\n            }\n        }\n    }\n\n    fn dfs(&mut self, v: usize,\
    \ mut k: usize, par: Option<usize>) -> usize {\n        self.used[v] = true;\n\
    \        self.ord[v] = k;\n        self.low[v] = self.ord[v];\n        k += 1;\n\
    \        let mut is_articulation = false;\n        let mut count = 0; // number\
    \ of child\n        for &edge in &self.graph[v].clone() {\n            if !self.used[edge.to]\
    \ {\n                count += 1;\n                k = self.dfs(edge.to, k, Some(v));\n\
    \                self.low[v] = self.low[v].min(self.low[edge.to]);\n         \
    \       if par.is_some() && self.ord[v] <= self.low[edge.to] {\n             \
    \       is_articulation = true;\n                }\n                if self.ord[v]\
    \ < self.low[edge.to] {\n                    self.bridge.push(edge);\n       \
    \         }\n            } else if Some(edge.to) != par {\n                self.low[v]\
    \ = self.low[v].min(self.ord[edge.to]);\n            }\n        }\n        if\
    \ par.is_none() && count > 1 {\n            is_articulation = true;\n        }\n\
    \        if is_articulation {\n            self.articulation.push(v);\n      \
    \  }\n        k\n    }\n\n    pub fn articulation(&self) -> Vec<usize> {\n   \
    \     self.articulation.clone()\n    }\n\n    pub fn bridge(&self) -> Vec<(usize,\
    \ usize)> {\n        self.bridge.iter().map(|&edge| (edge.from, edge.to)).collect()\n\
    \    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: graph/low-link/src/lib.rs
  requiredBy: []
  timestamp: '2026-01-01 18:54:13+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/grl_3_a/src/main.rs
  - verification/aizu-online-judge/grl_3_b/src/main.rs
documentation_of: graph/low-link/src/lib.rs
layout: document
title: Low Link
---

## Description

## Reference
- [https://kntychance.hatenablog.jp/entry/2022/09/16/161858](https://kntychance.hatenablog.jp/entry/2022/09/16/161858)
- [https://sen-comp.hatenablog.com/entry/2022/11/17/233858](https://sen-comp.hatenablog.com/entry/2022/11/17/233858)
- [https://kagamiz.hatenablog.com/entry/2013/10/05/005213](https://kagamiz.hatenablog.com/entry/2013/10/05/005213)
<!--- [https://ei1333.github.io/library/graph/others/low-link.hpp](https://ei1333.github.io/library/graph/others/low-link.hpp)-->
<!--- [https://github.com/drken1215/algorithm/blob/master/Graph/low_link.cpp](https://github.com/drken1215/algorithm/blob/master/Graph/low_link.cpp)-->
