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
  code: "macro_rules! chmin {\n    ($base:expr, $($ex:expr),+ $(,)*) => {\n      \
    \  let min = min!($($ex),+);\n        if $base > min {\n            $base = min;\n\
    \            true\n        } else {\n            false\n        }\n    };\n}\n\
    \nmacro_rules! min {\n    ($exa:expr $(,)*) => {\n        $exa\n    };\n    ($exa:expr,\
    \ $exb:expr $(,)*) => {\n        std::cmp::min($exa, $exb)\n    };\n    ($exa:expr,\
    \ $($ex:expr),+ $(,)*) => {\n        std::cmp::min($exa, min!($($ex),+))\n   \
    \ };\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: macro/chmin/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-12 00:31:22+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: macro/chmin/src/lib.rs
layout: document
title: chmin
---

## Description

## Reference

[https://qiita.com/maguro_tuna/items/fab200fdc1efde1612e7](https://qiita.com/maguro_tuna/items/fab200fdc1efde1612e7)
