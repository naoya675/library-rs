// Reference: https://docs.rs/superslice/latest/superslice/trait.Ext.html

use std::cmp::Ordering::{self, Less};

pub trait LowerBound {
    type Item;

    fn lower_bound(&self, x: &Self::Item) -> usize
    where
        Self::Item: Ord;

    fn lower_bound_by<'a, F>(&'a self, f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> Ordering;
}

impl<T> LowerBound for [T] {
    type Item = T;

    fn lower_bound(&self, x: &Self::Item) -> usize
    where
        T: Ord,
    {
        self.lower_bound_by(|y| y.cmp(x))
    }

    fn lower_bound_by<'a, F>(&'a self, mut f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> Ordering,
    {
        let s = self;
        let mut size = s.len();
        if size == 0 {
            return 0;
        }
        let mut base = 0;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            let cmp = f(unsafe { s.get_unchecked(mid) });
            base = if cmp == Less { mid } else { base };
            size -= half;
        }
        let cmp = f(unsafe { s.get_unchecked(base) });
        base + (cmp == Less) as usize
    }
}

pub fn lower_bound<T: Ord>(a: &[T], x: &T) -> usize {
    lower_bound_by(a, |y| y.cmp(x))
}

pub fn lower_bound_by<T, F>(a: &[T], mut f: F) -> usize
where
    F: FnMut(&T) -> Ordering,
{
    let s = a;
    let mut size = s.len();
    if size == 0 {
        return 0;
    }
    let mut base = 0;
    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        let cmp = f(unsafe { s.get_unchecked(mid) });
        base = if cmp == Less { mid } else { base };
        size -= half;
    }
    let cmp = f(unsafe { s.get_unchecked(base) });
    base + (cmp == Less) as usize
}
