#![doc = include_str!("README.md")]
#![no_std]
#![forbid(unsafe_code)]
use core::marker::PhantomData;
use replace_with::replace_with_or_abort_and_return;

type IdFn<X> = fn(X) -> X;
type DupFn<X> = fn(X) -> (X, X);
type StateFn<X0, X1> = fn((X0, X1)) -> (X0, (X0, X1));

/// Iterator scan methods that don't suck.
pub trait IterScan: Iterator + Sized {
    #[doc = include_str!("docs/scan-clone.md")]
    fn scan_clone<Compute, State>(
        self,
        initial: State,
        compute: Compute,
    ) -> Scan<Self, DupFn<State>, Compute, State, State>
    where
        State: Clone,
        Compute: FnMut(State, Self::Item) -> State,
    {
        Scan {
            compute,
            iter: self,
            state: initial,
            duplicate: |x| (x.clone(), x),
            _phantom: Default::default(),
        }
    }

    #[doc = include_str!("docs/scan-copy.md")]
    fn scan_copy<Compute, State>(
        self,
        initial: State,
        compute: Compute,
    ) -> Scan<Self, DupFn<State>, Compute, State, State>
    where
        State: Copy,
        Compute: FnMut(State, Self::Item) -> State,
    {
        Scan {
            compute,
            iter: self,
            state: initial,
            duplicate: |x| (x, x),
            _phantom: Default::default(),
        }
    }

    #[doc = include_str!("docs/scan-state-clone.md")]
    fn scan_state_clone<Compute, State, Value>(
        self,
        initial: State,
        compute: Compute,
    ) -> Scan<Self, StateFn<State, Value>, Compute, State, (State, Value)>
    where
        State: Clone,
        Compute: FnMut(State, Self::Item) -> (State, Value),
    {
        Scan {
            compute,
            iter: self,
            state: initial,
            duplicate: |(state, value)| (state.clone(), (state, value)),
            _phantom: Default::default(),
        }
    }

    #[doc = include_str!("docs/scan-state-copy.md")]
    fn scan_state_copy<Compute, State, Value>(
        self,
        initial: State,
        compute: Compute,
    ) -> Scan<Self, StateFn<State, Value>, Compute, State, (State, Value)>
    where
        State: Copy,
        Compute: FnMut(State, Self::Item) -> (State, Value),
    {
        Scan {
            compute,
            iter: self,
            state: initial,
            duplicate: |(state, value)| (state, (state, value)),
            _phantom: Default::default(),
        }
    }

    #[doc = include_str!("docs/scan-with-tuple.md")]
    fn scan_with_tuple<Compute, State, Value>(
        self,
        initial: State,
        compute: Compute,
    ) -> Scan<Self, IdFn<(State, Value)>, Compute, State, Value>
    where
        Compute: FnMut(State, Self::Item) -> (State, Value),
    {
        Scan {
            compute,
            iter: self,
            state: initial,
            duplicate: |x| x,
            _phantom: Default::default(),
        }
    }
}

impl<X: Iterator + Sized> IterScan for X {}

/// An iterator to maintain state while iterator over another iterator.
///
/// This `struct` is created by either [`IterScan::scan_clone`], [`IterScan::scan_copy`],
/// or [`IterScan::scan_with_tuple`], see their documentation for more.
#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Scan<Iter, Duplicate, Compute, State, Value> {
    iter: Iter,
    state: State,
    duplicate: Duplicate,
    compute: Compute,
    _phantom: PhantomData<Value>,
}

impl<Iter, Duplicate, Compute, State, Value, Intermediate> Iterator
    for Scan<Iter, Duplicate, Compute, State, Value>
where
    Iter: Iterator,
    Duplicate: FnMut(Intermediate) -> (State, Value),
    Compute: FnMut(State, Iter::Item) -> Intermediate,
{
    type Item = Value;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let Scan {
            iter,
            state,
            duplicate,
            compute,
            ..
        } = self;
        let x = iter.next()?;
        let y = replace_with_or_abort_and_return(state, |state| {
            let (state, y) = duplicate(compute(state, x));
            (y, state)
        });
        Some(y)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
