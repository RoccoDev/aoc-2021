use itertools::Itertools;

#[aoc_generator(day1)]
fn get_nums(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> usize {
    input.windows(2).filter(|arr| arr[0] < arr[1]).count()
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> usize {
    input
        .windows(3)
        .map(|arr| arr.iter().sum::<i32>())
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = example!(199 200 208 210 200 207 240 269 260 263);
        assert_eq!(part1(&get_nums(input)), 7);
    }

    #[test]
    fn part2_example() {
        let input = example!(199 200 208 210 200 207 240 269 260 263);
        assert_eq!(part2(&get_nums(input)), 5);
    }
}
