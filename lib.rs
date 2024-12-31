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

    /// This iterator adapter holds an internal state and emit a tuple of this state and a mapped value on each iteration.
    ///
    /// This internal state can be [cloned](Clone).
    ///
    /// `scan_state_clone()` takes 2 arguments:
    /// * An initial value which seeds the internal state.
    /// * A closure that:
    ///   - Takes 2 arguments: Copy of the internal state from the previous iteration and the current item.
    ///   - Returns the new state and the mapped value of the item.
    ///
    /// **Example:** Basic usage.
    ///
    /// ```
    /// use iter_scan::IterScan;
    /// enum SourceItem {
    ///     Separator,
    ///     Value(&'static str),
    /// }
    /// let source = [
    ///     SourceItem::Value("zero"),
    ///     SourceItem::Value("one"),
    ///     SourceItem::Value("two"),
    ///     SourceItem::Separator,
    ///     SourceItem::Value("three"),
    ///     SourceItem::Value("four"),
    ///     SourceItem::Separator,
    ///     SourceItem::Value("five"),
    ///     SourceItem::Separator,
    ///     SourceItem::Value("six"),
    /// ];
    /// let tagged: Vec<_> = source
    ///     .into_iter()
    ///     .scan_state_clone(0u32, |count, item| match item {
    ///         SourceItem::Separator => (count + 1, None),
    ///         SourceItem::Value(value) => (count, Some(value)),
    ///     })
    ///     .flat_map(|(count, item)| item.map(|item| (count, item)))
    ///     .collect();
    /// assert_eq!(
    ///     &tagged,
    ///     &[
    ///         (0, "zero"),
    ///         (0, "one"),
    ///         (0, "two"),
    ///         (1, "three"),
    ///         (1, "four"),
    ///         (2, "five"),
    ///         (3, "six"),
    ///     ],
    /// );
    /// ```
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

    /// This iterator adapter holds an internal state and emit a tuple of this state and a mapped value on each iteration.
    ///
    /// This internal state can be [copied](Copy).
    ///
    /// `scan_state_copy()` takes 2 arguments:
    /// * An initial value which seeds the internal state.
    /// * A closure that:
    ///   - Takes 2 arguments: Copy of the internal state from the previous iteration and the current item.
    ///   - Returns the new state and the mapped value of the item.
    ///
    /// **Example:** Basic usage.
    ///
    /// ```
    /// use iter_scan::IterScan;
    /// enum SourceItem {
    ///     Separator,
    ///     Value(&'static str),
    /// }
    /// let source = [
    ///     SourceItem::Value("zero"),
    ///     SourceItem::Value("one"),
    ///     SourceItem::Value("two"),
    ///     SourceItem::Separator,
    ///     SourceItem::Value("three"),
    ///     SourceItem::Value("four"),
    ///     SourceItem::Separator,
    ///     SourceItem::Value("five"),
    ///     SourceItem::Separator,
    ///     SourceItem::Value("six"),
    /// ];
    /// let tagged: Vec<_> = source
    ///     .into_iter()
    ///     .scan_state_copy(0u32, |count, item| match item {
    ///         SourceItem::Separator => (count + 1, None),
    ///         SourceItem::Value(value) => (count, Some(value)),
    ///     })
    ///     .flat_map(|(count, item)| item.map(|item| (count, item)))
    ///     .collect();
    /// assert_eq!(
    ///     &tagged,
    ///     &[
    ///         (0, "zero"),
    ///         (0, "one"),
    ///         (0, "two"),
    ///         (1, "three"),
    ///         (1, "four"),
    ///         (2, "five"),
    ///         (3, "six"),
    ///     ],
    /// );
    /// ```
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

    /// This iterator adapter holds an internal state and emit this state on each iteration.
    ///
    /// This adapter should be used when the internal state can neither be [cloned](Clone) nor [copied](Copy).
    ///
    /// `scan_with_tuple()` takes 2 arguments:
    /// * An initial value which seeds the internal state.
    /// * A closure that:
    ///   - Takes 2 arguments: Copy of the internal state from the previous iteration and the current item.
    ///   - Returns a tuple of the new state and a value.
    ///
    /// **Example:** Basic usage.
    ///
    /// ```
    /// use iter_scan::IterScan;
    /// enum SourceItem {
    ///     Separator,
    ///     Value(&'static str),
    /// }
    /// let source = [
    ///     SourceItem::Value("zero"),
    ///     SourceItem::Value("one"),
    ///     SourceItem::Value("two"),
    ///     SourceItem::Separator,
    ///     SourceItem::Value("three"),
    ///     SourceItem::Value("four"),
    ///     SourceItem::Separator,
    ///     SourceItem::Value("five"),
    ///     SourceItem::Separator,
    ///     SourceItem::Value("six"),
    /// ];
    /// let tagged: Vec<_> = source
    ///     .into_iter()
    ///     .scan_with_tuple(0u32, |prev_tag, item| match item {
    ///         SourceItem::Separator => (prev_tag + 1, None),
    ///         SourceItem::Value(value) => (prev_tag, Some((prev_tag, value))),
    ///     })
    ///     .flatten()
    ///     .collect();
    /// assert_eq!(
    ///     &tagged,
    ///     &[
    ///         (0, "zero"),
    ///         (0, "one"),
    ///         (0, "two"),
    ///         (1, "three"),
    ///         (1, "four"),
    ///         (2, "five"),
    ///         (3, "six"),
    ///     ],
    /// );
    /// ```
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
