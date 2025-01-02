use crate::{Iter, PseudoFunc};
use core::marker::PhantomData;

/// Pseudo-function to duplicate values by [cloning](Clone).
#[derive(Debug, Clone, Copy)]
struct DuplicateByCloning;

impl<X: Clone> PseudoFunc<X, (X, X)> for DuplicateByCloning {
    fn exec(x: X) -> (X, X) {
        (x.clone(), x)
    }
}

/// An iterator created by [`scan_clone`](crate::IterScan::scan_clone).
#[derive(Debug, Clone, Copy)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ScanClone<Source, Compute, Value> {
    internal: Iter<Source, DuplicateByCloning, Compute, Value, Value>,
}

impl<Source, Compute, Value> ScanClone<Source, Compute, Value> {
    pub(crate) fn new(source: Source, initial: Value, compute: Compute) -> Self {
        let internal = Iter {
            source,
            compute,
            state: initial,
            _phantom: PhantomData,
        };
        ScanClone { internal }
    }
}

impl<Source, Compute, Value> Iterator for ScanClone<Source, Compute, Value>
where
    Source: Iterator,
    Compute: FnMut(Value, Source::Item) -> Value,
    Value: Clone,
{
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.internal.size_hint()
    }
}
