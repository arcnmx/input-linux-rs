use std::{mem, fmt};
use std::ops::{Deref, DerefMut};
use enum_iterator::{IterableEnum, EnumIterator};

pub trait BitmaskTrait {
    type Array: Sized;
    type Index: IterableEnum;

    fn array_slice(array: &Self::Array) -> &[u8];
    fn array_slice_mut(array: &mut Self::Array) -> &mut [u8];
    fn index(index: &Self::Index) -> usize;
}

#[derive(Copy, Clone)]
pub struct Bitmask<T: BitmaskTrait> {
    mask: T::Array,
}

impl<T: BitmaskTrait> Default for Bitmask<T> {
    fn default() -> Self {
        Bitmask {
            mask: unsafe { mem::zeroed() },
        }
    }
}

impl<T: BitmaskTrait> Bitmask<T> {
    pub fn into_inner(self) -> T::Array {
        self.mask
    }

    pub fn data(&self) -> &T::Array {
        &self.mask
    }

    pub fn data_mut(&mut self) -> &mut T::Array {
        &mut self.mask
    }

    pub fn slice(&self) -> &[u8] {
        T::array_slice(&self.mask)
    }

    pub fn slice_mut(&mut self) -> &mut [u8] {
        T::array_slice_mut(&mut self.mask)
    }

    pub fn index(index: &T::Index) -> (usize, usize) {
        let index = T::index(index);
        (index / 8, index % 8)
    }

    pub fn get(&self, index: &T::Index) -> bool {
        let (offset, shift) = Self::index(index);
        (self.slice()[offset] & (1u8 << shift)) != 0
    }

    pub fn set(&mut self, index: &T::Index) {
        let (offset, shift) = Self::index(index);
        let v = &mut self.slice_mut()[offset];
        *v &= !(1u8 << shift);
        *v |= 1u8 << shift;
    }

    pub fn flip(&mut self, index: &T::Index) {
        let (offset, shift) = Self::index(index);
        self.slice_mut()[offset] ^= 1u8 << shift;
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

pub struct BitmaskIterator<'a, T: BitmaskTrait + 'a> {
    mask: &'a Bitmask<T>,
    iter: EnumIterator<T::Index>,
}

impl<'a, T: BitmaskTrait> Iterator for BitmaskIterator<'a, T> {
    type Item = T::Index;

    fn next(&mut self) -> Option<Self::Item> {
        let mask = &self.mask;
        self.iter.by_ref().filter(|i| mask.get(i)).next()
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
