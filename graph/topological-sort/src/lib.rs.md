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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{cmp::Reverse, collections::BinaryHeap};\n\n#[derive(Debug, Clone)]\n\
    pub struct TopologicalSort {\n    size: usize,\n    graph: Vec<Vec<usize>>,\n\
    }\n\nimpl TopologicalSort {\n    pub fn new(size: usize) -> Self {\n        Self\
    \ {\n            size,\n            graph: vec![vec![]; size],\n        }\n  \
    \  }\n\n    pub fn add_edge(&mut self, from: usize, to: usize) {\n        self.graph[from].push(to);\n\
    \    }\n\n    pub fn topological_sort(&mut self) -> Vec<usize> {\n        let\
    \ mut indegree = vec![0; self.size];\n        for from in 0..self.size {\n   \
    \         for i in 0..self.graph[from].len() {\n                let to = self.graph[from][i];\n\
    \                indegree[to] += 1;\n            }\n        }\n        let mut\
    \ heap = BinaryHeap::new();\n        for from in 0..self.size {\n            if\
    \ indegree[from] == 0 {\n                heap.push(Reverse(from));\n         \
    \   }\n        }\n        let mut res = vec![];\n        while let Some(Reverse(from))\
    \ = heap.pop() {\n            res.push(from);\n            for i in 0..self.graph[from].len()\
    \ {\n                let to = self.graph[from][i];\n                indegree[to]\
    \ -= 1;\n                if indegree[to] == 0 {\n                    heap.push(Reverse(to));\n\
    \                }\n            }\n        }\n        if res.len() != self.size\
    \ {\n            return vec![];\n        }\n        res\n    }\n}\n\n// lexicographic\
    \ order\npub fn topological_sort(size: usize, graph: &Vec<Vec<usize>>) -> Vec<usize>\
    \ {\n    let mut indegree = vec![0; size];\n    for from in 0..size {\n      \
    \  for i in 0..graph[from].len() {\n            let to = graph[from][i];\n   \
    \         indegree[to] += 1;\n        }\n    }\n    let mut heap = BinaryHeap::new();\n\
    \    for from in 0..size {\n        if indegree[from] == 0 {\n            heap.push(Reverse(from));\n\
    \        }\n    }\n    let mut res = vec![];\n    while let Some(Reverse(from))\
    \ = heap.pop() {\n        res.push(from);\n        for i in 0..graph[from].len()\
    \ {\n            let to = graph[from][i];\n            indegree[to] -= 1;\n  \
    \          if indegree[to] == 0 {\n                heap.push(Reverse(to));\n \
    \           }\n        }\n    }\n    if res.len() != size {\n        return vec![];\n\
    \    }\n    res\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: graph/topological-sort/src/lib.rs
  requiredBy: []
  timestamp: '2024-11-23 20:15:14+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: graph/topological-sort/src/lib.rs
layout: document
redirect_from:
- /library/graph/topological-sort/src/lib.rs
- /library/graph/topological-sort/src/lib.rs.html
title: graph/topological-sort/src/lib.rs
---
