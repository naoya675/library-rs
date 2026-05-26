---
title: Treap
documentation_of: //data-structure/treap/src/lib.rs
---

A randomized self-balancing binary search tree that maintains the BST property by key and a max-heap property by a random priority assigned to each node.
Operates as an ordered set over a key type `T: Ord`, supporting predecessor/successor, $k$-th element, split/merge, and set operations in $O(\log n)$ expected time.
The `insert` method rejects duplicates (set semantics), but the internal `insert_inner` allows duplicates; removing the `contains` guard in `insert` turns this into an ordered multiset.

## new

```rust
fn new() -> Self
```

Creates an empty Treap.

**Complexity**
- $O(1)$

## len

```rust
fn len(&self) -> usize
```

Returns the number of elements in the set.

**Complexity**
- $O(1)$

## is_empty

```rust
fn is_empty(&self) -> bool
```

Returns `true` if the set is empty.

**Complexity**
- $O(1)$

## insert

```rust
fn insert(&mut self, x: T) -> bool
```

Inserts $x$ into the set.
Returns `true` if $x$ was not already present, `false` if it was already present (no duplicate is added).

**Complexity**
- $O(\log n)$ expected

## remove

```rust
fn remove(&mut self, x: &T) -> bool
```

Removes $x$ from the set.
Returns `true` if $x$ was present, `false` otherwise.

**Complexity**
- $O(\log n)$ expected

## contains

```rust
fn contains(&self, x: &T) -> bool
```

Returns `true` if $x$ is in the set.

**Complexity**
- $O(\log n)$ expected

## kth

```rust
fn kth(&self, k: usize) -> &T
```

Returns a reference to the $k$-th smallest element (0-indexed).

**Constraints**
- $0 \leq k < n$

**Complexity**
- $O(\log n)$ expected

## min

```rust
fn min(&self) -> Option<&T>
```

Returns a reference to the smallest element, or `None` if the set is empty.

**Complexity**
- $O(\log n)$ expected

## max

```rust
fn max(&self) -> Option<&T>
```

Returns a reference to the largest element, or `None` if the set is empty.

**Complexity**
- $O(\log n)$ expected

## pop_min

```rust
fn pop_min(&mut self) -> Option<T>
```

Removes and returns the smallest element, or `None` if the set is empty.

**Complexity**
- $O(\log n)$ expected

## pop_max

```rust
fn pop_max(&mut self) -> Option<T>
```

Removes and returns the largest element, or `None` if the set is empty.

**Complexity**
- $O(\log n)$ expected

## clear

```rust
fn clear(&mut self)
```

Removes all elements from the set.

**Complexity**
- $O(n)$

## lower_bound

```rust
fn lower_bound(&self, x: &T) -> usize
```

Returns the number of elements strictly less than $x$.

**Complexity**
- $O(\log n)$ expected

## upper_bound

```rust
fn upper_bound(&self, x: &T) -> usize
```

Returns the number of elements less than or equal to $x$.

**Complexity**
- $O(\log n)$ expected

## successor

```rust
fn successor(&self, x: &T) -> Option<&T>
```

Returns a reference to the smallest element greater than or equal to $x$, or `None` if no such element exists.

**Complexity**
- $O(\log n)$ expected

## predecessor

```rust
fn predecessor(&self, x: &T) -> Option<&T>
```

Returns a reference to the largest element less than or equal to $x$, or `None` if no such element exists.

**Complexity**
- $O(\log n)$ expected

## split_off

```rust
fn split_off(&mut self, x: &T) -> Self
```

Splits the set by value.
After the call, `self` contains the elements less than $x$ and the returned Treap contains the elements greater than or equal to $x$.

**Complexity**
- $O(\log n)$ expected

## split_off_at

```rust
fn split_off_at(&mut self, k: usize) -> Self
```

Splits the set by index.
After the call, `self` contains the smallest $k$ elements and the returned Treap contains the rest.

**Complexity**
- $O(\log n)$ expected

## merge

```rust
fn merge(&mut self, rhs: Self)
```

Merges `rhs` into `self`.
Requires that every element of `self` is strictly less than every element of `rhs`.

**Constraints**
- `self.max() < rhs.min()`

**Complexity**
- $O(\log (n + m))$ expected, where $n$ and $m$ are the sizes of `self` and `rhs`.

## union

```rust
fn union(&mut self, rhs: Self)
```

Merges `rhs` into `self`, removing duplicate keys.
No ordering constraint on inputs.

**Complexity**
- $O((n + m) \log (n + m))$ expected

## iter

```rust
fn iter(&self) -> Iter<'_, T>
```

Returns an iterator that visits the elements in ascending order.

**Complexity**
- $O(n)$ for full traversal

## Reference
- Guy E. Blelloch and Margaret Reid-Miller, ["Fast Set Operations Using Treaps"](https://www.cs.cmu.edu/afs/cs.cmu.edu/project/scandal/public/papers/treaps-spaa98.pdf), SPAA 1998.
    - [https://www.cs.cmu.edu/afs/cs.cmu.edu/project/scandal/public/papers/treaps-spaa98.html](https://www.cs.cmu.edu/afs/cs.cmu.edu/project/scandal/public/papers/treaps-spaa98.html)
    - [https://www.cs.cmu.edu/~scandal/treaps.html](https://www.cs.cmu.edu/~scandal/treaps.html)
- R. Seidel and C. R. Aragon, ["Randomized search trees"](https://link.springer.com/article/10.1007/BF01940876), Algorithmica 16 (1996), 464–497.
- [https://cp-algorithms.com/data_structures/treap.html](https://cp-algorithms.com/data_structures/treap.html)
- [https://www.cs.umd.edu/class/fall2020/cmsc420-0201/Lects/lect08-treap.pdf](https://www.cs.umd.edu/class/fall2020/cmsc420-0201/Lects/lect08-treap.pdf)
- [https://www.slideshare.net/slideshow/2-12188757/12188757](https://www.slideshare.net/slideshow/2-12188757/12188757)
- [http://www.prefield.com/algorithm/container/treap.html](https://web.archive.org/web/20200221155641/http://www.prefield.com/algorithm/container/treap.html)
- [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=2268](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=2268)
- Implicit Treap
    - [https://qiita.com/hamamu/items/570e7c36bf73913cbeb8](https://qiita.com/hamamu/items/570e7c36bf73913cbeb8)
    - [https://xuzijian629.hatenablog.com/entry/2018/12/08/000452](https://xuzijian629.hatenablog.com/entry/2018/12/08/000452)
    - [https://xuzijian629.hatenablog.com/entry/2019/10/25/234938](https://xuzijian629.hatenablog.com/entry/2019/10/25/234938)
    <!--- [https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=3108610](https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=3108610)-->
