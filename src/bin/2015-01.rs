#![no_implicit_prelude]
extern crate wiwi;
use wiwi::aoc::prelude::*;

fn main() {
	let input = get_input(2015, 1);

	// part 1
	let floor = input
		.chars()
		.map(get_value)
		.sum::<isize>();
	print_p1(floor);

	let mut val = 0isize;
	let floor = input
		.chars()
		.enumerate()
		.find(|(i, c)| {
			val += get_value(*c);
			val.is_negative()
		})
		.unwrap()
		.0;
	print_p2(floor + 1);
}

fn get_value(c: char) -> isize {
	match c {
		'(' => { 1 }
		')' => { -1 }
		_ => { unreachable!() }
	}
}
