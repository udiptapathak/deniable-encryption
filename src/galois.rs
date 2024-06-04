/*
 * galois.rs
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

use std::mem;

pub struct Galois<T> {
	order: T,
	base: T
}

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

impl<T: Unsigned> Galois<T> {
	pub fn irr_poly(order: T, base: T) -> Galois<T> {
		Galois {order, base}
	}

	pub fn add(&self, a: &T, b: &T) -> T {
		a.xor(&b)
	}

	pub fn mul(&self, a: &T, b: &T) -> T {
		let mut term = b.copy();
		let mut prod = T::zero();
		let n = mem::size_of::<T>() << 3;
		let mut i = 0;
		while i < n {
			if a.isset(&i) {prod = prod.xor(&term);}
			term.shl();
			i += 1;
		}
		if prod.lt(&self.order) {
			return prod;
		}
		(_, prod) = self.div(&prod, &self.base);
		prod
	}

	pub fn div(&self, a: &T, b: &T) -> (T, T) {
		let (mut r0, mut r1) = (T::zero(), a.copy());
		let mut divisor = b.copy();
		let b1 = a.lmb_pos();
		let b2 = b.lmb_pos();
		if b1 < b2 {return (r0, r1);}
		let mut bit = b1 - b2;
		divisor.shli(&bit);
		loop {
			r0.shl();
			if !r1.le(&r1.xor(&divisor)) {
				r1 = r1.xor(&divisor);
				r0 = r0.xor(&T::one());
			}
			if bit == 0 {break;}
			divisor.shr();
			bit -= 1;
		}
		(r0, r1)
	}

	pub fn inv(&self, a: &T) -> T {
		let (mut t0, mut t1) = (T::zero(), T::one());
		let (mut r0, mut r1) = (self.base.copy(), a.copy());
		let zero = T::zero();
		let mut x = 0;
		while !r1.eq(&zero) {
			let (q, r) = self.div(&r0, &r1);
			x += 1;
			if x == 4 {
				break;
			}
			(r0, r1) = (r1, r);
			(t0, t1) = (t1.copy(), t0.xor(&self.mul(&t1, &q)));
		}
		t0
	}
}
