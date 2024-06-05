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
use std::fmt::Display;
use crate::matrix::*;
use crate::unsigned::*;

pub struct Galois<T> {
	order: T,
	base: T
}

impl<T: Unsigned + Display> Galois<T> {
	pub const fn irr_poly(order: T, base: T) -> Galois<T> {
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
		if prod.lt(&self.order) {return prod;}
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
			if x == 4 {break;}
			(r0, r1) = (r1, r);
			(t0, t1) = (t1.copy(), t0.xor(&self.mul(&t1, &q)));
		}
		t0
	}

	fn reduce_left(&self, matrix: &mut Matrix<T>, i: usize, a_inv: &T) {
		let mut j = i + 1;
		let col = matrix.col();
		while j < col {
			matrix.set(i, j, self.mul(&matrix.get(i, j), a_inv));
			j += 1;
		}
	}

	fn reduce_all_left(&self, matrix: &mut Matrix<T>, i: usize, k: usize) {
		let col = matrix.col();
		let a = matrix.get(k, i);
		let mut j = i;
		while j < col {
			matrix.set(k, j, matrix.get(k, j).xor(
				&self.mul(&matrix.get(i, j), &a)));
			j += 1;
		}
	}

	fn reduce_all(&self, matrix: &mut Matrix<T>, i: usize) {
		let row = matrix.row();
		let mut k = 0;
		while k < row {
			if k != i {self.reduce_all_left(matrix, i, k)};
			k += 1;
		}
	}

	pub fn solve_linear(&self, matrix: &mut Matrix<T>) {
		let (row, col) = (matrix.row(), matrix.col());
		assert!{row + 1 == col, "augmented matrix has wrong dimensions"};
		let mut i = 0;
		while i < row {
			let a = matrix.get(i, i);
			matrix.set(i, i, T::one());
			let a_inv = self.inv(&a);
			self.reduce_left(matrix, i, &a_inv);
			self.reduce_all(matrix, i);
			i += 1;
		}
	}
}
