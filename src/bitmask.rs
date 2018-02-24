use std::mem;

pub trait BitmaskTrait {
    type Array: Sized;
    type Index;

    fn array_slice(array: &Self::Array) -> &[u8];
    fn array_slice_mut(array: &mut Self::Array) -> &mut [u8];
    fn index(index: &Self::Index) -> usize;
}

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

    pub fn index(index: &T::Index) -> (usize, usize) {
        let index = T::index(index);
        (index / 8, index % 8)
    }

    pub fn get(&self, index: &T::Index) -> bool {
        let (offset, shift) = Self::index(index);
        (T::array_slice(&self.mask)[offset] & (1u8 << shift)) != 0
    }

    pub fn set(&mut self, index: &T::Index) {
        let (offset, shift) = Self::index(index);
        let v = &mut T::array_slice_mut(&mut self.mask)[offset];
        *v &= !(1u8 << shift);
        *v |= 1u8 << shift;
    }

    pub fn flip(&mut self, index: &T::Index) {
        let (offset, shift) = Self::index(index);
        T::array_slice_mut(&mut self.mask)[offset] ^= 1u8 << shift;
    }
}
