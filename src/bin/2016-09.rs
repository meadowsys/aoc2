#![no_implicit_prelude]
extern crate wiwi;
extern crate nom;
use wiwi::aoc::prelude::*;

use nom::Parser;
use nom::bytes::complete::{ self as bytes, * };
use nom::multi::*;
use nom::number::complete::*;
use nom::sequence::*;

fn main() {
	let input = get_input_buf(2016, 9);

	// we workin with ASCII now aha
	assert!(input.is_ascii());

	let decompressed = decompress(&input, true);
	print_p1(decompressed.len());

	let decompressed = decompress(&input, false);
	print_p2(decompressed.len());
}

fn decompress(mut input: &[u8], v1: bool) -> Vec<u8> {
	let inner_marker = tuple((
		take_while::<_, &[u8], _>(|c| c.is_ascii_digit()),
		tag::<_, _, ()>(b"x"),
		take_while::<_, &[u8], _>(|c| c.is_ascii_digit())
	));

	let mut parse_marker = delimited(
		tag(b"("),
		inner_marker,
		tag(b")")
	);

	let mut decompressed = Vec::new();
	loop {
		let stuff;
		(input, stuff) = take_while::<_, _, ()>(|c: u8| c.is_ascii_alphanumeric() || c == b')')(input).unwrap();
		decompressed.extend_from_slice(stuff);
		if input.is_empty() { break }

		let marker;
		(input, marker) = parse_marker(input).unwrap();
		let (amount, _, repeat) = marker;
		let amount = str::from_utf8(amount).unwrap().parse().unwrap();
		let repeat = str::from_utf8(repeat).unwrap().parse().unwrap();

		let stuff;
		(input, stuff) = bytes::take::<usize, &[u8], ()>(amount)(input).unwrap();
		if v1 {
			(0..repeat).for_each(|_| decompressed.extend_from_slice(stuff));
		} else {
			let stuff = decompress(stuff, false);
			(0..repeat).for_each(|_| decompressed.extend_from_slice(&stuff));
		}
	}
	decompressed
}
