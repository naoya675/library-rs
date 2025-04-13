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
  code: "pub trait Zero {\n    fn zero() -> Self;\n    fn is_zero(&self) -> bool;\n\
    }\n\npub trait One {\n    fn one() -> Self;\n    fn is_one(&self) -> bool;\n}\n\
    \npub trait BoundedBelow {\n    fn min_value() -> Self;\n}\n\npub trait BoundedAbove\
    \ {\n    fn max_value() -> Self;\n}\n\nmacro_rules! impl_zero {\n    ($($type:ty),\
    \ *) => {\n        $(\n            impl Zero for $type {\n                fn zero()\
    \ -> Self {\n                    0\n                }\n                fn is_zero(&self)\
    \ -> bool {\n                    *self == 0\n                }\n            }\n\
    \        )*\n    };\n}\n\nmacro_rules! impl_one {\n    ($($type:ty), *) => {\n\
    \        $(\n            impl One for $type {\n                fn one() -> Self\
    \ {\n                    1\n                }\n                fn is_one(&self)\
    \ -> bool {\n                    *self == 1\n                }\n            }\n\
    \        )*\n    };\n}\n\nmacro_rules! impl_bounded_below {\n    ($($type:ty),\
    \ *) => {\n        $(\n            impl BoundedBelow for $type {\n           \
    \     fn min_value() -> Self {\n                    Self::min_value()\n      \
    \          }\n            }\n        )*\n    };\n}\n\nmacro_rules! impl_bounded_above\
    \ {\n    ($($type:ty), *) => {\n        $(\n            impl BoundedAbove for\
    \ $type {\n                fn max_value() -> Self {\n                    Self::max_value()\n\
    \                }\n            }\n        )*\n    };\n}\n\nimpl_zero!(i8, u8,\
    \ i16, u16, i32, u32, u64, i64, isize, usize);\nimpl_one!(i8, u8, i16, u16, i32,\
    \ u32, u64, i64, isize, usize);\nimpl_bounded_below!(i8, u8, i16, u16, i32, u32,\
    \ u64, i64, isize, usize);\nimpl_bounded_above!(i8, u8, i16, u16, i32, u32, u64,\
    \ i64, isize, usize);\n"
  dependsOn: []
  isVerificationFile: false
  path: algebra/internal-trait/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-13 01:08:14+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: algebra/internal-trait/src/lib.rs
layout: document
redirect_from:
- /library/algebra/internal-trait/src/lib.rs
- /library/algebra/internal-trait/src/lib.rs.html
title: algebra/internal-trait/src/lib.rs
---
