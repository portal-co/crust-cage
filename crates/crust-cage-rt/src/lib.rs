#![no_std]
#![feature(generic_const_exprs)]
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[doc(hidden)]
pub use bytemuck;
use bytemuck::{Pod, Zeroable};
#[doc(hidden)]
pub use core;
use core::mem::ManuallyDrop;
#[cfg(feature = "alloc")]
#[doc(hidden)]
pub extern crate alloc;
#[cfg(feature = "alloc")]
pub trait CageVec: CageSlice {
    fn vec(&mut self) -> &mut Vec<u8>;
    fn to_cage_slice(&mut self) -> &mut (dyn CageSlice + '_);
}
pub trait CageSlice {
    fn slice(&mut self) -> &mut [u8];
}
pub trait cage_Copy: cage_Clone {}
impl<T: cage_Clone> cage_Copy for T {}
pub trait cage_Clone: Copy {}
impl<T: Copy> cage_Clone for T {}
pub trait PodExt: bytemuck::Pod {
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
impl<T: Pod> PodExt for T {}
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct StackVCage<const N: usize>(pub [u8; N]);
unsafe impl<const N: usize> Zeroable for StackVCage<N> {}
unsafe impl<const N: usize> Pod for StackVCage<N> {}
impl<const N: usize> CageSlice for StackVCage<N> {
    fn slice(&mut self) -> &mut [u8] {
        &mut self.0
    }
}
#[macro_export]
macro_rules! max {
    () => {
        0
    };
    (,$($b:tt)*) => {
        $crate::max!($($b)*)
    };
    ([$a:expr] $($b:tt)*) => {
        match $a{
            a => match $crate::max!($($b)*){
                b => if a > b{
                    a
                }else{
                    b
                }
            }
        }
    }
}
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub union Pay<A, B> {
    pub first: ManuallyDrop<A>,
    pub second: ManuallyDrop<B>,
}
unsafe impl<A: Zeroable, B: Zeroable> Zeroable for Pay<A, B> {}
unsafe impl<A: Pod, B: Pod> Pod for Pay<A, B> {}
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Tup<A, B> {
    pub first: A,
    pub second: B,
}
unsafe impl<A: Zeroable, B: Zeroable> Zeroable for Tup<A, B> {}
unsafe impl<A: Pod, B: Pod> Pod for Tup<A, B> {}
#[macro_export]
macro_rules! newtype {
    (<$($a:ident $(: [$($p:tt)*])?),*>$i:ident = $j:ty) => {
        #[derive(Clone, Copy)]
        #[repr(transparent)]
        pub struct $i<$($a $(: $($p)*)?),*>(pub $j);
        const _: ()={
            unsafe impl<$($a $(: $($p)*)?),*> $crate::bytemuck::Zeroable for $i<$($a),*> where $j:  $crate::bytemuck::Zeroable{

            }
            unsafe impl<$($a $(: $($p)*)?),*> $crate::bytemuck::Pod for $i<$($a),*> where $j:  $crate::bytemuck::Pod{

            }
        };
    };
}
