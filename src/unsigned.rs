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


pub trait Unsigned {
	fn and(&self, other: &Self) -> Self;
	fn copy(&self) -> Self;
	fn eq(&self, other: &Self) -> bool;
	fn isset(&self, bit: &usize) -> bool;
	fn le(&self, other: &Self) -> bool;
	fn lmb_pos(&self) -> usize;
	fn lt(&self, other: &Self) -> bool;
	fn mul(&self, other: &Self) -> Self;
	fn one() -> Self;
	fn shl(&mut self);
	fn shli(&mut self, bit: &usize);
	fn shr(&mut self);
	fn xor(&self, other: &Self) -> Self;
	fn zero() -> Self;
}

macro_rules! impl_for_unsigned {
	() => {
		#[inline(always)]
		fn and(&self, other: &Self) -> Self {*self & *other}
		#[inline(always)]
		fn copy(&self) -> Self {*self}
		#[inline(always)]
		fn eq(&self, other: &Self) -> bool {*self == *other}
		#[inline(always)]
		fn isset(&self, bit: &usize) -> bool {*self & (1 << bit) != 0}
		#[inline(always)]
		fn le(&self, other: &Self) -> bool {*self <= *other}
		fn lmb_pos(&self) -> usize {
			let mut value = *self;
			let mut bit = 0;
			while value.ne(&Self::zero()) {
				value.shr();
				bit += 1;
			}
			bit
		}
		#[inline(always)]
		fn lt(&self, other: &Self) -> bool {*self < *other}
		#[inline(always)]
		fn mul(&self, other: &Self) -> Self {*self * *other}
		#[inline(always)]
		fn one() -> Self {1 as Self}
		#[inline(always)]
		fn shl(&mut self) {*self <<= 1;}
		#[inline(always)]
		fn shli(&mut self, bit: &usize) {*self <<= bit}
		#[inline(always)]
		fn shr(&mut self) {*self >>= 1;}
		#[inline(always)]
		fn xor(&self, other: &Self) -> Self {*self ^ *other}
		#[inline(always)]
		fn zero() -> Self {0 as Self}
	}
}

impl Unsigned for u8 {impl_for_unsigned!{}}
impl Unsigned for u16 {impl_for_unsigned!{}}
impl Unsigned for u32 {impl_for_unsigned!{}}
impl Unsigned for u64 {impl_for_unsigned!{}}

