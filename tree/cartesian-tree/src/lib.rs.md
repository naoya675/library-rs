---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':x:'
    path: verification/library-checker/cartesian_tree/src/main.rs
    title: verification/library-checker/cartesian_tree/src/main.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "#[derive(Debug, Clone)]\npub struct CartesianTree<T> {\n    a: Vec<T>,\n\
    \    n: usize,\n    root: usize,\n    parent: Vec<usize>,\n    lchild: Vec<usize>,\n\
    \    rchild: Vec<usize>,\n}\n\nimpl<T: Copy + Ord + PartialOrd + PartialEq> CartesianTree<T>\
    \ {\n    pub fn new(a: Vec<T>) -> Self {\n        let n = a.len();\n        Self\
    \ {\n            a,\n            n,\n            root: n,\n            parent:\
    \ vec![n; n],\n            lchild: vec![n; n],\n            rchild: vec![n; n],\n\
    \        }\n    }\n\n    pub fn run(&mut self, min: bool) -> Vec<usize> {\n  \
    \      let mut stack = vec![];\n        for i in 0..self.n {\n            let\
    \ mut p = self.n;\n            while !stack.is_empty() && !((self.a[i] < self.a[*stack.last().unwrap()])\
    \ ^ min) {\n                if let Some(j) = stack.pop() {\n                 \
    \   self.rchild[j] = p;\n                    p = j;\n                }\n     \
    \       }\n            if p != self.n {\n                self.parent[p] = i;\n\
    \            }\n            if !stack.is_empty() {\n                self.parent[i]\
    \ = *stack.last().unwrap();\n            }\n            self.lchild[i] = p;\n\
    \            stack.push(i);\n        }\n        for i in 0..stack.len() - 1 {\n\
    \            self.rchild[stack[i]] = stack[i + 1];\n        }\n        self.root\
    \ = stack[0];\n        self.parent.clone()\n    }\n}\n\npub fn cartesian_tree<T:\
    \ Copy + Ord + PartialOrd + PartialEq>(a: Vec<T>) -> Vec<usize> {\n    let n =\
    \ a.len();\n    let mut parent = vec![n; n];\n    let mut stack = vec![];\n  \
    \  for i in 0..n {\n        let mut p = n;\n        while !stack.is_empty() &&\
    \ a[i] < a[*stack.last().unwrap()] {\n            p = stack.pop().unwrap();\n\
    \        }\n        if p != n {\n            parent[p] = i;\n        }\n     \
    \   if !stack.is_empty() {\n            parent[i] = *stack.last().unwrap();\n\
    \        }\n        stack.push(i);\n    }\n    parent\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: tree/cartesian-tree/src/lib.rs
  requiredBy: []
  timestamp: '2025-08-21 20:48:10+09:00'
  verificationStatus: LIBRARY_ALL_WA
  verifiedWith:
  - verification/library-checker/cartesian_tree/src/main.rs
documentation_of: tree/cartesian-tree/src/lib.rs
layout: document
title: Cartesian Tree
---

## Description

## Reference
- [https://drken1215.hatenablog.com/entry/2023/10/19/025800](https://drken1215.hatenablog.com/entry/2023/10/19/025800)
