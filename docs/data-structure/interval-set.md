---
title: Interval Set
documentation_of: //data-structure/interval-set/src/lib.rs
---

Manages a set of non-overlapping half-open intervals $[l, r)$ with associated values. Adjacent intervals with the same value are automatically merged. Supports update, insert, erase with optional add/del callbacks.

## new

```rust
fn new(identity: VAL) -> Self
```

Creates an empty interval set. Points not covered by any interval are considered to have the value `identity`.

**Complexity**
- $O(1)$

## from_vec

```rust
fn from_vec(v: &[VAL], identity: VAL) -> Self
```

Creates an interval set from a slice. Consecutive equal elements are grouped into a single interval. Requires `T: From<usize>`.

**Complexity**
- $O(n \log n)$

## get

```rust
fn get(&self, p: T) -> Option<(T, T, VAL)>
```

Returns the interval $[l, r)$ containing point $p$ as `Some((l, r, val))`. Returns `None` if $p$ is not covered by any interval.

**Complexity**
- $O(\log n)$

## lower_bound

```rust
fn lower_bound(&self, p: T) -> Option<(T, T, VAL)>
```

Returns the first interval that contains $p$ or starts after $p$. Returns `None` if no such interval exists.

**Complexity**
- $O(\log n)$

## upper_bound

```rust
fn upper_bound(&self, p: T) -> Option<(T, T, VAL)>
```

Returns the first interval that starts after $p$. Returns `None` if no such interval exists.

**Complexity**
- $O(\log n)$

## covered_point

```rust
fn covered_point(&self, p: T) -> bool
```

Returns whether point $p$ is covered by some interval.

**Complexity**
- $O(\log n)$

## covered_range

```rust
fn covered_range(&self, l: T, r: T) -> bool
```

Returns whether the entire range $[l, r)$ is covered by a single interval.

**Constraints**
- $l \leq r$

**Complexity**
- $O(\log n)$

## same

```rust
fn same(&self, p: T, q: T) -> bool
```

Returns whether points $p$ and $q$ belong to the same interval.

**Complexity**
- $O(\log n)$

## get_val

```rust
fn get_val(&self, p: T) -> VAL
```

Returns the value of the interval containing $p$. Returns `identity` if $p$ is not covered.

**Complexity**
- $O(\log n)$

## get_mex

```rust
fn get_mex(&self, p: T) -> T
```

Returns the minimum excludant starting from $p$: the smallest value $\geq p$ that is not covered by any interval.

**Complexity**
- $O(\log n)$

## inner_update

```rust
fn inner_update<F, G>(&mut self, l: T, r: T, val: VAL, add: F, del: G)
```

Updates the range $[l, r)$ to value `val`. Calls `del(l, r, &val)` for each removed interval and `add(l, r, &val)` for each added interval. `add` and `del` must be inverse operations of each other.

**Constraints**
- $l \leq r$

**Complexity**
- $O(k \log n)$ where $k$ is the number of intervals affected

## update

```rust
fn update(&mut self, l: T, r: T, val: VAL)
```

Updates the range $[l, r)$ to value `val` without callbacks.

**Constraints**
- $l \leq r$

**Complexity**
- $O(k \log n)$

## inner_insert

```rust
fn inner_insert<F, G>(&mut self, l: T, r: T, add: F, del: G)
```

Inserts the range $[l, r)$ with the `identity` value, with add/del callbacks.

**Constraints**
- $l \leq r$

**Complexity**
- $O(k \log n)$

## insert

```rust
fn insert(&mut self, l: T, r: T)
```

Inserts the range $[l, r)$ with the `identity` value.

**Constraints**
- $l \leq r$

**Complexity**
- $O(k \log n)$

## inner_erase

```rust
fn inner_erase<F, G>(&mut self, l: T, r: T, add: F, del: G)
```

Erases all intervals in the range $[l, r)$, with add/del callbacks. `add` is called for remaining pieces after splitting, `del` for removed pieces. `add` and `del` must be inverse operations of each other.

**Constraints**
- $l \leq r$

**Complexity**
- $O(k \log n)$

## erase

```rust
fn erase(&mut self, l: T, r: T)
```

Erases all intervals in the range $[l, r)$ without callbacks.

**Constraints**
- $l \leq r$

**Complexity**
- $O(k \log n)$

## iter

```rust
fn iter(&self) -> impl Iterator<Item = (T, T, VAL)>
```

Returns an iterator over all intervals as $(l, r,$ `val`$)$ tuples, sorted by $l$.

**Complexity**
- $O(n)$

## Reference
- [https://rsk0315.hatenablog.com/entry/2020/10/11/125049](https://rsk0315.hatenablog.com/entry/2020/10/11/125049)
- [https://qiita.com/hibit/items/7e27a41212f849179a79](https://qiita.com/hibit/items/7e27a41212f849179a79)
- [https://github.com/drken1215/algorithm/blob/master/DataStructureSegment/intervals_set.cpp](https://github.com/drken1215/algorithm/blob/master/DataStructureSegment/intervals_set.cpp)
<!--- [https://github.com/drken1215/algorithm/blob/master/DataStructureSegment/intervals_set_with_noninvertible_del.cpp](https://github.com/drken1215/algorithm/blob/master/DataStructureSegment/intervals_set_with_noninvertible_del.cpp)-->
<!--- [https://github.com/drken1215/algorithm/blob/master/DataStructure/intervals_management.cpp](https://github.com/drken1215/algorithm/blob/master/DataStructure/intervals_management.cpp)-->

## Verified
<!--- [https://atcoder.jp/contests/abc255/tasks/abc255_h](https://atcoder.jp/contests/abc255/tasks/abc255_h) ([submission]())-->
<!--- [https://atcoder.jp/contests/abc256/tasks/abc256_h](https://atcoder.jp/contests/abc256/tasks/abc256_h) ([submission]())-->
- [https://atcoder.jp/contests/abc330/tasks/abc330_e](https://atcoder.jp/contests/abc330/tasks/abc330_e) ([submission](https://atcoder.jp/contests/abc330/submissions/74535082))
- [https://atcoder.jp/contests/past202012-open/tasks/past202012_n](https://atcoder.jp/contests/past202012-open/tasks/past202012_n) ([submission](https://atcoder.jp/contests/past202012-open/submissions/74535018))
