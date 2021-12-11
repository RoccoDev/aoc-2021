use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

struct SegmentDisplay {
    patterns: Vec<FxHashSet<char>>,
    output: Vec<Vec<char>>,
}

struct Parser {
    numbers: Vec<FxHashSet<char>>,
    inputs: Vec<FxHashSet<char>>
}

impl Parser {
    pub fn new(display: &SegmentDisplay) -> Self {
        let defaults: FxHashMap<_, _> = display.patterns.iter().map(|v| (v.len(), v)).collect();
        let mut numbers = Vec::with_capacity(10);
        for _ in 0..10 {
            numbers.push(Default::default());
        }
        let inputs = display.patterns.iter().filter(|s| {
            let len = s.len();
            len != 2 && len != 4 && len != 3 && len != 7
        }).cloned().collect();

        numbers[1] = defaults[&2].clone();
        numbers[4] = defaults[&4].clone();
        numbers[7] = defaults[&3].clone();
        numbers[8] = defaults[&7].clone();

        Parser {
            numbers, inputs
        }
    }

    pub fn parse(mut self, output: &[Vec<char>]) -> usize {
        calc_spots(&mut self.numbers, &self.inputs);
        let number_map: FxHashMap<_, _> = self.numbers.iter().enumerate().map(|(i, x)| (x.iter().copied().sorted().collect::<Vec<_>>(), i)).collect();

        let mut sum = 0;
        let mut digit = 1000;
        for dig in output {
            let num = number_map[dig];
            sum += num * digit;
            digit /= 10;
        }
        sum
    }
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Vec<SegmentDisplay> {
    input.lines().map(|l| {
        let (patterns, output) = l.split(" | ").collect_tuple().unwrap();
        let patterns: Vec<_> = patterns.split_whitespace().map(|s| s.chars().sorted().collect()).collect();
        let output: Vec<_> = output.split_whitespace().map(|s| s.chars().sorted().collect()).collect();
        SegmentDisplay {
            patterns,
            output,
        }
    }).collect()
}

#[aoc(day8, part1)]
fn part1(input: &[SegmentDisplay]) -> usize {
    input.iter().map(|d| {
        let map = d.output.iter().counts_by(|v| v.len());
        map.get(&2).unwrap_or(&0) +
            map.get(&3).unwrap_or(&0) +
            map.get(&4).unwrap_or(&0) +
            map.get(&7).unwrap_or(&0)
    }).sum()
}

#[aoc(day8, part2)]
fn part2(input: &[SegmentDisplay]) -> usize {
    input.iter().map(|d| Parser::new(d).parse(&d.output)).sum()
}


fn calc_spots(number_reprs: &mut [FxHashSet<char>], inputs: &[FxHashSet<char>]) {
    let top = first_diff(&number_reprs[7], &number_reprs[1]);

    // 6 contains everything in top + (8 - 7)
    let diff_8_7 = diff(&number_reprs[8], &number_reprs[7]);
    number_reprs[6] = inputs.iter().find(|s| s.contains(&top) && s.is_superset(&diff_8_7)).cloned().unwrap();

    // top_right is 8 - 6
    let top_right = first_diff(&number_reprs[8], &number_reprs[6]);

    let diff_8_4 = diff(&number_reprs[8], &number_reprs[4]);
    // From 8, remove 4, remove (3) and you should get only one number (which is bottom left)
    // Then you'd have either 5, 9, or 3, but 3 has top_right and has length 5
    let (three, bottom_left) = inputs.iter().find_map(|s| {
        if s.len() != 5 /* filter out 9 */ || !s.contains(&top_right) /* filter out 5 */ {
            return None;
        }
        let diff = diff_8_4.difference(s).collect::<Vec<_>>();
        (diff.len() == 1).then(|| (s.clone(), *diff[0]))
    }).unwrap();
    number_reprs[3] = three;
    // The other is 5
    number_reprs[5] = inputs.iter().find(|&s| s.len() == 5 && !s.contains(&top_right)).cloned().unwrap();
    // The other 5-length is find
    number_reprs[2] = inputs.iter().find(|&s| s.len() == 5 && *s != number_reprs[3] && *s != number_reprs[5]).cloned().unwrap();

    // 6-length is either 6, 9, or 0
    number_reprs[0] = inputs.iter().find_map(|s| {
        if s.len() != 6 {
            return None;
        }
        let diff = number_reprs[8].difference(s).collect::<Vec<_>>();
        (diff.len() == 1 && *diff[0] != bottom_left && *diff[0] != top_right).then(|| s.clone())
    }).unwrap();
    // Other one is 9
    number_reprs[9] = inputs.iter().find(|&s| s.len() == 6 && *s != number_reprs[6] && *s != number_reprs[0]).cloned().unwrap();
}

fn first_diff(s1: &FxHashSet<char>, s2: &FxHashSet<char>) -> char {
    *s1.difference(s2).next().unwrap()
}

fn diff(s1: &FxHashSet<char>, s2: &FxHashSet<char>) -> FxHashSet<char> {
    s1.difference(s2).copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;
        assert_eq!(part1(&parse(input)), 26);
    }

    #[test]
    fn part2_example() {
        let input = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;
        assert_eq!(part2(&parse(input)), 61229);
    }
}
