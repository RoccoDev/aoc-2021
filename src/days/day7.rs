use itertools::Itertools;

struct Crabs {
    numbers: Vec<i32>,
    max: i32
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Crabs {
    let numbers: Vec<_> = input.split(',').map(|s| s.parse().unwrap()).sorted().collect();
    let max = numbers.iter().max().copied().unwrap();
    Crabs {
        numbers,
        max
    }
}

#[aoc(day7, part1)]
fn part1(input: &Crabs) -> i32 {
    (0..=input.max).map(|i| input.numbers.iter().map(|&n| (n - i).abs()).sum()).min().unwrap()
}

#[aoc(day7, part2)]
fn part2(input: &Crabs) -> i32 {
    (0..=input.max).map(|i| input.numbers.iter().map(|&n| {
        // https://en.wikipedia.org/wiki/Triangular_number
        let n = (n - i).abs();
        (n * n + n) / 2
    }).sum()).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"16,1,2,0,4,2,7,1,2,14"#;
        assert_eq!(part1(&parse(input)), 37);
    }

    #[test]
    fn part2_example() {
        let input = r#"16,1,2,0,4,2,7,1,2,14"#;
        assert_eq!(part2(&parse(input)), 168);
    }
}
