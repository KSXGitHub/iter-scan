use crate::PseudoFunc;
use core::marker::PhantomData;
use replace_with::replace_with_or_abort_and_return;

/// An internal iterator to maintain state while iterating over the source iterator.
#[derive(Debug, Clone, Copy)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Iter<Source, Duplicate, Compute, State, Value> {
    pub(crate) source: Source,
    pub(crate) state: State,
    pub(crate) compute: Compute,
    pub(crate) _phantom: PhantomData<(Duplicate, Value)>,
}

impl<Source, Duplicate, Compute, State, Value, Intermediate> Iterator
    for Iter<Source, Duplicate, Compute, State, Value>
where
    Source: Iterator,
    Duplicate: PseudoFunc<Intermediate, (State, Value)>,
    Compute: FnMut(State, Source::Item) -> Intermediate,
{
    type Item = Value;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let Iter {
            source,
            state,
            compute,
            _phantom,
        } = self;
        let x = source.next()?;
        let y = replace_with_or_abort_and_return(state, |state| {
            let (state, y) = Duplicate::exec(compute(state, x));
            (y, state)
        });
        Some(y)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.source.size_hint()
    }
}
