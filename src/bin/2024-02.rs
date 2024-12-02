#![no_implicit_prelude]
#![feature(array_windows, iter_map_windows)]
extern crate wiwi;
use wiwi::aoc::prelude::*;

fn main() {
	let input = get_input(2024, 2);

	let input = input.lines().map(|line| line.split(' ').map(|num| num.parse().unwrap()).collect::<Vec<usize>>()).collect::<Vec<_>>();

	let p1 = input.iter().filter(|vec| is_safe(vec.iter().copied()).unwrap_or(true)).count();
	print_p1(p1);

	let p2 = input.iter().filter(|vec| (0..vec.len()).any(|del| is_safe(vec.iter().copied().enumerate().filter(|(i, _)| *i != del).map(|(_, n)| n)).unwrap_or(true))).count();
	print_p2(p2);

}

fn is_safe<I>(iter: I) -> Option<bool>
where
	I: IntoIterator<Item = usize>
{
	let mut iter = iter.into_iter().peekable();

	let a = iter.next()?;
	let b = *iter.peek()?;
	let op = if a < b {
		|a, b| a < b && (1..=3).contains(&(b - a))
	} else {
		|a, b| b < a && (1..=3).contains(&(a - b))
	};

	if !op(a, b) { return Some(false) }

	Some(iter.map_windows::<_, _, 2>(|[a, b]| op(*a, *b)).all(identity))
}
