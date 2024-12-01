#![no_implicit_prelude]
extern crate wiwi;
use wiwi::aoc::prelude::*;
use wiwi::chain::*;
use std::thread;
use atomic::Ordering::*;

fn main() {
	let input = get_input(2019, 2);

	let input = input.split(',').map(str::FromStr::from_str).collect::<Result<Vec<usize>, _>>().unwrap();

	let mut p1 = input.clone();
	p1[1] = 12;
	p1[2] = 2;
	run_intcode(&mut p1);
	print_p1(p1[0]);

	// let's try some multithreaded atomics programming for part 2 because why not
	let result = AtomicUsize::new(0);
	let mut ranges = [0usize..20, 20..40, 40..60, 60..80, 80..100];
	let expected = 19690720usize;
	thread::scope(|s| {
		for range in &mut ranges {
			s.spawn(|| {
				for noun in range {
					for verb in 0..100 {
						// does this actually have to do acquire/release? or can this
						// all be relaxed, cause nothing is like, producing new values
						// from values written from past threads or anything depending
						// on past writes
						if result.load(Acquire) != 0 { return }

						let mut input = input.clone();
						input[1] = noun;
						input[2] = verb;
						run_intcode(&mut input);

						let output = input[0];
						if output == expected {
							result.store(100 * noun + verb, Release);
						}
					}
				}
			});
		}
	});

	print_p2(result.into_inner());
}

fn run_intcode(memory: &mut [usize]) {
	let mut pos = 0;
	loop {
		match memory[pos] {
			1 => {
				let (l, r, out) = (memory[pos + 1], memory[pos + 2], memory[pos + 3]);
				memory[out] = memory[l] + memory[r];
				pos += 4;
			}
			2 => {
				let (l, r, out) = (memory[pos + 1], memory[pos + 2], memory[pos + 3]);
				memory[out] = memory[l] * memory[r];
				pos += 4;
			}
			99 => { break }
			_ => { unreachable!() }
		}
	}
}
