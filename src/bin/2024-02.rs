#![no_implicit_prelude]
#![feature(array_windows)]
extern crate wiwi;
use wiwi::aoc::prelude::*;

fn main() {
	let input = get_input(2024, 2);

	let p1 = input.lines().filter(|line| {
		let vec = line.split(' ').map(|num| num.parse().unwrap()).collect::<Vec<usize>>();

		let all_inc = vec.array_windows::<2>().all(|[a, b]| a < b);
		let all_dec = vec.array_windows::<2>().all(|[a, b]| a > b);
		if !(all_inc || all_dec) { return false }

		let op: fn(usize, usize) -> usize = if all_inc { |a, b| b - a } else { |a, b| a - b };
		// let sm_diff = vec.array_windows::<2>().all
		vec.array_windows::<2>().all(|[a, b]| {
			(1..=3).contains(&op(*a, *b))
		})
	}).count();
	print_p1(p1);
}
