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
  code: "macro_rules! debug {\n    ($exa:expr $(,)*) => {\n        #[cfg(debug_assertions)]\n\
    \            eprintln!(concat!(stringify!($exa), \" = {:?}\"), &$exa);\n     \
    \   };\n    ($exa:expr, $($ex:expr),* $(,)*) => {\n        #[cfg(debug_assertions)]\n\
    \            eprintln!(concat!(stringify!($exa), \" = {:?}\",\n              \
    \  $(\", \", stringify!($ex), \" = {:?}\" ),*), &$exa, $($ex),*);\n        };\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: macro/debug/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-12 00:31:22+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: macro/debug/src/lib.rs
layout: document
redirect_from:
- /library/macro/debug/src/lib.rs
- /library/macro/debug/src/lib.rs.html
title: macro/debug/src/lib.rs
---
