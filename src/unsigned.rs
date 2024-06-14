/*
 * unsigned.rs
 *
 * This file is part of the project - deniable-encryption
 * authored by Udipta Pathak (udiptapathak00@gmail.com)
 *
 * Source code may be used and modified by anyone to produce their work in any
 * form under the condition: give credit to this project where it is used.
 *
 * This project comes without warranty.
 *
 * Further refer to the license attached to the project root.
 */

pub trait Number<T>: Copy
	+ std::cmp::PartialEq<T>
	+ std::cmp::PartialOrd
	+ std::fmt::Display
	+ std::ops::BitXor<Output = T>
	+ std::ops::BitXorAssign
	+ std::ops::ShlAssign<usize>
	+ std::ops::ShrAssign<usize>
{}

impl<T> Number<T> for T
where T: Copy 
	+ std::cmp::PartialEq
	+ std::cmp::PartialOrd
	+ std::fmt::Display
	+ std::ops::BitXor<Output = T>
	+ std::ops::BitXorAssign
	+ std::ops::ShlAssign<usize>
	+ std::ops::ShrAssign<usize>
{}

pub trait Unsigned {
	fn isset(&self, bit: &usize) -> bool;
	fn lmb_pos(&self) -> usize;
	fn one() -> Self;
	fn zero() -> Self;
}

macro_rules! impl_for_unsigned {
	() => {
		#[inline(always)]
		fn isset(&self, bit: &usize) -> bool {*self & (1 << bit) != 0}
		#[inline(always)]
		fn lmb_pos(&self) -> usize {
			let mut value = *self;
			let mut bit = 0;
			while value.ne(&Self::zero()) {
				value >>= 1;
				bit += 1;
			}
			bit
		}
		#[inline(always)]
		fn one() -> Self {1 as Self}
		#[inline(always)]
		fn zero() -> Self {0 as Self}
	}
}

impl Unsigned for u8 {impl_for_unsigned!{}}
impl Unsigned for u16 {impl_for_unsigned!{}}
impl Unsigned for u32 {impl_for_unsigned!{}}
impl Unsigned for u64 {impl_for_unsigned!{}}
impl Unsigned for u128 {impl_for_unsigned!{}}

