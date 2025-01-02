use crate::{Iter, PseudoFunc};
use core::marker::PhantomData;

/// Pseudo-function to duplicate states by [cloning](Clone).
#[derive(Debug, Clone, Copy)]
struct DuplicateX0ByCloning;

impl<X0: Clone, X1> PseudoFunc<(X0, X1), (X0, (X0, X1))> for DuplicateX0ByCloning {
    fn exec(x: (X0, X1)) -> (X0, (X0, X1)) {
        (x.0.clone(), x)
    }
}

/// An iterator created by [`scan_state_clone`](crate::IterScan::scan_state_clone).
#[derive(Debug, Clone, Copy)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ScanStateClone<Source, Compute, State, Value> {
    internal: Iter<Source, DuplicateX0ByCloning, Compute, State, (State, Value)>,
}

impl<Source, Compute, State, Value> ScanStateClone<Source, Compute, State, Value> {
    pub(crate) fn new(source: Source, initial: State, compute: Compute) -> Self {
        let internal = Iter {
            source,
            compute,
            state: initial,
            _phantom: PhantomData,
        };
        ScanStateClone { internal }
    }
}

impl<Source, Compute, State, Value> Iterator for ScanStateClone<Source, Compute, State, Value>
where
    Source: Iterator,
    Compute: FnMut(State, Source::Item) -> (State, Value),
    State: Clone,
{
    type Item = (State, Value);

    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.internal.size_hint()
    }
}
