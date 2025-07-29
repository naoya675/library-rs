---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/grl_5_c/src/main.rs
    title: verification/aizu-online-judge/grl_5_c/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/grl_5_d/src/main.rs
    title: verification/aizu-online-judge/grl_5_d/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/grl_5_e/src/main.rs
    title: verification/aizu-online-judge/grl_5_e/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://nyaannyaan.github.io/library/tree/heavy-light-decomposition.hpp.html
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// reference: https://nyaannyaan.github.io/library/tree/heavy-light-decomposition.hpp.html\n\
    \n#[derive(Debug, Clone)]\npub struct Edge<Cost> {\n    from: usize,\n    to:\
    \ usize,\n    cost: Cost,\n}\n\nimpl<Cost: Copy> Edge<Cost> {\n    pub fn new(from:\
    \ usize, to: usize, cost: Cost) -> Self {\n        Self { from, to, cost }\n \
    \   }\n}\n\n#[derive(Debug, Clone)]\npub struct HeavyLightDecomposition<Cost>\
    \ {\n    graph: Vec<Vec<Edge<Cost>>>,\n    depth: Vec<usize>,\n    subtree: Vec<usize>,\n\
    \    head: Vec<usize>,\n    parent: Vec<usize>,\n    preorder: Vec<usize>,\n \
    \   postorder: Vec<usize>,\n    rev: Vec<usize>,\n    n: usize,\n    time: usize,\n\
    }\n\nimpl<Cost: Copy + Default> HeavyLightDecomposition<Cost> {\n    pub fn new(n:\
    \ usize) -> Self {\n        Self {\n            graph: vec![vec![]; n],\n    \
    \        depth: vec![0; n],\n            subtree: vec![0; n],\n            head:\
    \ vec![0; n],\n            parent: vec![0; n],\n            preorder: vec![0;\
    \ n],\n            postorder: vec![0; n],\n            rev: vec![0; n],\n    \
    \        n,\n            time: 0,\n        }\n    }\n\n    pub fn add_edge(&mut\
    \ self, from: usize, to: usize, cost: Cost) {\n        self.graph[from].push(Edge::new(from,\
    \ to, cost));\n    }\n\n    pub fn init(&mut self, root: usize) {\n        self.dfs1(root,\
    \ root);\n        self.time = 0;\n        self.head[root] = root;\n        self.dfs2(root,\
    \ root);\n    }\n\n    fn dfs1(&mut self, v: usize, p: usize) {\n        self.subtree[v]\
    \ = 1;\n        if !self.graph[v].is_empty() && self.graph[v][0].to == p {\n \
    \           let l = self.graph[v].len() - 1;\n            self.graph[v].swap(0,\
    \ l);\n        }\n        for (i, edge) in self.graph[v].clone().iter().enumerate()\
    \ {\n            if edge.to == p {\n                continue;\n            }\n\
    \            self.depth[edge.to] = self.depth[v] + 1;\n            self.dfs1(edge.to,\
    \ v);\n            self.subtree[v] = self.subtree[v] + self.subtree[edge.to];\n\
    \            if self.subtree[self.graph[v][0].to] < self.subtree[edge.to] {\n\
    \                self.graph[v].swap(0, i);\n            }\n        }\n    }\n\n\
    \    // heavy light decomposition\n    fn dfs2(&mut self, v: usize, p: usize)\
    \ {\n        self.parent[v] = p;\n        self.preorder[v] = self.time;\n    \
    \    self.time += 1;\n        self.rev[self.preorder[v]] = v;\n        for edge\
    \ in self.graph[v].clone() {\n            if edge.to == p {\n                continue;\n\
    \            }\n            self.head[edge.to] = if edge.to == self.graph[v][0].to\
    \ { self.head[v] } else { edge.to };\n            self.dfs2(edge.to, v);\n   \
    \     }\n        self.postorder[v] = self.time;\n    }\n\n    // [u, v)\n    fn\
    \ ascend(&self, mut u: usize, v: usize) -> Vec<(usize, usize)> {\n        assert!(self.lca(u,\
    \ v) == v);\n        let mut res = vec![];\n        loop {\n            if self.head[u]\
    \ != self.head[v] {\n                res.push((self.preorder[u], self.preorder[self.head[u]]));\n\
    \                u = self.parent[self.head[u]];\n            } else {\n      \
    \          break;\n            }\n        }\n        if u != v {\n           \
    \ res.push((self.preorder[u], self.preorder[v] + 1));\n        }\n        res\n\
    \    }\n\n    // (u, v]\n    fn descend(&self, u: usize, v: usize) -> Vec<(usize,\
    \ usize)> {\n        assert!(self.lca(u, v) == u);\n        if u == v {\n    \
    \        return vec![];\n        }\n        if self.head[u] == self.head[v] {\n\
    \            return vec![(self.preorder[u] + 1, self.preorder[v])];\n        }\n\
    \        let mut res = self.descend(u, self.parent[self.head[v]]);\n        res.push((self.preorder[self.head[v]],\
    \ self.preorder[v]));\n        res\n    }\n\n    pub fn distance(&self, u: usize,\
    \ v: usize) -> usize {\n        self.depth[u] + self.depth[v] - 2 * self.depth[self.lca(u,\
    \ v)]\n    }\n\n    // Level Ancestor\n    pub fn la(&self, mut v: usize, mut\
    \ k: usize) -> usize {\n        assert!(v < self.n);\n        assert!(k <= self.depth[v]);\n\
    \        loop {\n            let u = self.head[v];\n            if self.preorder[v]\
    \ - k >= self.preorder[u] {\n                return self.rev[self.preorder[v]\
    \ - k];\n            }\n            k -= self.preorder[v] - self.preorder[u] +\
    \ 1;\n            v = self.parent[u];\n        }\n    }\n\n    // Lowest Common\
    \ Ancestor\n    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {\n   \
    \     assert!(u < self.n);\n        assert!(v < self.n);\n        loop {\n   \
    \         if self.preorder[u] > self.preorder[v] {\n                std::mem::swap(&mut\
    \ u, &mut v);\n            }\n            if self.head[u] == self.head[v] {\n\
    \                return u;\n            }\n            v = self.parent[self.head[v]];\n\
    \        }\n    }\n\n    // unverify\n    pub fn for_each_subtree<F>(&self, v:\
    \ usize, mut f: F)\n    where\n        F: FnMut(usize, usize),\n    {\n      \
    \  assert!(v < self.n);\n        f(self.preorder[v], self.postorder[v]);\n   \
    \ }\n\n    // unverify\n    pub fn for_each_subtree_edge<F>(&self, v: usize, mut\
    \ f: F)\n    where\n        F: FnMut(usize, usize),\n    {\n        assert!(v\
    \ < self.n);\n        f(self.preorder[v] + 1, self.postorder[v]);\n    }\n\n \
    \   // noncommutative, unverify\n    pub fn for_each_noncommutative<F>(&mut self,\
    \ u: usize, v: usize, mut f: F)\n    where\n        F: FnMut(usize, usize),\n\
    \    {\n        let l = self.lca(u, v);\n        for (l, r) in self.ascend(u,\
    \ l) {\n            f(l + 1, r);\n        }\n        f(self.preorder[l], self.preorder[l\
    \ + 1]);\n        for (l, r) in self.descend(l, v) {\n            f(l, r + 1);\n\
    \        }\n    }\n\n    // noncommutative, unverify\n    pub fn for_each_noncommutative_edge<F>(&mut\
    \ self, u: usize, v: usize, mut f: F)\n    where\n        F: FnMut(usize, usize),\n\
    \    {\n        let l = self.lca(u, v);\n        for (l, r) in self.ascend(u,\
    \ l) {\n            f(l + 1, r);\n        }\n        for (l, r) in self.descend(l,\
    \ v) {\n            f(l, r + 1);\n        }\n    }\n\n    // unverify\n    pub\
    \ fn for_each<F>(&mut self, u: usize, v: usize, mut f: F)\n    where\n       \
    \ F: FnMut(usize, usize),\n    {\n        let l = self.lca(u, v);\n        for\
    \ (l, r) in self.ascend(u, l) {\n            f(r.min(l + 1), r.max(l + 1));\n\
    \        }\n        f(self.preorder[l], self.preorder[l + 1]);\n        for (l,\
    \ r) in self.descend(l, v) {\n            f(l.min(r + 1), l.max(r + 1));\n   \
    \     }\n    }\n\n    pub fn for_each_edge<F>(&mut self, u: usize, v: usize, mut\
    \ f: F)\n    where\n        F: FnMut(usize, usize),\n    {\n        let l = self.lca(u,\
    \ v);\n        for (l, r) in self.ascend(u, l) {\n            f(r.min(l + 1),\
    \ r.max(l + 1));\n        }\n        for (l, r) in self.descend(l, v) {\n    \
    \        f(l.min(r + 1), l.max(r + 1));\n        }\n    }\n\n    pub fn index(&self,\
    \ v: usize) -> (usize, usize) {\n        assert!(v < self.n);\n        (self.preorder[v],\
    \ self.postorder[v])\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: tree/heavy-light-decomposition/src/lib.rs
  requiredBy: []
  timestamp: '2025-06-21 17:54:09+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/grl_5_c/src/main.rs
  - verification/aizu-online-judge/grl_5_d/src/main.rs
  - verification/aizu-online-judge/grl_5_e/src/main.rs
documentation_of: tree/heavy-light-decomposition/src/lib.rs
layout: document
title: Heavy-Light Decomposition
---

## Description
