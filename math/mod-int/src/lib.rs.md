---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: math/mod-combinatorial/src/lib.rs
    title: Mod Combinatorial
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
    \ -> u64 {\n        self.value % MOD\n    }\n\n    pub fn pow(&self, mut n: u64)\
    \ -> Self {\n        let mut value = self.value;\n        let mut res = 1;\n \
    \       while n > 0 {\n            if n & 1 != 0 {\n                res = (res\
    \ * value) % MOD;\n            }\n            value = (value * value) % MOD;\n\
    \            n >>= 1;\n        }\n        Self { value: res }\n    }\n\n    pub\
    \ fn inv(&self) -> Self {\n        self.pow(MOD - 2)\n    }\n}\n\nimpl<const MOD:\
    \ u64> std::ops::Add for ModInt<MOD> {\n    type Output = Self;\n    fn add(self,\
    \ rhs: Self) -> Self {\n        Self {\n            value: (self.value + rhs.value)\
    \ % MOD,\n        }\n    }\n}\n\nimpl<const MOD: u64> std::ops::AddAssign for\
    \ ModInt<MOD> {\n    fn add_assign(&mut self, rhs: Self) {\n        *self = *self\
    \ + rhs;\n    }\n}\n\nimpl<const MOD: u64> std::ops::Sub for ModInt<MOD> {\n \
    \   type Output = Self;\n    fn sub(mut self, rhs: Self) -> Self {\n        if\
    \ self.value < rhs.value {\n            self.value += MOD;\n        }\n      \
    \  Self {\n            value: (self.value - rhs.value) % MOD,\n        }\n   \
    \ }\n}\n\nimpl<const MOD: u64> std::ops::SubAssign for ModInt<MOD> {\n    fn sub_assign(&mut\
    \ self, rhs: Self) {\n        *self = *self - rhs;\n    }\n}\n\nimpl<const MOD:\
    \ u64> std::ops::Mul for ModInt<MOD> {\n    type Output = Self;\n    fn mul(self,\
    \ rhs: Self) -> Self {\n        Self {\n            value: (self.value * rhs.value)\
    \ % MOD,\n        }\n    }\n}\n\nimpl<const MOD: u64> std::ops::MulAssign for\
    \ ModInt<MOD> {\n    fn mul_assign(&mut self, rhs: Self) {\n        *self = *self\
    \ * rhs;\n    }\n}\n\nimpl<const MOD: u64> std::ops::Div for ModInt<MOD> {\n \
    \   type Output = Self;\n    fn div(self, rhs: Self) -> Self {\n        if rhs.value\
    \ == 0 {\n            panic!();\n        }\n        self * rhs.inv()\n    }\n\
    }\n\nimpl<const MOD: u64> std::ops::DivAssign for ModInt<MOD> {\n    fn div_assign(&mut\
    \ self, rhs: Self) {\n        *self = *self / rhs;\n    }\n}\n\nimpl<const MOD:\
    \ u64> std::fmt::Display for ModInt<MOD> {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)\
    \ -> std::fmt::Result {\n        write!(f, \"{}\", self.value)\n    }\n}\n\n/*\n\
    macro_rules! impl_from {\n    ($($type:ty), *) => {\n        $(\n            impl<const\
    \ MOD: u64> From<$type> for ModInt<MOD> {\n                fn from(value: $type)\
    \ -> Self {\n                    Self::new(value as u64)\n                }\n\
    \            }\n        )*\n    };\n}\n\nimpl_from!(i8, u8, i16, u16, i32, u32,\
    \ u64, i64, isize, usize);\n*/\n\n/*\nmacro_rules! impl_ops {\n    ($trait:ident,\
    \ $fn:ident, $op:tt) => {\n        impl<const MOD: u64> std::ops::$trait for ModInt<MOD>\
    \ {\n            fn $fn(&mut self, rhs: Self) {\n                *self = *self\
    \ $op rhs;\n            }\n        }\n    };\n}\n\nimpl_ops!(AddAssign, add_assign,\
    \ +);\nimpl_ops!(SubAssign, sub_assign, -);\nimpl_ops!(MulAssign, mul_assign,\
    \ *);\nimpl_ops!(DivAssign, div_assign, /);\n*/\n"
  dependsOn: []
  isVerificationFile: false
  path: math/mod-int/src/lib.rs
  requiredBy:
  - math/mod-combinatorial/src/lib.rs
  timestamp: '2025-04-04 10:54:03+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/library-checker/range_affine_range_sum/src/main.rs
documentation_of: math/mod-int/src/lib.rs
layout: document
title: Mod Int
---

## Description
