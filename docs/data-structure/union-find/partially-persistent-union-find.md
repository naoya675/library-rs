---
title: Partially Persistent Union-Find (部分永続 Union-Find)
documentation_of: //data-structure/union-find/partially-persistent-union-find/src/lib.rs
---

A data structure for managing disjoint sets with access to past states. Each `merge` advances an internal time counter, and queries can target any past time.

## new

```rust
fn new(n: usize) -> Self
```

Creates $n$ sets at time $0$. Set $i$ ($0 \leq i < n$) initially contains only element $i$.

**Constraints**
- $0 \leq n$

**Complexity**
- $O(n)$

## merge

```rust
fn merge(&mut self, x: usize, y: usize) -> usize
```

Advances the internal time by $1$ and merges the set that contains $x$ and the set that contains $y$. Returns the representative of the merged set.

**Constraints**
- $0 \leq x, y < n$

**Complexity**
- $O(\log n)$

## same

```rust
fn same(&self, t: usize, x: usize, y: usize) -> bool
```

Returns whether $x$ and $y$ belong to the same set at time $t$.

**Constraints**
- $0 \leq x, y < n$
- $0 \leq t \leq$ `self.now()`

**Complexity**
- $O(\log n)$

## leader

```rust
fn leader(&self, t: usize, x: usize) -> usize
```

Returns the representative of the set that contains $x$ at time $t$.

**Constraints**
- $0 \leq x < n$
- $0 \leq t \leq$ `self.now()`

**Complexity**
- $O(\log n)$

## size

```rust
fn size(&self, t: usize, x: usize) -> usize
```

Returns the number of elements in the set that contains $x$ at time $t$.

**Constraints**
- $0 \leq x < n$
- $0 \leq t \leq$ `self.now()`

**Complexity**
- $O(\log n)$

## now

```rust
fn now(&self) -> usize
```

Returns the current time, equal to the number of `merge` calls performed so far.

**Complexity**
- $O(1)$

## Reference
- [https://blog.tiramister.net/posts/persistent-unionfind/](https://blog.tiramister.net/posts/persistent-unionfind/)
- [https://camypaper.bitbucket.io/2016/12/18/adc2016/](https://camypaper.bitbucket.io/2016/12/18/adc2016/)
- [https://ikatakos.com/pot/programming_algorithm/data_structure/union_find_tree](https://ikatakos.com/pot/programming_algorithm/data_structure/union_find_tree)
- [https://noshi91.hatenablog.com/entry/2018/02/18/161529](https://noshi91.hatenablog.com/entry/2018/02/18/161529)
- [https://noshi91.hatenablog.com/entry/2018/05/30/191943](https://noshi91.hatenablog.com/entry/2018/05/30/191943)
- [https://qiita.com/alumite14/items/1fd477a14cf5c3019326](https://qiita.com/alumite14/items/1fd477a14cf5c3019326)
- [https://qiita.com/alumite14/items/f4c355720f2a6da88ca5](https://qiita.com/alumite14/items/f4c355720f2a6da88ca5)
- [https://speakerdeck.com/camypaper/persistent-dsu](https://speakerdeck.com/camypaper/persistent-dsu)

## Verified
- [https://atcoder.jp/contests/code-thanks-festival-2017/tasks/code_thanks_festival_2017_h](https://atcoder.jp/contests/code-thanks-festival-2017/tasks/code_thanks_festival_2017_h) ([submission](https://atcoder.jp/contests/code-thanks-festival-2017/submissions/75494801))
- [https://atcoder.jp/contests/agc002/tasks/agc002_d](https://atcoder.jp/contests/agc002/tasks/agc002_d) ([submission](https://atcoder.jp/contests/agc002/submissions/75494734) / [editorial](https://www.mathenachia.blog/agc002d-usereditorial/))
