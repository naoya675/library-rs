---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/alds1_14_a/src/main.rs
    title: verification/aizu-online-judge/alds1_14_a/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/alds1_14_b/src/main.rs
    title: verification/aizu-online-judge/alds1_14_b/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://qiita.com/keymoon/items/11fac5627672a6d6a9f6
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! https://qiita.com/keymoon/items/11fac5627672a6d6a9f6\n\n#[derive(Debug,\
    \ Clone, Copy, PartialEq, Eq)]\npub struct MersenneModint {\n    value: u64,\n\
    }\n\nimpl MersenneModint {\n    const MOD: u64 = (1u64 << 61) - 1;\n    const\
    \ MASK30: u64 = (1u64 << 30) - 1;\n    const MASK31: u64 = (1u64 << 31) - 1;\n\
    \    const MASK61: u64 = Self::MOD;\n\n    pub fn new(n: u64) -> Self {\n    \
    \    Self {\n            value: (n % Self::MOD),\n            // value: (n.rem_euclid(Self::MOD)),\n\
    \        }\n    }\n\n    pub fn value(&self) -> u64 {\n        self.value\n  \
    \  }\n\n    pub fn pow(&self, mut n: u64) -> Self {\n        let mut value = *self;\n\
    \        let mut res = Self::new(1);\n        while n > 0 {\n            if n\
    \ & 1 != 0 {\n                res = res * value;\n            }\n            value\
    \ = value * value;\n            n >>= 1;\n        }\n        res\n    }\n\n  \
    \  pub fn inv(&self) -> Self {\n        self.pow(Self::MOD - 2)\n    }\n\n   \
    \ pub fn rand() -> Self {\n        use rand::Rng;\n        let mut rng = rand::thread_rng();\n\
    \        Self::new(rng.gen_range(Self::MASK31..Self::MASK61))\n    }\n}\n\nimpl\
    \ std::ops::Add for MersenneModint {\n    type Output = Self;\n    fn add(self,\
    \ rhs: Self) -> Self {\n        Self {\n            value: (self.value + rhs.value)\
    \ % Self::MOD,\n        }\n    }\n}\n\nimpl std::ops::AddAssign for MersenneModint\
    \ {\n    fn add_assign(&mut self, rhs: Self) {\n        *self = *self + rhs;\n\
    \    }\n}\n\nimpl std::ops::Sub for MersenneModint {\n    type Output = Self;\n\
    \    fn sub(mut self, rhs: Self) -> Self {\n        if self.value < rhs.value\
    \ {\n            self.value += Self::MOD;\n        }\n        Self {\n       \
    \     value: (self.value - rhs.value) % Self::MOD,\n        }\n    }\n}\n\nimpl\
    \ std::ops::SubAssign for MersenneModint {\n    fn sub_assign(&mut self, rhs:\
    \ Self) {\n        *self = *self - rhs;\n    }\n}\n\nimpl std::ops::Mul for MersenneModint\
    \ {\n    type Output = Self;\n    fn mul(self, rhs: Self) -> Self {\n        let\
    \ au = self.value >> 31;\n        let ad = self.value & Self::MASK31;\n      \
    \  let bu = rhs.value() >> 31;\n        let bd = rhs.value() & Self::MASK31;\n\
    \        let mid = ad * bu + au * bd;\n        let midu = mid >> 30;\n       \
    \ let midd = mid & Self::MASK30;\n        let su = ((au * bu) << 1) + midu + (midd\
    \ << 31) + ad * bd;\n        Self {\n            value: (su >> 61) + (su & Self::MASK61),\n\
    \        }\n    }\n}\n\nimpl std::ops::MulAssign for MersenneModint {\n    fn\
    \ mul_assign(&mut self, rhs: Self) {\n        *self = *self * rhs;\n    }\n}\n\
    \nimpl std::ops::Div for MersenneModint {\n    type Output = Self;\n    fn div(self,\
    \ rhs: Self) -> Self {\n        if rhs.value == 0 {\n            panic!();\n \
    \       }\n        self * rhs.inv()\n    }\n}\n\nimpl std::ops::DivAssign for\
    \ MersenneModint {\n    fn div_assign(&mut self, rhs: Self) {\n        *self =\
    \ *self / rhs;\n    }\n}\n\nimpl std::ops::Neg for MersenneModint {\n    type\
    \ Output = Self;\n    fn neg(self) -> Self {\n        Self::new(0) - self\n  \
    \  }\n}\n\npub trait Zero {\n    fn zero() -> Self;\n    fn is_zero(&self) ->\
    \ bool;\n}\n\nimpl Zero for MersenneModint {\n    fn zero() -> Self {\n      \
    \  Self::new(0)\n    }\n\n    fn is_zero(&self) -> bool {\n        Self::new(0)\
    \ == *self\n    }\n}\n\npub trait One {\n    fn one() -> Self;\n    fn is_one(&self)\
    \ -> bool;\n}\n\nimpl One for MersenneModint {\n    fn one() -> Self {\n     \
    \   Self::new(1)\n    }\n\n    fn is_one(&self) -> bool {\n        Self::new(1)\
    \ == *self\n    }\n}\n\nimpl Default for MersenneModint {\n    fn default() ->\
    \ Self {\n        Self::new(0)\n    }\n}\n\nimpl std::fmt::Display for MersenneModint\
    \ {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n\
    \        write!(f, \"{}\", self.value)\n    }\n}\n\nimpl From<u64> for MersenneModint\
    \ {\n    fn from(value: u64) -> Self {\n        Self::new(value)\n    }\n}\n\n\
    /*\nmacro_rules! impl_from {\n    ($($type:ty), *) => {\n        $(\n        \
    \    impl From<$type> for MersenneModint {\n                fn from(value: $type)\
    \ -> Self {\n                    Self::new(value as u64)\n                }\n\
    \            }\n        )*\n    };\n}\n\nimpl_from!(i8, u8, i16, u16, i32, u32,\
    \ u64, i64, isize, usize);\n*/\n\n/*\nmacro_rules! impl_ops {\n    ($trait:ident,\
    \ $fn:ident, $op:tt) => {\n        impl std::ops::$trait for MersenneModint {\n\
    \            fn $fn(&mut self, rhs: Self) {\n                *self = *self $op\
    \ rhs;\n            }\n        }\n    };\n}\n\nimpl_ops!(AddAssign, add_assign,\
    \ +);\nimpl_ops!(SubAssign, sub_assign, -);\nimpl_ops!(MulAssign, mul_assign,\
    \ *);\nimpl_ops!(DivAssign, div_assign, /);\n*/\n"
  dependsOn: []
  isVerificationFile: false
  path: math/mersenne-modint/src/lib.rs
  requiredBy: []
  timestamp: '2025-05-28 17:48:17+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/alds1_14_b/src/main.rs
  - verification/aizu-online-judge/alds1_14_a/src/main.rs
documentation_of: math/mersenne-modint/src/lib.rs
layout: document
title: Mersenne Modint
---

## Description
