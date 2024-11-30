#![no_implicit_prelude]
extern crate wiwi;
use wiwi::aoc::prelude::*;

use std::collections::VecDeque;

fn main() {
	let input = get_input(2016, 8);

	let width = 50;
	let height = 6;

	let mut display = Display::new(width, height);
	input
		.trim()
		.lines()
		.filter(|l| !l.trim().is_empty())
		.for_each(|line| {
			let mut words = line.trim().split(' ');
			match words.next().unwrap().trim() {
				"rect" => {
					let word = words.next().unwrap();
					assert!(words.next().is_none());

					let (a, b) = word.split_once('x').unwrap();
					let a = a.parse().unwrap();
					let b = b.parse().unwrap();

					display.rect(a, b);
				}

				"rotate" => {
					let is_row = match words.next().unwrap().trim() {
						"row" => { true }
						"column" => { false }
						_ => { unreachable!() }
					};

					let arg1 = words.next().unwrap().trim();
					let by = words.next().unwrap().trim();
					let m = words.next().unwrap().trim();
					assert!(by == "by");
					assert!(words.next().is_none());

					let (direction, rc) = arg1.split_once('=').unwrap();
					assert!(direction == if is_row { "y" } else { "x" });

					let rc = rc.parse().unwrap();
					let n = m.parse().unwrap();

					if is_row {
						display.row(rc, n);
					} else {
						display.col(rc, n);
					}
				}

				_ => { unreachable!() }
			}
		});

	print_p1(display.p1_count());
	print_p2(display.p2_print());
}

/// topleft || d[0]    | d[1]    | d[2]    ...
/// d[*][0] || d[0][0] | d[1][0] | d[2][0]
/// d[*][1] || d[0][1] | d[1][1] | d[2][1]
/// d[*][2] || d[0][2] | d[1][2] | d[2][2]
/// d[*][3] || d[0][3] | d[1][3] | d[2][3]
/// ...
struct Display {
	//      cols     rows
	inner: VecDeque<VecDeque<bool>>
}

impl Display {
	fn new(w: usize, h: usize) -> Self {
		let col = iter::repeat(false).take(h).collect();
		Self { inner: iter::repeat(col).take(w).collect() }
	}

	fn rect(&mut self, a: usize, b: usize) {
		for a in 0..a {
			for b in 0..b {
				self.inner[a][b] = true;
			}
		}
	}

	fn row(&mut self, r: usize, n: usize) {
		// perform rotation of every col
		self.inner.rotate_right(n);

		// record out rotated for requested col
		let rotated = self.inner.iter().map(|c| c[r]).collect::<Vec<_>>();

		// rotate every col back
		self.inner.rotate_left(n);

		// write in values
		self.inner.iter_mut().zip(rotated).for_each(|(c, rotated)| c[r] = rotated);
	}

	fn col(&mut self, c: usize, n: usize) {
		self.inner[c].rotate_right(n);
	}

	fn p1_count(&self) -> usize {
		self.inner.iter().flatten().filter(|cell| **cell).count()
	}

	fn p2_print(&self) -> String {
		iter::once('\n').chain(self.inner.iter().flat_map(|c| c.iter().map(|r| if *r { '#' } else { ' ' }).chain(iter::once('\n')))).collect()
	}
}
