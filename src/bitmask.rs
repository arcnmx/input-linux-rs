//! Bitmasks are a set of bits that can be indexed and queried by the values of
//! various enum types.

use std::fmt;
use std::ops::{Deref, DerefMut};
use crate::EventKind;
use crate::enum_iterator::{IterableEnum, EnumIterator};

/// A generic trait that can be used to index by a given type into a set of bits.
pub trait BitmaskTrait {
    /// The underlying array storage data type.
    type Array: Sized;
    /// The type that the bitmask can be indexed by.
    type Index: Copy + IterableEnum;
    /// Empty default data.
    const ZERO: Self::Array;

    /// The default empty state of the bitmask.
    fn array_default() -> Self::Array;
    /// A representation of the bitmask as a slice of bytes.
    fn array_slice(array: &Self::Array) -> &[u8];
    /// A mutable representation of the bitmask as a slice of bytes.
    fn array_slice_mut(array: &mut Self::Array) -> &mut [u8];
    /// Computes a normalized index value.
    fn index(index: Self::Index) -> usize;
    /// Validates whether a given index is valid and in range for the bitmask.
    ///
    /// Dynamically allocated bitmasks may not be large enough to contain all indices.
    fn index_valid(_array: &Self::Array, _index: Self::Index) -> bool { true }
}

impl BitmaskTrait for Vec<u8> {
    type Array = Self;
    type Index = u16;
    const ZERO: Self = Vec::new();

    fn array_default() -> Self::Array { vec![0u8; 0x10] }
    fn array_slice(array: &Self::Array) -> &[u8] { array }
    fn array_slice_mut(array: &mut Self::Array) -> &mut [u8] { array }
    fn index(index: Self::Index) -> usize { index as _ }
    fn index_valid(array: &Self::Array, index: Self::Index) -> bool { array.len() > index as usize }
}

/// A dynamically allocated bitmask.
pub type BitmaskVec = Bitmask<Vec<u8>>;

impl Bitmask<Vec<u8>> {
    /// Reallocate the bitmask to fit all valid code values for the given event
    /// type.
    pub fn resize(&mut self, kind: EventKind) {
        self.data_mut().resize((kind.count_bits().unwrap_or(0x80) + 7) / 8, 0);
    }
}

#[derive(Copy, Clone)]
/// A set of bits that can be indexed by specific enum values.
pub struct Bitmask<T: BitmaskTrait> {
    mask: T::Array,
}

impl<T: BitmaskTrait> Default for Bitmask<T> {
    fn default() -> Self {
        Bitmask {
            mask: T::array_default(),
        }
    }
}

impl<T: BitmaskTrait> Bitmask<T> {
    /// A new empty bitmask.
    pub const EMPTY: Self = Self {
        mask: T::ZERO,
    };

    /// Extracts the underlying bitmask data.
    pub fn into_inner(self) -> T::Array {
        self.mask
    }

    /// Iterates over all set bits.
    pub fn iter(&self) -> BitmaskIterator<T> {
        self.into_iter()
    }

    /// Borrows the underlying bitmask data.
    pub fn data(&self) -> &T::Array {
        &self.mask
    }

    /// Mutably borrows the underlying bitmask data.
    pub fn data_mut(&mut self) -> &mut T::Array {
        &mut self.mask
    }

    fn slice(&self) -> &[u8] {
        T::array_slice(&self.mask)
    }

    fn slice_mut(&mut self) -> &mut [u8] {
        T::array_slice_mut(&mut self.mask)
    }

    fn index(index: T::Index) -> (usize, usize) {
        let index = T::index(index);
        (index / 8, index % 8)
    }

    fn index_valid(&self, index: T::Index) -> bool {
        T::index_valid(&self.mask, index)
    }

    /// Unset all bits in the bitmask.
    pub fn clear(&mut self) {
        for b in self.slice_mut() {
            *b = 0;
        }
    }

    /// Gets the status of an index in the bitmask.
    pub fn get(&self, index: T::Index) -> bool {
        let (offset, shift) = Self::index(index);
        (self.slice()[offset] & (1u8 << shift)) != 0
    }

    /// Sets the status of an index in the bitmask.
    pub fn insert(&mut self, index: T::Index) {
        let (offset, shift) = Self::index(index);
        let v = &mut self.slice_mut()[offset];
        *v |= 1u8 << shift;
    }

    /// Clears the status of an index in the bitmask.
    pub fn remove(&mut self, index: T::Index) {
        let (offset, shift) = Self::index(index);
        let v = &mut self.slice_mut()[offset];
        *v &= !(1u8 << shift);
    }

    /// Inverts the status of an index in the bitmask.
    pub fn toggle(&mut self, index: T::Index) {
        let (offset, shift) = Self::index(index);
        self.slice_mut()[offset] ^= 1u8 << shift;
    }

    /// Merges the provided indices into the bitmask.
    pub fn or<I: IntoIterator<Item=T::Index>>(&mut self, indices: I) {
        for index in indices {
            self.insert(index);
        }
    }
}

impl<'a, T: BitmaskTrait> IntoIterator for &'a Bitmask<T> {
    type Item = T::Index;
    type IntoIter = BitmaskIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        BitmaskIterator {
            mask: self,
            iter: T::Index::iter(),
        }
    }
}

/// An iterator over the set bits of a bitmask.
pub struct BitmaskIterator<'a, T: BitmaskTrait + 'a> {
    mask: &'a Bitmask<T>,
    iter: EnumIterator<T::Index>,
}

impl<'a, T: BitmaskTrait> Iterator for BitmaskIterator<'a, T> {
    type Item = T::Index;

    fn next(&mut self) -> Option<Self::Item> {
        let mask = &self.mask;
        self.iter.by_ref().take_while(|i| mask.index_valid(*i)).find(|i| mask.get(*i))
    }
}

impl<T: BitmaskTrait> fmt::Debug for Bitmask<T> where T::Index: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.into_iter()).finish()
    }
}

impl<T: BitmaskTrait> Deref for Bitmask<T> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.slice()
    }
}

impl<T: BitmaskTrait> DerefMut for Bitmask<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.slice_mut()
    }
}

impl<T: BitmaskTrait> AsRef<[u8]> for Bitmask<T> {
    fn as_ref(&self) -> &[u8] {
        self.slice()
    }
}

impl<T: BitmaskTrait> AsMut<[u8]> for Bitmask<T> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.slice_mut()
    }
}
