//! Iteration over enum values.

use std::marker::PhantomData;

/// Allows [`EnumIterator`] to iterate over an enum type.
pub trait IterableEnum: Sized {
    /// Given an index into the iterator, return the value and the next index.
    ///
    /// Iterators shall start by calling this with `0`.
    fn iter_next(v: usize) -> Option<(usize, Self)>;

    /// Instantiates an [`EnumIterator`] for an iterable type.
    fn iter() -> EnumIterator<Self>;
}

impl IterableEnum for u16 {
    fn iter_next(v: usize) -> Option<(usize, Self)> {
        if v < 0xffff {
            Some((v + 1, v as u16))
        } else {
            None
        }
    }

    fn iter() -> EnumIterator<Self> {
        EnumIterator::new(0)
    }
}

/// Iterates over all values of an enum.
#[derive(Copy, Clone, Debug)]
pub struct EnumIterator<T> {
    count: usize,
    _marker: PhantomData<fn() -> T>,
}

impl<T: IterableEnum> EnumIterator<T> {
    pub(crate) fn new(start: usize) -> Self {
        EnumIterator {
            count: start,
            _marker: PhantomData,
        }
    }
}

impl<T: IterableEnum> Iterator for EnumIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match T::iter_next(self.count) {
            Some((new_count, res)) => {
                self.count = new_count;
                Some(res)
            },
            None => None,
        }
    }
}
