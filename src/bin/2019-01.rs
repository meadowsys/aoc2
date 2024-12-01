#![no_implicit_prelude]
extern crate wiwi;
use wiwi::aoc::prelude::*;
use wiwi::chain::*;

fn main() {
	let input = get_input(2019, 1);

	let p1 = input.trim().lines().map(|l| l.parse().unwrap()).map(fuel_for).sum::<usize>();
	print_p1(p1);
}

fn fuel_for(mass: usize) -> usize {
	(mass / 3) - 2
}
