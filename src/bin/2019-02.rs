#![no_implicit_prelude]
extern crate wiwi;
use wiwi::aoc::prelude::*;
use wiwi::chain::*;

fn main() {
	let input = get_input(2019, 2);

	let input = input.split(',').map(str::FromStr::from_str).collect::<Result<Vec<usize>, _>>().unwrap();

	let mut p1 = input.clone();
	p1[1] = 12;
	p1[2] = 2;
	let mut pos = 0;
	loop {
		match p1[pos] {
			1 => {
				let (l, r, out) = (p1[pos + 1], p1[pos + 2], p1[pos + 3]);
				p1[out] = p1[l] + p1[r];
				pos += 4;
			}
			2 => {
				let (l, r, out) = (p1[pos + 1], p1[pos + 2], p1[pos + 3]);
				p1[out] = p1[l] * p1[r];
				pos += 4;
			}
			99 => { break }
			_ => { unreachable!() }
		}
	}
	print_p1(p1[0]);
}
