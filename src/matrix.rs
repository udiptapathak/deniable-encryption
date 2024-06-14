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

use crate::unsigned::*;

pub struct Matrix<T> {
	data: Vec<T>,
	row: usize,
	col: usize
}

impl<T: Unsigned + Number<T>> Matrix<T> {
	pub fn from(buf: &[T], row: usize, col: usize) -> Matrix<T> {
		let len = buf.len();
		let mut data = Vec::<T>::with_capacity(len);
		let mut i = 0;
		while i < len {
			data.push(buf[i]);
			i += 1;
		}
		Matrix {data, row, col}
	}

	pub fn row(&self) -> usize {
		self.row
	}

	pub fn col(&self) -> usize {
		self.col
	}
}

impl<T: Number<T>> std::fmt::Display for Matrix<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut i = 0;
		let (row, col) = (self.row, self.col);
		while i < row {
			let mut j = 0;
			while j < col {
				write!(f, "{} ", self[i][j])?;
				j += 1;
			}
			i += 1;
			write!(f, "\n")?;
		}
		write!(f, "")
	}
}

impl<T> std::ops::Index<usize> for Matrix<T>
{
	type Output = [T];

	fn index(&self, index: usize) -> &Self::Output {
		&self.data[index * self.col ..]
	}
}

impl<T> std::ops::IndexMut<usize> for Matrix<T>
{
	#[inline(always)]
	fn index_mut(&mut self, index: usize) -> &mut [T] {
		&mut self.data[index * self.col ..]
	}
}
