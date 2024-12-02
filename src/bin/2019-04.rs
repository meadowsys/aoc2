#![no_implicit_prelude]
#![feature(array_windows)]
extern crate wiwi;
use wiwi::aoc::prelude::*;
use iter::repeat;

fn main() {
	let input = get_input(2019, 4);
	let (l, h) = input.split_once('-').unwrap();
	let l = l.parse().unwrap();
	let h = h.parse().unwrap();
	let range = l..h;

	let p1 = (1usize..=9)
		// ew
		.flat_map(|n| repeat(n).zip(n..=9))
		.flat_map(|(n1, n)| repeat((n1, n)).zip(n..=9))
		.flat_map(|((n1, n2), n)| repeat((n1, n2, n)).zip(n..=9))
		.flat_map(|((n1, n2, n3), n)| repeat((n1, n2, n3, n)).zip(n..=9))
		.flat_map(|((n1, n2, n3, n4), n)| repeat((n1, n2, n3, n4, n)).zip(n..=9))
		.filter(|((n1, n2, n3, n4, n5), n6)| {
			let digits = [n1, n2, n3, n4, n5, n6];
			if !digits.array_windows().any(|[n1, n2]| n1 == n2) { return false }
			let num = [n1, n2, n3, n4, n5, n6].into_iter().fold(0, |acc, curr| (acc * 10) + curr);
			range.contains(&num)
		})
		.count();

	print_p1(p1);
}
