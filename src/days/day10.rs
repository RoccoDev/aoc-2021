use std::collections::VecDeque;

#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

const OPENERS: [char; 4] = ['(', '[', '{', '<'];
const CLOSERS: [char; 4] = [')', ']', '}', '>'];

#[aoc(day10, part1)]
fn part1(input: &[String]) -> usize {
    input.iter().filter_map(|s| {
        let mut levels = VecDeque::with_capacity(s.len());
        for char in s.chars() {
            if OPENERS.contains(&char) {
                levels.push_back(char);
                continue;
            }
            let index = CLOSERS.iter().position(|&c| c == char).unwrap();
            if levels.pop_back().and_then(|l| (l == OPENERS[index]).then(|| ())).is_none() {
                return Some(char);
            }
        }
        None
    }).map(|c| match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!()
    }).sum()
}

#[aoc(day10, part2)]
fn part2(input: &[String]) -> usize {
    let mut scores = input.iter().filter_map(|s| {
        let mut levels = VecDeque::with_capacity(s.len());
        for char in s.chars() {
            let index = OPENERS.iter().position(|&c| c == char);
            if let Some(index) = index {
                levels.push_back(index);
                continue;
            }
            levels.pop_back().and_then(|i| (CLOSERS[i] == char).then(|| ()))?;
        }

        if levels.is_empty() {
            return None;
        }

        Some(levels.into_iter().rev().map(|i| match CLOSERS[i] {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => unreachable!()
        }).fold(0, |acc, x| acc * 5 + x))
    }).collect::<Vec<_>>();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;
        assert_eq!(part1(&parse(input)), 26397);
    }

    #[test]
    fn part2_example() {
        let input = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;
        assert_eq!(part2(&parse(input)), 288957);
    }
}
