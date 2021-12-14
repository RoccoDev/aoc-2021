use fxhash::FxHashMap;
use itertools::Itertools;

struct Input {
    template: String,
    instructions: FxHashMap<(char, char), char>
}

#[aoc_generator(day14)]
fn parse(input: &str) -> Input {
    let (template, instructions) = input.split("\n\n").collect_tuple().unwrap();
    let instructions = instructions.lines().map(|s| {
        let (from, between) = s.split(" -> ").collect_tuple().unwrap();
        (from.chars().collect_tuple().unwrap(), between.chars().next().unwrap())
    }).collect();
    Input { template: template.to_string(), instructions }
}

#[aoc(day14, part1)]
fn part1(input: &Input) -> usize {
    calc(input, 10)
}

#[aoc(day14, part2)]
fn part2(input: &Input) -> usize {
    calc(input, 40)
}

fn calc(input: &Input, steps: usize) -> usize {
    let mut pairs: FxHashMap<(char, char), usize> = FxHashMap::default();
    let mut chars: FxHashMap<char, usize> = FxHashMap::default();
    chars.insert(input.template.chars().next().unwrap(), 1);

    for inst in &input.instructions {
        pairs.insert(*inst.0, 0);
    }

    for (a, b) in input.template.chars().tuple_windows() {
        pairs.entry((a, b)).and_modify(|i| *i += 1);
        *chars.entry(b).or_insert(0) += 1;
    }

    for _ in 0..steps {
        let mut new_pairs = pairs.clone();
        for ((a, b), count) in pairs {
            if count == 0 {
                continue;
            }
            // Each pair generates 2 new pairs and removes itself
            let between = input.instructions[&(a, b)];
            let new = ((a, between), (between, b));
            *chars.entry(between).or_insert(0) += count;
            new_pairs.entry((a, b)).and_modify(|i| *i -= count);
            new_pairs.entry(new.0).and_modify(|i| *i += count);
            new_pairs.entry(new.1).and_modify(|i| *i += count);
        }
        pairs = new_pairs;
    }
    chars.into_values().minmax().into_option().map(|(min, max)| max - min).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;
        assert_eq!(part1(&parse(input)), 1588);
    }

    #[test]
    fn part2_example() {
        let input = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;
        assert_eq!(part2(&parse(input)), 2188189693529);
    }
}
