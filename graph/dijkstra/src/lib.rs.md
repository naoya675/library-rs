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
    \ Clone, Copy)]\npub struct Edge {\n    from: usize,\n    to: usize,\n    cost:\
    \ Cost,\n}\n\nimpl Edge {\n    pub fn new(from: usize, to: usize, cost: Cost)\
    \ -> Self {\n        Self { from, to, cost }\n    }\n}\n\n#[derive(Debug, Clone)]\n\
    pub struct Dijkstra {\n    size: usize,\n    graph: Vec<Vec<Edge>>,\n}\n\nimpl\
    \ Dijkstra {\n    pub fn new(size: usize) -> Self {\n        Self {\n        \
    \    size,\n            graph: vec![vec![]; size],\n        }\n    }\n\n    pub\
    \ fn add_edge(&mut self, from: usize, to: usize, cost: Cost) {\n        self.graph[from].push(Edge::new(from,\
    \ to, cost));\n    }\n\n    pub fn dijkstra(&mut self, s: usize) -> Vec<Cost>\
    \ {\n        let mut dist = vec![Cost::MAX / 4; self.size];\n        dist[s] =\
    \ 0;\n        let mut heap: BinaryHeap<(Cost, usize)> = BinaryHeap::new();\n \
    \       heap.push((-dist[s], s));\n        while let Some((d, from)) = heap.pop()\
    \ {\n            if dist[from] < -d {\n                continue;\n           \
    \ }\n            for edge in &self.graph[from] {\n                if dist[edge.to]\
    \ > dist[from] + edge.cost {\n                    dist[edge.to] = dist[from] +\
    \ edge.cost;\n                    heap.push((-dist[edge.to], edge.to));\n    \
    \            }\n            }\n        }\n        dist\n    }\n\n    pub fn dijkstra_prev(&mut\
    \ self, s: usize) -> (Vec<Cost>, Vec<usize>) {\n        let mut dist = vec![Cost::MAX\
    \ / 4; self.size];\n        let mut prev = vec![self.size; self.size];\n     \
    \   dist[s] = 0;\n        let mut heap: BinaryHeap<(Cost, usize)> = BinaryHeap::new();\n\
    \        heap.push((-dist[s], s));\n        while let Some((d, from)) = heap.pop()\
    \ {\n            if dist[from] < -d {\n                continue;\n           \
    \ }\n            for edge in &self.graph[from] {\n                if dist[edge.to]\
    \ > dist[from] + edge.cost {\n                    dist[edge.to] = dist[from] +\
    \ edge.cost;\n                    prev[edge.to] = from;\n                    heap.push((-dist[edge.to],\
    \ edge.to));\n                }\n            }\n        }\n        (dist, prev)\n\
    \    }\n}\n\n/*\npub fn dijkstra(size: usize, graph: &Vec<Vec<(usize, i64)>>,\
    \ s: usize) -> Vec<i64> {\n    let mut dist = vec![i64::MAX / 4; size];\n    dist[s]\
    \ = 0;\n    let mut heap: BinaryHeap<(i64, usize)> = BinaryHeap::new();\n    heap.push((-dist[s],\
    \ s));\n    while let Some((d, from)) = heap.pop() {\n        if dist[from] <\
    \ -d {\n            continue;\n        }\n        for &(to, cost) in &graph[from]\
    \ {\n            if dist[to] > dist[from] + cost {\n                dist[to] =\
    \ dist[from] + cost;\n                heap.push((-dist[to], to));\n          \
    \  }\n        }\n    }\n    dist\n}\n*/\n"
  dependsOn: []
  isVerificationFile: false
  path: graph/dijkstra/src/lib.rs
  requiredBy: []
  timestamp: '2025-04-13 18:21:44+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/grl_1_a/src/main.rs
documentation_of: graph/dijkstra/src/lib.rs
layout: document
title: Dijkstra
---

## Description
