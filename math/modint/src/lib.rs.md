---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/range_affine_range_sum/src/main.rs
    title: verification/library-checker/range_affine_range_sum/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct ModInt<const MOD:\
    \ u64> {\n    value: u64,\n}\n\nimpl<const MOD: u64> ModInt<MOD> {\n    pub fn\
    \ new(n: u64) -> Self {\n        Self {\n            value: (n % MOD),\n     \
    \       // value: (n.rem_euclid(MOD)),\n        }\n    }\n\n    pub fn value(&self)\
    \ -> u64 {\n        self.value % MOD\n    }\n\n    pub fn power(&self, mut n:\
    \ u64) -> Self {\n        let mut value = self.value;\n        let mut res = 1;\n\
    \        while n > 0 {\n            if n & 1 != 0 {\n                res = (res\
    \ * value) % MOD;\n            }\n            value = (value * value) % MOD;\n\
    \            n >>= 1;\n        }\n        Self { value: res }\n    }\n\n    pub\
    \ fn pow(&self, n: Self) -> Self {\n        self.power(n.value)\n    }\n\n   \
    \ pub fn inv(&self) -> Self {\n        self.power(MOD - 2)\n    }\n}\n\nimpl<const\
    \ MOD: u64> std::ops::Add for ModInt<MOD> {\n    type Output = Self;\n    fn add(self,\
    \ rhs: Self) -> Self {\n        Self {\n            value: (self.value + rhs.value)\
    \ % MOD,\n        }\n    }\n}\n\nimpl<const MOD: u64> std::ops::AddAssign for\
    \ ModInt<MOD> {\n    fn add_assign(&mut self, rhs: Self) {\n        *self = Self\
    \ {\n            value: (self.value + rhs.value) % MOD,\n        };\n    }\n}\n\
    \nimpl<const MOD: u64> std::ops::Sub for ModInt<MOD> {\n    type Output = Self;\n\
    \    fn sub(mut self, rhs: Self) -> Self {\n        if self.value < rhs.value\
    \ {\n            self.value += MOD;\n        }\n        Self {\n            value:\
    \ (self.value - rhs.value) % MOD,\n        }\n    }\n}\n\nimpl<const MOD: u64>\
    \ std::ops::SubAssign for ModInt<MOD> {\n    fn sub_assign(&mut self, rhs: Self)\
    \ {\n        if self.value < rhs.value {\n            self.value += MOD;\n   \
    \     }\n        *self = Self {\n            value: (self.value - rhs.value) %\
    \ MOD,\n        };\n    }\n}\n\nimpl<const MOD: u64> std::ops::Mul for ModInt<MOD>\
    \ {\n    type Output = Self;\n    fn mul(self, rhs: Self) -> Self {\n        Self\
    \ {\n            value: (self.value * rhs.value) % MOD,\n        }\n    }\n}\n\
    \nimpl<const MOD: u64> std::ops::MulAssign for ModInt<MOD> {\n    fn mul_assign(&mut\
    \ self, rhs: Self) {\n        *self = Self {\n            value: (self.value *\
    \ rhs.value) % MOD,\n        };\n    }\n}\n\nimpl<const MOD: u64> std::ops::Div\
    \ for ModInt<MOD> {\n    type Output = Self;\n    fn div(self, rhs: Self) -> Self\
    \ {\n        if rhs.value == 0 {\n            panic!();\n        }\n        self\
    \ * rhs.inv()\n    }\n}\n\nimpl<const MOD: u64> std::ops::DivAssign for ModInt<MOD>\
    \ {\n    fn div_assign(&mut self, rhs: Self) {\n        if rhs.value == 0 {\n\
    \            panic!();\n        }\n        *self *= rhs.inv();\n    }\n}\n\nimpl<const\
    \ MOD: u64> std::fmt::Display for ModInt<MOD> {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)\
    \ -> std::fmt::Result {\n        write!(f, \"{}\", self.value)\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: math/modint/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-02 22:22:32+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/library-checker/range_affine_range_sum/src/main.rs
documentation_of: math/modint/src/lib.rs
layout: document
title: ModInt
---

## Description
