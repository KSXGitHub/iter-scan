#![doc = include_str!("README.md")]
#![no_std]
use core::marker::PhantomData;
use replace_with::replace_with_or_abort_and_return;

type IdFn<X> = fn(X) -> X;
type DupFn<X> = fn(X) -> (X, X);
type Tpl2<X> = (X, X);

/// Iterator scan methods that don't suck.
pub trait IterScan: Iterator + Sized {
    /// This iterator adapter holds an internal state and emit this state on each iteration.
    ///
    /// This internal state can be [cloned](Clone).
    ///
    /// `scan_clone()` takes 2 arguments:
    /// * An initial value which seeds the internal state.
    /// * A closure that:
    ///   - Takes 2 arguments: Copy of the internal state from the previous iteration and the current item.
    ///   - Returns the new state for the next iteration.
    ///
    /// **Example:** Basic usage
    ///
    /// ```rust
    /// use iter_scan::IterScan;
    /// let input = ['a', 'b', 'c', 'd', 'e', 'f'];
    /// let output: Vec<_> = input
    ///     .into_iter()
    ///     .scan_clone(String::new(), |acc, x| format!("{acc}{x}"))
    ///     .collect();
    /// assert_eq!(output, ["a", "ab", "abc", "abcd", "abcde", "abcdef"]);
    /// ```
    fn scan_clone<Compute, State>(
        self,
        initial: State,
        compute: Compute,
    ) -> Scan<Self, DupFn<State>, Compute, State>
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

    /// This iterator adapter holds an internal state and emit this state on each iteration.
    ///
    /// This internal state can be [copied](Copy).
    ///
    /// `scan_copy()` takes 2 arguments:
    /// * An initial value which seeds the internal state.
    /// * A closure that:
    ///   - Takes 2 arguments: Copy of the internal state from the previous iteration and the current item.
    ///   - Returns the new state for the next iteration.
    ///
    /// **Example:** Basic usage
    ///
    /// ```rust
    /// use iter_scan::IterScan;
    /// let input = [2, 3, 4, 5];
    /// let output: Vec<u64> = input
    ///     .into_iter()
    ///     .scan_copy(1, |acc, x| acc * x)
    ///     .collect();
    /// assert_eq!(output, [2, 6, 24, 120]);
    /// ```
    fn scan_copy<Compute, State>(
        self,
        initial: State,
        compute: Compute,
    ) -> Scan<Self, DupFn<State>, Compute, State>
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

    /// This iterator adapter holds an internal state and emit this state on each iteration.
    ///
    /// This adapter should be used when the internal state can neither be [cloned](Clone) nor [copied](Copy).
    ///
    /// `scan_with_tuple()` takes 2 arguments:
    /// * An initial value which seeds the internal state.
    /// * A closure that:
    ///   - Takes 2 arguments: Copy of the internal state from the previous iteration and the current item.
    ///   - Returns a tuple of the new state and its identical copy.
    fn scan_with_tuple<Compute, State>(
        self,
        initial: State,
        compute: Compute,
    ) -> Scan<Self, IdFn<Tpl2<State>>, Compute, State>
    where
        Compute: FnMut(State, Self::Item) -> Tpl2<State>,
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
pub struct Scan<Iter, Duplicate, Compute, State> {
    iter: Iter,
    state: State,
    duplicate: Duplicate,
    compute: Compute,
    _phantom: PhantomData<State>,
}

impl<Iter, Duplicate, Compute, State, Intermediate> Iterator
    for Scan<Iter, Duplicate, Compute, State>
where
    Iter: Iterator,
    Duplicate: FnMut(Intermediate) -> Tpl2<State>,
    Compute: FnMut(State, Iter::Item) -> Intermediate,
{
    type Item = State;

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
        let f = |state| duplicate(compute(state, x));
        let y = replace_with_or_abort_and_return(state, f);
        Some(y)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper) // can't know a lower bound, due to the scan function
    }
}
