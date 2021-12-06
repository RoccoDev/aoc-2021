
#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<u8> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

#[aoc(day6, part1)]
fn part1(input: &[u8]) -> usize {
    calc(input, 80)
}

#[aoc(day6, part2)]
fn part2(input: &[u8]) -> usize {
    calc(input, 256)
}

// Feels so good predicting that Part 2 will just have more iterations :upside_down:
fn calc(input: &[u8], days: usize) -> usize {
    let mut timers = [0usize; 9];
    for &fish in input {
        timers[fish as usize] += 1;
    }
    // We don't have to track states and lose our mind exponentially, we just need to count
    // the fish for each timer
    for _ in 0..days {
        let bearing = timers[0];

        // Shifting the array

        // Fast + unsafe method
        // Because the array is unsized, we need to use pointers to operate on slices.
        // SAFETY: usize is Copy, both slices are valid and properly aligned, and intrinsics::copy is well-defined for
        // overlapping slices.
        unsafe { std::intrinsics::copy(timers[1..9].as_ptr(), timers[0..8].as_mut_ptr(), 8) }

        // Safe method
        // timers.rotate_left(1);

        timers[timers.len() - 1] = 0; // Reset level 8 after shifting
        timers[6] += bearing;
        timers[8] += bearing;
    }
    timers.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"3,4,3,1,2"#;
        assert_eq!(part1(&parse(input)), 5934);
    }

    #[test]
    fn part2_example() {
        let input = r#"3,4,3,1,2"#;
        assert_eq!(part2(&parse(input)), 26984457539);
    }
}
