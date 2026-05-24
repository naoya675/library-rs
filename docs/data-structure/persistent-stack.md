---
title: Persistent Stack
documentation_of: //data-structure/persistent-stack/src/lib.rs
---

A fully persistent stack.
Every operation returns a new version of the stack without destroying the previous one, so any past version remains accessible.
The internal representation is a singly linked list whose nodes are shared via `Rc`; all operations run in $O(1)$ time and use $O(1)$ extra memory per operation.

## new

```rust
fn new() -> Self
```

Creates an empty stack.

**Complexity**
- $O(1)$

## is_empty

```rust
fn is_empty(&self) -> bool
```

Returns whether the stack is empty.

**Complexity**
- $O(1)$

## top

```rust
fn top(&self) -> Option<&T>
```

Returns a reference to the top element, or `None` if the stack is empty.

**Complexity**
- $O(1)$

## push

```rust
fn push(&self, x: T) -> Self
```

Returns a new stack obtained by pushing `x` onto the top.
The receiver is not modified.

**Complexity**
- $O(1)$

## pop

```rust
fn pop(&self) -> Option<(T, Self)>
```

Returns the top element together with a new stack with that element removed.
Returns `None` if the stack is empty.
The receiver is not modified.

**Constraints**
- `T: Clone`

**Complexity**
- $O(1)$

## Reference
- Chris Okasaki, ["Purely Functional Data Structures"](https://www.cs.cmu.edu/~rwh/students/okasaki.pdf), CMU PhD Thesis 1996.
- Chris Okasaki, ["Purely Functional Data Structures"](https://theswissbay.ch/pdf/Gentoomen%20Library/Programming/Functional%20Programming/Chris_Okasaki-Purely_Functional_Data_Structures-Cambridge_University_Press%281998%29.pdf), Cambridge University Press 1998.
- [https://37zigen.com/bankers-queue/](https://37zigen.com/bankers-queue/)
- [https://www.mathenachia.blog/persistent-queue/](https://www.mathenachia.blog/persistent-queue/)
- [https://noshi91.hatenablog.com/entry/2019/02/04/175100](https://noshi91.hatenablog.com/entry/2019/02/04/175100)
<!--- [https://github.com/noshi91/Library/blob/master/data_structure/persistent_stack.cpp](https://github.com/noshi91/Library/blob/master/data_structure/persistent_stack.cpp)-->
