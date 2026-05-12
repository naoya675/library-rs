---
title: XorShift64
documentation_of: //math/xorshift-64/src/lib.rs
---

64-bit state xorshift pseudo-random number generator (Marsaglia). Initial state is derived from the seed via splitmix64.

## new

```rust
fn new(seed: u64) -> Self
```

Creates a new generator with the given seed.

**Complexity**
- $O(1)$

## next_u64

```rust
fn next_u64(&mut self) -> u64
```

Returns the next raw $u64$ value.

**Complexity**
- $O(1)$

## random_range

```rust
fn random_range(&mut self, range: Range<u64>) -> u64
```

Returns a value in $[l, r)$ where `range = l..r`.

**Constraints**
- $l < r$

**Complexity**
- $O(1)$

**Note**

Uses `next_u64() % len`, which has modulo bias up to $\text{len} / 2^{64}$ when $\text{len}$ does not divide $2^{64}$. For unbiased sampling, see [Lemire's method](https://arxiv.org/abs/1805.10941).

## random_f64

```rust
fn random_f64(&mut self) -> f64
```

Returns a value in $[0, 1)$.

**Complexity**
- $O(1)$

## shuffle

```rust
fn shuffle<T>(&mut self, a: &mut [T])
```

Shuffles $a$ in place using Fisher-Yates.

**Complexity**
- $O(n)$

## Reference
- [https://www.jstatsoft.org/article/view/v008i14](https://www.jstatsoft.org/article/view/v008i14)
- [https://blog.visvirial.com/articles/575](https://blog.visvirial.com/articles/575)
