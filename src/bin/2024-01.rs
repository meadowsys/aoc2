#![no_implicit_prelude]
extern crate wiwi;
use wiwi::aoc::prelude::*;
use wiwi::chain::*;

fn main() {
	let input = get_input(2024, 1);

	let p1 = input
		.trim()
		.lines()
		.map(|s| s.trim())
		.filter(|s| !s.is_empty())
		.map(|s| s.split_once("  ").unwrap())
		.map(|(l, r)| (l.trim().parse::<usize>().unwrap(), r.trim().parse::<usize>().unwrap()))
		.collect::<(Vec<usize>, Vec<usize>)>()
		.into_generic_chain()
		.with_inner(|(l, r)| {
			l.sort();
			r.sort();
		})
		.map(|(l, r)| l.into_iter().zip(r))
		.into_inner()
		.fold(0usize, |acc, (l, r)| acc + (l.into_signed() - r.into_signed()).abs().into_unsigned());
	print_p1(p1);
}
