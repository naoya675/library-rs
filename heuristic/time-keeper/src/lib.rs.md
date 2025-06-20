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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "#[derive(Debug, Clone)]\nstruct TimeKeeper {\n    time: std::time::Instant,\n\
    \    time_threshold: f64,\n}\n\nimpl TimeKeeper {\n    fn new(time_threshold:\
    \ f64) -> Self {\n        TimeKeeper {\n            time: std::time::Instant::now(),\n\
    \            time_threshold,\n        }\n    }\n\n    fn is_time_over(&self) ->\
    \ bool {\n        let elapsed_time = self.time.elapsed().as_nanos() as f64 * 1e-9;\n\
    \        #[cfg(feature = \"local\")]\n        {\n            elapsed_time * 0.90\
    \ >= self.time_shreshold\n        }\n        #[cfg(not(feature = \"local\"))]\n\
    \        {\n            elapsed_time >= self.time_threshold\n        }\n    }\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: heuristic/time-keeper/src/lib.rs
  requiredBy: []
  timestamp: '2024-03-28 18:17:25+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: heuristic/time-keeper/src/lib.rs
layout: document
redirect_from:
- /library/heuristic/time-keeper/src/lib.rs
- /library/heuristic/time-keeper/src/lib.rs.html
title: heuristic/time-keeper/src/lib.rs
---
