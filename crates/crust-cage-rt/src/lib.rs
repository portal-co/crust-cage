#![no_std]
#![feature(generic_const_exprs)]
use alloc::vec::Vec;
#[doc(hidden)]
pub use bytemuck;
use bytemuck::Pod;
#[doc(hidden)]
pub use core;
#[doc(hidden)]
pub extern crate alloc;
pub trait Cage: CageSlice {
    fn vec(&mut self) -> &mut Vec<u8>;
    fn to_cage_slice(&mut self) -> &mut (dyn CageSlice + '_);
}
pub trait CageSlice {
    fn slice(&mut self) -> &mut [u8];
}
pub trait cage_Copy: cage_Clone {}
impl<T: cage_Clone> cage_Copy for T {}
pub trait cage_Clone: Copy + bytemuck::Pod {
    fn fetch(cage: &mut (dyn CageSlice + '_), idx: usize) -> Self {
        let b: &[u8] = &cage.slice()[idx..][..size_of::<Self>()];
        bytemuck::pod_read_unaligned(b)
    }
    fn put(&self, cage: &mut (dyn CageSlice + '_), idx: usize) {
        let b = &mut cage.slice()[idx..][..size_of::<Self>()];
        b.copy_from_slice(unsafe {
            core::slice::from_raw_parts((&raw const *self).cast(), size_of_val(self))
        });
    }
}
impl<T: Copy + bytemuck::Pod> cage_Clone for T {}
