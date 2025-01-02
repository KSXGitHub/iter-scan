use crate::{Iter, PseudoFunc};
use core::marker::PhantomData;

/// Pseudo-function that returns the value it received as-is.
#[derive(Debug, Clone, Copy)]
pub struct Identity;

impl<X> PseudoFunc<X, X> for Identity {
    fn exec(x: X) -> X {
        x
    }
}

/// An iterator created by [`scan_with_tuple`](crate::IterScan::scan_with_tuple).
#[derive(Debug, Clone, Copy)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ScanWithTuple<Source, Compute, State, Value> {
    internal: Iter<Source, Identity, Compute, State, Value>,
}

impl<Source, Compute, State, Value> ScanWithTuple<Source, Compute, State, Value> {
    pub(crate) fn new(source: Source, initial: State, compute: Compute) -> Self {
        let internal = Iter {
            source,
            compute,
            state: initial,
            _phantom: PhantomData,
        };
        Self { internal }
    }
}

impl<Source, Compute, State, Value> Iterator for ScanWithTuple<Source, Compute, State, Value>
where
    Source: Iterator,
    Compute: FnMut(State, Source::Item) -> (State, Value),
{
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.internal.size_hint()
    }
}
