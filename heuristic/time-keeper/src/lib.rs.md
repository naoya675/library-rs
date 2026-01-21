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
  code: "#[derive(Debug, Clone)]\npub struct TimeKeeper {\n    time: std::time::Instant,\n\
    \    time_threshold: f64,\n}\n\nimpl TimeKeeper {\n    pub fn new(time_threshold:\
    \ f64) -> Self {\n        TimeKeeper {\n            time: std::time::Instant::now(),\n\
    \            time_threshold,\n        }\n    }\n\n    pub fn elapsed_time(&self)\
    \ -> f64 {\n        let elapsed_time = self.time.elapsed().as_nanos() as f64 *\
    \ 1e-9;\n        elapsed_time\n    }\n\n    pub fn elapsed_ratio(&self) -> f64\
    \ {\n        let elapsed_time = self.time.elapsed().as_nanos() as f64 * 1e-9;\n\
    \        elapsed_time / self.time_threshold\n    }\n\n    pub fn is_time_over(&self)\
    \ -> bool {\n        let elapsed_time = self.time.elapsed().as_nanos() as f64\
    \ * 1e-9;\n        #[cfg(feature = \"local\")]\n        {\n            elapsed_time\
    \ * 0.90 >= self.time_threshold\n        }\n        #[cfg(not(feature = \"local\"\
    ))]\n        {\n            elapsed_time * 0.90 >= self.time_threshold\n     \
    \   }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: heuristic/time-keeper/src/lib.rs
  requiredBy: []
  timestamp: '2026-01-21 18:49:44+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: heuristic/time-keeper/src/lib.rs
layout: document
title: Time Keeper
---

## Description

## Reference
- [https://zenn.dev/tipstar0125/articles/245bceec86e40a](https://zenn.dev/tipstar0125/articles/245bceec86e40a)
