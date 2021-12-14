use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

#[aoc_generator(day12)]
fn parse(input: &str) -> FxHashMap<i64, FxHashSet<i64>> {
    let mut map = FxHashMap::default();
    for line in input.lines() {
        let (start, end) = line.split('-').map(get_cave_id).collect_tuple().unwrap();
        map.entry(start).or_insert_with(FxHashSet::default).insert(end);
    }
    for (k, v) in map.clone().into_iter() {
        for cave in &v {
            map.entry(*cave).or_insert_with(FxHashSet::default).insert(k);
        }
    }
    map
}

#[aoc(day12, part1)]
fn part1(input: &FxHashMap<i64, FxHashSet<i64>>) -> usize {
    let (mut paths, mut small) = (0, FxHashSet::default());
    traverse(input, 0, (&mut paths, &mut small, None));
    paths
}

#[aoc(day12, part2)]
fn part2(input: &FxHashMap<i64, FxHashSet<i64>>) -> usize {
    let (mut paths, mut small) = (0, FxHashSet::default());
    traverse(input, 0, (&mut paths, &mut small, Some(false)));
    paths
}

fn traverse(paths: &FxHashMap<i64, FxHashSet<i64>>, current_cave: i64,
            (path_cnt, visited_small, mut exception): (&mut usize, &mut FxHashSet<i64>, Option<bool>)) {
    if current_cave == i64::MAX {
        *path_cnt += 1;
        return;
    } else if current_cave < 0 {
        if visited_small.contains(&current_cave) {
            if let Some(exception) = exception.as_mut() {
                if !*exception {
                    *exception = true;
                } else {
                    return;
                }
            } else {
                return;
            }
        }
        visited_small.insert(current_cave);
    }
    for path in paths.get(&current_cave).iter().flat_map(|v| v.iter()) {
        if *path != 0 {
            let mut visited_small = visited_small.clone();
            traverse(paths, *path, (path_cnt, &mut visited_small, exception));
        }
    }
}

fn get_cave_id(cave: &str) -> i64 {
    match cave {
        "start" => 0,
        "end" => i64::MAX,
        s => {
            let lowercase = s.chars().all(|c| c.is_lowercase());
            let num: i64 = s.chars().map(|c| c as u16 as i64).product();
            if lowercase { -num } else { num }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        assert_eq!(part1(&parse(input)), 19);
    }

    #[test]
    fn part2_example() {
        let input = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        assert_eq!(part2(&parse(input)), 103);
    }
}
