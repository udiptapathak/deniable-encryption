/*
 * matrix.rs
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

use std::fmt::Display;
use crate::unsigned::*;

pub struct Matrix<'a, T> {
	data: &'a mut [T],
	row: usize,
	col: usize
}

impl<'a, T: Unsigned> Matrix<'a, T> {
	pub fn from(data: &mut [T], row: usize, col: usize) -> Matrix<T> {
		Matrix {data, row, col}
	}

	pub fn get(&self, i: usize, j: usize) -> T {
		self.data[i * self.col + j].copy()
	}

	pub fn set(&mut self, i: usize, j: usize, value: T) {
		self.data[i * self.col + j] = value.copy();
	}

	pub fn row(&self) -> usize {
		self.row
	}

	pub fn col(&self) -> usize {
		self.col
	}
}

impl<'a, T: Unsigned + Display> Display for Matrix<'a, T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut i = 0;
		let (row, col) = (self.row, self.col);
		while i < row {
			let mut j = 0;
			while j < col {
				write!(f, "{} ", self.get(i, j))?;
				j += 1;
			}
			i += 1;
			write!(f, "\n")?;
		}
		write!(f, "")
	}
}

