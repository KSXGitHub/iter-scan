use crate::{ScanClone, ScanCopy, ScanStateClone, ScanStateCopy, ScanWithTuple};

/// Iterator scan methods that don't suck.
pub trait IterScan: Iterator + Sized {
    #[doc = include_str!("docs/scan-clone.md")]
    fn scan_clone<Compute, State>(
        self,
        initial: State,
        compute: Compute,
    ) -> ScanClone<Self, Compute, State>
    where
        State: Clone,
        Compute: FnMut(State, Self::Item) -> State,
    {
        ScanClone::new(self, initial, compute)
    }

    #[doc = include_str!("docs/scan-copy.md")]
    fn scan_copy<Compute, State>(
        self,
        initial: State,
        compute: Compute,
    ) -> ScanCopy<Self, Compute, State>
    where
        State: Copy,
        Compute: FnMut(State, Self::Item) -> State,
    {
        ScanCopy::new(self, initial, compute)
    }

    #[doc = include_str!("docs/scan-state-clone.md")]
    fn scan_state_clone<Compute, State, Value>(
        self,
        initial: State,
        compute: Compute,
    ) -> ScanStateClone<Self, Compute, State, Value>
    where
        State: Clone,
        Compute: FnMut(State, Self::Item) -> (State, Value),
    {
        ScanStateClone::new(self, initial, compute)
    }

    #[doc = include_str!("docs/scan-state-copy.md")]
    fn scan_state_copy<Compute, State, Value>(
        self,
        initial: State,
        compute: Compute,
    ) -> ScanStateCopy<Self, Compute, State, Value>
    where
        State: Copy,
        Compute: FnMut(State, Self::Item) -> (State, Value),
    {
        ScanStateCopy::new(self, initial, compute)
    }

    #[doc = include_str!("docs/scan-with-tuple.md")]
    fn scan_with_tuple<Compute, State, Value>(
        self,
        initial: State,
        compute: Compute,
    ) -> ScanWithTuple<Self, Compute, State, Value>
    where
        Compute: FnMut(State, Self::Item) -> (State, Value),
    {
        ScanWithTuple::new(self, initial, compute)
    }
}

impl<X: Iterator + Sized> IterScan for X {}
