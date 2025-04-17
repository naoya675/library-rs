---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/grl_5_a/src/main.rs
    title: verification/aizu-online-judge/grl_5_a/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://atcoder.jp/contests/abc222/editorial/2749
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "#[derive(Debug, Clone)]\npub struct Edge<Cost> {\n    from: usize,\n    to:\
    \ usize,\n    cost: Cost,\n}\n\nimpl<Cost: Copy> Edge<Cost> {\n    pub fn new(from:\
    \ usize, to: usize, cost: Cost) -> Self {\n        Self { from, to, cost }\n \
    \   }\n}\n\n#[derive(Debug, Clone)]\npub struct Rerooting<\n    Cost,\n    Data,\n\
    \    Merge: Fn(Data, Data) -> Data,\n    E: Fn() -> Data,\n    Leaf: Fn() -> Data,\n\
    \    Apply: Fn(Data, usize, usize, Cost) -> Data,\n> {\n    graph: Vec<Vec<Edge<Cost>>>,\n\
    \    dp: Vec<Data>,\n    memo: Vec<Data>,\n    merge: Merge,\n    e: E,\n    leaf:\
    \ Leaf,\n    apply: Apply,\n    n: usize,\n}\n\nimpl<\n        Cost: Copy + Default,\n\
    \        Data: Copy,\n        Merge: Fn(Data, Data) -> Data,\n        E: Fn()\
    \ -> Data,\n        Leaf: Fn() -> Data,\n        Apply: Fn(Data, usize, usize,\
    \ Cost) -> Data,\n    > Rerooting<Cost, Data, Merge, E, Leaf, Apply>\n{\n    pub\
    \ fn new(n: usize, merge: Merge, e: E, leaf: Leaf, apply: Apply) -> Self {\n \
    \       Self {\n            graph: vec![vec![]; n],\n            dp: vec![],\n\
    \            memo: vec![],\n            merge,\n            e,\n            leaf,\n\
    \            apply,\n            n,\n        }\n    }\n\n    pub fn add_edge(&mut\
    \ self, from: usize, to: usize, cost: Cost) {\n        self.graph[from].push(Edge::new(from,\
    \ to, cost));\n    }\n\n    pub fn run(&mut self) -> Vec<Data> {\n        self.memo.resize(self.n,\
    \ (self.e)());\n        self.dp.resize(self.n, (self.e)());\n        self.dfs1(0,\
    \ usize::MAX);\n        self.dfs2(0, usize::MAX, (self.e)());\n        self.dp.clone()\n\
    \    }\n\n    fn dfs1(&mut self, c: usize, p: usize) {\n        let mut upd =\
    \ false;\n        for edge in self.graph[c].clone() {\n            if edge.to\
    \ == p {\n                continue;\n            }\n            self.dfs1(edge.to,\
    \ c);\n            upd = true;\n            self.memo[c] = (self.merge)(\n   \
    \             self.memo[c],\n                (self.apply)(self.memo[edge.to],\
    \ edge.to, c, edge.cost),\n            );\n        }\n        if !upd {\n    \
    \        self.memo[c] = (self.leaf)();\n        }\n    }\n\n    fn dfs2(&mut self,\
    \ c: usize, p: usize, val: Data) {\n        let mut ds = vec![(self.e)()];\n \
    \       for edge in self.graph[c].clone() {\n            if edge.to != p {\n \
    \               ds.push((self.apply)(self.memo[edge.to], edge.to, c, edge.cost));\n\
    \            } else {\n                ds.push((self.apply)(val, edge.to, c, edge.cost));\n\
    \            }\n        }\n        let n = ds.len();\n        let mut idx = 1;\n\
    \        let mut head = vec![(self.e)(); n + 1];\n        let mut tail = vec![(self.e)();\
    \ n + 1];\n        for i in 0..n {\n            head[i + 1] = (self.merge)(head[i],\
    \ ds[i]);\n        }\n        for i in (0..n).rev() {\n            tail[i] = (self.merge)(tail[i\
    \ + 1], ds[i]);\n        }\n        self.dp[c] = head[n];\n        for edge in\
    \ self.graph[c].clone() {\n            if edge.to != p {\n                self.dfs2(edge.to,\
    \ c, (self.merge)(head[idx], tail[idx + 1]));\n            }\n            idx\
    \ += 1;\n        }\n    }\n\n    /* [warning]\n    fn dfs2(&mut self, c: usize,\
    \ p: usize, val: Data) {\n        let mut ds = vec![val];\n        for edge in\
    \ self.graph[c].clone() {\n            if edge.to == p {\n                continue;\n\
    \            }\n            ds.push((self.apply)(self.memo[edge.to], edge.to,\
    \ c, edge.cost));\n        }\n        let n = ds.len();\n        let mut idx =\
    \ 1;\n        let mut head = vec![(self.e)(); n + 1];\n        let mut tail =\
    \ vec![(self.e)(); n + 1];\n        for i in 0..n {\n            head[i + 1] =\
    \ (self.merge)(head[i], ds[i]);\n        }\n        for i in (0..n).rev() {\n\
    \            tail[i] = (self.merge)(tail[i + 1], ds[i]);\n        }\n        self.dp[c]\
    \ = head[n];\n        for edge in self.graph[c].clone() {\n            if edge.to\
    \ == p {\n                continue;\n            }\n            let sub = (self.merge)(head[idx],\
    \ tail[idx + 1]);\n            self.dfs2(edge.to, c, (self.apply)(sub, c, edge.to,\
    \ edge.cost));\n            idx += 1;\n        }\n    }\n    */\n}\n\npub struct\
    \ RerootingDiameter;\n\nimpl RerootingDiameter {\n    pub fn new(\n        n:\
    \ usize,\n    ) -> Rerooting<\n        usize,\n        usize,\n        impl Fn(usize,\
    \ usize) -> usize,\n        impl Fn() -> usize,\n        impl Fn() -> usize,\n\
    \        impl Fn(usize, usize, usize, usize) -> usize,\n    > {\n        let merge\
    \ = |a: usize, b: usize| std::cmp::max(a, b);\n        let e = || 0;\n       \
    \ let leaf = || 0;\n        let apply = |a: usize, _: usize, _: usize, w: usize|\
    \ a + w;\n        Rerooting::new(n, merge, e, leaf, apply)\n    }\n}\n\n// reference:\
    \ https://atcoder.jp/contests/abc222/editorial/2749\n"
  dependsOn: []
  isVerificationFile: false
  path: dynamic-programming/rerooting/src/lib.rs
  requiredBy: []
  timestamp: '2025-04-18 00:17:29+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/grl_5_a/src/main.rs
documentation_of: dynamic-programming/rerooting/src/lib.rs
layout: document
title: Rerooting
---

## Description

## Reference

[https://atcoder.jp/contests/abc222/editorial/2749](https://atcoder.jp/contests/abc222/editorial/2749)
