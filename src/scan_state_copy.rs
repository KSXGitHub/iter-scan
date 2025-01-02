use crate::{Iter, PseudoFunc};
use core::marker::PhantomData;

/// Pseudo-function to duplicate states by [copying](Copy).
#[derive(Debug, Clone, Copy)]
struct DuplicateX0ByCopying;

impl<X0: Copy, X1> PseudoFunc<(X0, X1), (X0, (X0, X1))> for DuplicateX0ByCopying {
    fn exec(x: (X0, X1)) -> (X0, (X0, X1)) {
        (x.0, x)
    }
}

/// An iterator created by [`scan_state_copy`](crate::IterScan::scan_state_copy).
#[derive(Debug, Clone, Copy)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ScanStateCopy<Source, Compute, State, Value> {
    internal: Iter<Source, DuplicateX0ByCopying, Compute, State, (State, Value)>,
}

impl<Source, Compute, State, Value> ScanStateCopy<Source, Compute, State, Value> {
    pub(crate) fn new(source: Source, initial: State, compute: Compute) -> Self {
        let internal = Iter {
            source,
            compute,
            state: initial,
            _phantom: PhantomData,
        };
        ScanStateCopy { internal }
    }
}

impl<Source, Compute, State, Value> Iterator for ScanStateCopy<Source, Compute, State, Value>
where
    Source: Iterator,
    Compute: FnMut(State, Source::Item) -> (State, Value),
    State: Copy,
{
    type Item = (State, Value);

    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.internal.size_hint()
    }
}
