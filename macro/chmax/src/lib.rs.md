---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://qiita.com/maguro_tuna/items/fab200fdc1efde1612e7
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! https://qiita.com/maguro_tuna/items/fab200fdc1efde1612e7\n\nmacro_rules!\
    \ chmax {\n    ($base:expr, $($ex:expr),+ $(,)*) => {\n        let max = max!($($ex),+);\n\
    \        if $base < max {\n            $base = max;\n            true\n      \
    \  } else {\n            false\n        }\n    };\n}\n\nmacro_rules! max {\n \
    \   ($exa:expr $(,)*) => {\n        $exa\n    };\n    ($exa:expr, $exb:expr $(,)*)\
    \ => {\n        std::cmp::max($exa, $exb)\n    };\n    ($exa:expr, $($ex:expr),+\
    \ $(,)*) => {\n        std::cmp::max($exa, max!($($ex),+))\n    };\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: macro/chmax/src/lib.rs
  requiredBy: []
  timestamp: '2025-05-26 15:54:30+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: macro/chmax/src/lib.rs
layout: document
title: chmax
---

## Description
