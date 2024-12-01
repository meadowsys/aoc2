#![no_implicit_prelude]
extern crate wiwi;
use wiwi::aoc::prelude::*;
use wiwi::chain::*;

fn main() {
	let input = get_input(2024, 1);

	let (l, r) = input.trim()
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
		.into_inner();

	let p1 = l.iter()
		.copied()
		.zip(r.iter().copied())
		.fold(0usize, |acc, (l, r)| acc + (l.into_signed() - r.into_signed()).abs().into_unsigned());
	print_p1(p1);

	let r_map = r.iter()
		.copied()
		.fold(
			HashMapChain::new().into_inner().into_generic_chain(),
			|map, curr| map.with_inner(|map| {
				*map.entry(curr).or_insert(0usize) += curr;
			})
		)
		.into_inner();

	let p2 = l.iter()
		.map(|l| r_map.get(l).copied().unwrap_or_default())
		.sum::<usize>();
	print_p2(p2);
}
