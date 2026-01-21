---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/grl_6_a_maxflow/src/main.rs
    title: verification/aizu-online-judge/grl_6_a_maxflow/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/grl_7_a_maxflow/src/main.rs
    title: verification/aizu-online-judge/grl_7_a_maxflow/src/main.rs
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
  code: "use std::collections::VecDeque;\n\npub trait CapTrait:\n    Copy + PartialOrd\
    \ + Ord + PartialEq + Eq + std::ops::Add<Output = Self> + std::ops::AddAssign\
    \ + std::ops::Sub<Output = Self> + std::ops::SubAssign + Default\n{\n    fn max_value()\
    \ -> Self;\n    fn min_value() -> Self;\n}\n\nmacro_rules! impl_cap_trait {\n\
    \    ($($type:ty), *) => {\n        $(\n            impl CapTrait for $type {\n\
    \                fn max_value() -> Self { <$type>::MAX }\n                fn min_value()\
    \ -> Self { <$type>::MIN }\n            }\n        )*\n    };\n}\n\nimpl_cap_trait!(u32,\
    \ i32, u64, i64, usize, isize);\n\n#[derive(Clone, Debug)]\npub struct Edge<Cap>\
    \ {\n    pub from: usize,\n    pub to: usize,\n    pub cap: Cap,\n    pub flow:\
    \ Cap,\n}\n\n#[derive(Clone, Debug)]\nstruct InnerEdge<Cap> {\n    to: usize,\n\
    \    rev: usize,\n    cap: Cap,\n}\n\npub struct Maxflow<Cap> {\n    n: usize,\n\
    \    g: Vec<Vec<InnerEdge<Cap>>>,\n    pos: Vec<(usize, usize)>,\n}\n\nimpl<Cap:\
    \ CapTrait> Maxflow<Cap> {\n    pub fn new(n: usize) -> Self {\n        Maxflow\
    \ {\n            n,\n            g: vec![vec![]; n],\n            pos: vec![],\n\
    \        }\n    }\n\n    pub fn add_edge(&mut self, from: usize, to: usize, cap:\
    \ Cap) -> usize {\n        assert!(from < self.n);\n        assert!(to < self.n);\n\
    \        assert!(Cap::default() <= cap);\n        let m = self.pos.len();\n  \
    \      self.pos.push((from, self.g[from].len()));\n        let from_id = self.g[from].len();\n\
    \        let to_id = self.g[to].len() + if from == to { 1 } else { 0 };\n    \
    \    self.g[from].push(InnerEdge { to, rev: to_id, cap });\n        self.g[to].push(InnerEdge\
    \ {\n            to: from,\n            rev: from_id,\n            cap: Cap::default(),\n\
    \        });\n        m\n    }\n\n    pub fn get_edge(&self, i: usize) -> Edge<Cap>\
    \ {\n        assert!(i < self.pos.len());\n        let (from, from_id) = self.pos[i];\n\
    \        let e = &self.g[from][from_id];\n        let re = &self.g[e.to][e.rev];\n\
    \        Edge {\n            from,\n            to: e.to,\n            cap: e.cap\
    \ + re.cap,\n            flow: re.cap,\n        }\n    }\n\n    pub fn edges(&self)\
    \ -> Vec<Edge<Cap>> {\n        (0..self.pos.len()).map(|i| self.get_edge(i)).collect()\n\
    \    }\n\n    pub fn change_edge(&mut self, i: usize, new_cap: Cap, new_flow:\
    \ Cap) {\n        assert!(i < self.pos.len());\n        assert!(Cap::default()\
    \ <= new_flow);\n        assert!(new_flow <= new_cap);\n        let (from, from_id)\
    \ = self.pos[i];\n        let e = &self.g[from][from_id];\n        let to = e.to;\n\
    \        let rev = e.rev;\n        self.g[from][from_id].cap = new_cap - new_flow;\n\
    \        self.g[to][rev].cap = new_flow;\n    }\n\n    pub fn flow(&mut self,\
    \ s: usize, t: usize) -> Cap {\n        self.flow_with_limit(s, t, Cap::max_value())\n\
    \    }\n\n    fn bfs(&mut self, s: usize, t: usize, que: &mut VecDeque<usize>,\
    \ level: &mut [i64]) {\n        level.fill(-1);\n        level[s] = 0;\n     \
    \   que.clear();\n        que.push_back(s);\n        while let Some(v) = que.pop_front()\
    \ {\n            for e in &self.g[v] {\n                if e.cap == Cap::default()\
    \ || level[e.to] >= 0 {\n                    continue;\n                }\n  \
    \              level[e.to] = level[v] + 1;\n                if e.to == t {\n \
    \                   return;\n                }\n                que.push_back(e.to);\n\
    \            }\n        }\n    }\n\n    fn dfs(&mut self, s: usize, t: usize,\
    \ v: usize, up: Cap, level: &mut [i64], iter: &mut [usize]) -> Cap {\n       \
    \ if v == s {\n            return up;\n        }\n        let mut res = Cap::default();\n\
    \        let level_v = level[v];\n        while iter[v] < self.g[v].len() {\n\
    \            let i = iter[v];\n            let e = &self.g[v][i];\n          \
    \  let to = e.to;\n            let rev = e.rev;\n            if level_v <= level[to]\
    \ || self.g[to][rev].cap == Cap::default() {\n                iter[v] += 1;\n\
    \                continue;\n            }\n            let cap = self.g[to][rev].cap;\n\
    \            let d = self.dfs(s, t, to, std::cmp::min(up - res, cap), level, iter);\n\
    \            if d <= Cap::default() {\n                iter[v] += 1;\n       \
    \         continue;\n            }\n            self.g[v][i].cap += d;\n     \
    \       self.g[to][rev].cap -= d;\n            res += d;\n            if res ==\
    \ up {\n                return res;\n            }\n            iter[v] += 1;\n\
    \        }\n        level[v] = self.n as i64;\n        res\n    }\n\n    pub fn\
    \ flow_with_limit(&mut self, s: usize, t: usize, flow_limit: Cap) -> Cap {\n \
    \       assert!(s < self.n);\n        assert!(t < self.n);\n        assert!(s\
    \ != t);\n\n        let mut level = vec![0; self.n];\n        let mut iter = vec![0;\
    \ self.n];\n        let mut que = VecDeque::new();\n\n        let mut flow = Cap::default();\n\
    \        while flow < flow_limit {\n            self.bfs(s, t, &mut que, &mut\
    \ level);\n            if level[t] == -1 {\n                break;\n         \
    \   }\n\n            iter.fill(0);\n            let f = self.dfs(s, t, t, flow_limit\
    \ - flow, &mut level, &mut iter);\n            if f == Cap::default() {\n    \
    \            break;\n            }\n            flow += f;\n        }\n      \
    \  flow\n    }\n\n    pub fn min_cut(&self, s: usize) -> Vec<bool> {\n       \
    \ let mut visited = vec![false; self.n];\n        let mut que = VecDeque::new();\n\
    \        que.push_back(s);\n        while let Some(p) = que.pop_front() {\n  \
    \          visited[p] = true;\n            for e in &self.g[p] {\n           \
    \     if e.cap > Cap::default() && !visited[e.to] {\n                    visited[e.to]\
    \ = true;\n                    que.push_back(e.to);\n                }\n     \
    \       }\n        }\n        visited\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: graph/maxflow/src/lib.rs
  requiredBy: []
  timestamp: '2026-01-01 00:11:18+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/grl_6_a_maxflow/src/main.rs
  - verification/aizu-online-judge/grl_7_a_maxflow/src/main.rs
documentation_of: graph/maxflow/src/lib.rs
layout: document
title: Maxflow
---

## Description

## Reference
- [https://atcoder.github.io/ac-library/production/document_en/maxflow.html](https://atcoder.github.io/ac-library/production/document_en/maxflow.html)
