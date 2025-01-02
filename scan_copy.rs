use crate::{Iter, PseudoFunc};
use core::marker::PhantomData;

/// Pseudo-function to duplicate values by [copying](Copy).
#[derive(Debug, Clone, Copy)]
struct DuplicateByCopying;

impl<X: Copy> PseudoFunc<X, (X, X)> for DuplicateByCopying {
    fn exec(x: X) -> (X, X) {
        (x, x)
    }
}

/// An iterator created by [`scan_copy`](crate::IterScan::scan_copy).
#[derive(Debug, Clone, Copy)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ScanCopy<Source, Compute, Value> {
    internal: Iter<Source, DuplicateByCopying, Compute, Value, Value>,
}

impl<Source, Compute, Value> ScanCopy<Source, Compute, Value> {
    pub(crate) fn new(source: Source, initial: Value, compute: Compute) -> Self {
        let internal = Iter {
            source,
            compute,
            state: initial,
            _phantom: PhantomData,
        };
        ScanCopy { internal }
    }
}

impl<Source, Compute, Value> Iterator for ScanCopy<Source, Compute, Value>
where
    Source: Iterator,
    Compute: FnMut(Value, Source::Item) -> Value,
    Value: Copy,
{
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.internal.size_hint()
    }
}
