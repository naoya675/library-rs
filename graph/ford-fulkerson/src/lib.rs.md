---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/grl_6_a/src/main.rs
    title: verification/aizu-online-judge/grl_6_a/src/main.rs
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
  code: "pub type Cap = i64;\n\n#[derive(Debug, Clone, Copy)]\npub struct Edge {\n\
    \    to: usize,\n    cap: Cap,\n    rev: usize,\n}\n\nimpl Edge {\n    pub fn\
    \ new(to: usize, cap: Cap, rev: usize) -> Self {\n        Self { to, cap, rev\
    \ }\n    }\n}\n\n#[derive(Debug, Clone)]\npub struct FordFulkerson {\n    size:\
    \ usize,\n    used: Vec<bool>,\n    graph: Vec<Vec<Edge>>,\n}\n\nimpl FordFulkerson\
    \ {\n    pub fn new(size: usize) -> Self {\n        Self {\n            size,\n\
    \            used: vec![false; size],\n            graph: vec![vec![]; size],\n\
    \        }\n    }\n\n    pub fn add_edge(&mut self, a: usize, b: usize, c: Cap)\
    \ {\n        let alen = self.graph[a].len();\n        let blen = self.graph[b].len();\n\
    \        self.graph[a].push(Edge::new(b, c, blen));\n        self.graph[b].push(Edge::new(a,\
    \ 0, alen));\n    }\n\n    pub fn flow(&mut self, s: usize, t: usize) -> Cap {\n\
    \        let mut total_flow = 0;\n        loop {\n            for i in 0..self.size\
    \ {\n                self.used[i] = false;\n            }\n            let flow\
    \ = self.dfs(s, t, Cap::MAX);\n            if flow == 0 {\n                break;\n\
    \            }\n            total_flow += flow;\n        }\n        total_flow\n\
    \    }\n\n    fn dfs(&mut self, v: usize, t: usize, f: Cap) -> Cap {\n       \
    \ if v == t {\n            return f;\n        }\n        self.used[v] = true;\n\
    \        for i in 0..self.graph[v].len() {\n            let u = self.graph[v][i];\n\
    \            if u.cap == 0 {\n                continue;\n            }\n     \
    \       if self.used[u.to] {\n                continue;\n            }\n     \
    \       let flow = self.dfs(u.to, t, f.min(u.cap));\n            if flow > 0 {\n\
    \                self.graph[v][i].cap -= flow;\n                self.graph[u.to][u.rev].cap\
    \ += flow;\n                return flow;\n            }\n        }\n        0\n\
    \    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: graph/ford-fulkerson/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-13 01:08:14+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/grl_6_a/src/main.rs
documentation_of: graph/ford-fulkerson/src/lib.rs
layout: document
title: Ford Fulkerson
---

## Description
