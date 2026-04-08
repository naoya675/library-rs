---
title: Binomial
documentation_of: //math/binomial/src/lib.rs
---

Provides factorial-based combinatorial functions: binomial coefficient, permutation, multinomial coefficient, and Catalan number. Internal factorial tables are extended lazily on demand, so the maximum input size does not need to be known at construction time.

## new

```rust
fn new() -> Self
```

Creates an empty instance with no precomputed values. Internal tables are extended on demand when accessor methods are called.

**Complexity**
- $O(1)$

## with_capacity

```rust
fn with_capacity(n: usize) -> Self
```

Creates an instance with internal tables precomputed up to index $n$. Subsequent queries whose arguments do not exceed $n$ run in $O(1)$ without triggering further extension.

**Constraints**
- $0 \leq n$

**Complexity**
- $O(n)$

## fact

```rust
fn fact(&mut self, n: usize) -> T
```

Returns $n!$.

**Constraints**
- $0 \leq n$

**Complexity**
- amortized $O(1)$

## finv

```rust
fn finv(&mut self, n: usize) -> T
```

Returns $(n!)^{-1}$, the multiplicative inverse of $n!$.

**Constraints**
- $0 \leq n$

**Complexity**
- amortized $O(1)$

## inv

```rust
fn inv(&mut self, n: usize) -> T
```

Returns $n^{-1}$, the multiplicative inverse of $n$. Panics when $n = 0$.

**Constraints**
- $1 \leq n$

**Complexity**
- amortized $O(1)$

## perm

```rust
fn perm(&mut self, n: usize, r: usize) -> T
```

Returns ${}_nP_r = \dfrac{n!}{(n - r)!}$, the number of ways to arrange $r$ distinct elements chosen from $n$ elements.

**Constraints**
- $0 \leq n, r$

**Complexity**
- amortized $O(1)$

## comb

```rust
fn comb(&mut self, n: usize, r: usize) -> T
```

Returns the binomial coefficient $\dbinom{n}{r}$, the number of ways to choose $r$ elements from a set of $n$ elements.

**Constraints**
- $0 \leq n, r$

**Complexity**
- amortized $O(1)$

## multinomial

```rust
fn multinomial(&mut self, k: &[usize]) -> T
```

Returns the multinomial coefficient $\dbinom{n}{k_1, k_2, \ldots, k_m} = \dfrac{n!}{k_1! \, k_2! \cdots k_m!}$, the number of ways to partition $n$ distinct elements into groups of sizes $k_1, k_2, \ldots, k_m$.

**Constraints**
- $0 \leq k_i$ for each $i$

**Complexity**
- amortized $O(\lvert k \rvert)$

## homo

```rust
fn homo(&mut self, n: usize, r: usize) -> T
```

Returns ${}_nH_r = \dbinom{n + r - 1}{r}$, the number of ways to place $r$ indistinguishable balls into $n$ distinguishable boxes where each box may contain any number of balls.

**Constraints**
- $0 \leq n, r$

**Complexity**
- amortized $O(1)$

## catalan

```rust
fn catalan(&mut self, n: usize) -> T
```

Returns $C_n = \dfrac{1}{n + 1} \dbinom{2n}{n}$, the $n$-th Catalan number, which counts the number of valid parenthesis expressions with $n$ pairs of parentheses, the number of binary trees with $n$ nodes, and many other combinatorial structures.

**Constraints**
- $0 \leq n$

**Complexity**
- amortized $O(1)$

## Reference
- [https://cses.fi/book/book.pdf](https://cses.fi/book/book.pdf) (Antti Laaksonen, *Competitive Programmer's Handbook*)
