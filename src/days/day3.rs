use std::ops::Sub;

struct Data {
    numbers: Vec<u32>,
    /// The length of each entry
    number_len: usize,
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Data {
    let numbers = input
        .lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect();
    let len = input.lines().next().unwrap().len();
    Data {
        numbers,
        number_len: len,
    }
}

#[aoc(day3, part1)]
fn part1(input: &Data) -> u32 {
    let mut gamma = 0u32;
    for i in 0..input.number_len {
        if get_most_common(&input.numbers, i,input.number_len) == 1 {
            gamma |= 1 << (input.number_len - i - 1);
        }
    }
    let epsilon = gamma ^ ((1 << input.number_len) - 1);
    gamma * epsilon
}

#[aoc(day3, part2)]
fn part2(input: &Data) -> u32 {
    let mut numbers = input.numbers.clone();
    let mut numbers_clone = input.numbers.clone();

    reduce_rating(&mut numbers, input.number_len, true);
    reduce_rating(&mut numbers_clone, input.number_len, false);
    numbers[0] * numbers_clone[0]
}

fn reduce_rating(nums: &mut Vec<u32>, num_len: usize, pick_most_common: bool) {
    let mut i = 0;
    while nums.len() > 1 {
        let most_common = get_most_common(nums, i, num_len);
        let most_common = if pick_most_common { most_common } else { 1 - most_common };
        nums.retain(|n| ((n >> (num_len - i - 1)) & 1) as u8 == most_common);
        i += 1;
    }
}

fn get_most_common(numbers: &[u32], bit: usize, num_len: usize) -> u8 {
    let bit = num_len - bit - 1;
    let set_bits = numbers
        .iter()
        .copied()
        .filter(|n| (n >> bit) & 1 == 1)
        .count();
    let most = 2 * set_bits;
    if most >= numbers.len() {
        1 // Most common is 1 if equal
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = example!(00100 11110 10110 10111 10101 01111 00111 11100 10000 11001 00010 01010);
        assert_eq!(part1(&parse(input)), 198);
    }

    #[test]
    fn part2_example() {
        let input = example!(00100 11110 10110 10111 10101 01111 00111 11100 10000 11001 00010 01010);
        assert_eq!(part2(&parse(input)), 230);
    }
}
