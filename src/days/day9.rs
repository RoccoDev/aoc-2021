use fxhash::FxHashSet;
use itertools::Itertools;

struct Grid {
    inner: Vec<Vec<char>>,
    line_len: isize,
}

struct GridIter<'g> {
    pos: (isize, isize),
    grid: &'g Grid,
}

impl Grid {
    pub fn from(raw: Vec<Vec<char>>) -> Self {
        Grid {
            line_len: raw[0].len() as isize,
            inner: raw,
        }
    }

    pub fn iter_windows(&self, start_pos: (isize, isize)) -> GridIter<'_> {
        GridIter {
            pos: start_pos,
            grid: self,
        }
    }
}

impl<'g> Iterator for GridIter<'g> {
    type Item = [Option<((isize, isize), &'g char)>; 5];

    fn next(&mut self) -> Option<Self::Item> {
        let mut window = [None; 5];

        let mut i = 0;
        for x in -1..=1 {
            for z in -1..=1 {
                if x == 0 || z == 0 {
                    window[i] = self.grid.inner.get((self.pos.1 + z) as usize)
                        .and_then(|v| v.get((self.pos.0 + x) as usize))
                        .map(|c| ((self.pos.0 + x, self.pos.1 + z), c));
                    i += 1;
                }
            }
        }
        self.pos.0 += 1;
        if self.pos.0 >= self.grid.line_len {
            self.pos.0 = 0;
            self.pos.1 += 1;
        }
        window[2].map(|_| window)
    }
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Grid {
    Grid::from(input.lines().map(|s| s.chars().collect()).collect())
}

#[aoc(day9, part1)]
fn part1(input: &Grid) -> u32 {
    input.iter_windows((0, 0))
        .filter_map(|w| {
            let mid = *w[2].unwrap().1;
            w[0..2].iter().chain(w[3..5].iter()).flatten().all(|(_, &c)| c > mid)
                .then(|| 1 + mid.to_digit(10).unwrap())
        })
        .sum()
}

#[aoc(day9, part2)]
fn part2(input: &Grid) -> usize {
    let low_points = input.iter_windows((0, 0))
        .filter_map(|w| {
            let (mid_point, &mid) = w[2].unwrap();
            w[0..2].iter().chain(w[3..5].iter()).flatten().all(|(_, &c)| c > mid)
                .then(|| (mid_point, mid))
        });
    let sizes = low_points
        .map(|((x, z), point)| {
            let mut points = FxHashSet::default();
            spread_basin(input, (x, z), point, &mut points);
            points.len() + 1
        });
    sizes.sorted().rev().take(3).product()
}

fn spread_basin(grid: &Grid, start_pos: (isize, isize), value: char, points: &mut FxHashSet<(isize, isize)>) {
    for ((x, z), new_value) in grid.iter_windows(start_pos).next().into_iter()
        .flat_map(|a| a.into_iter().flatten())
        .filter(|(pos, &v)| v != '9' && *pos != start_pos && (value as u8) <= v as u8) {
        let pos = (x, z);
        if !points.contains(&pos) {
            points.insert(pos);
            spread_basin(grid, pos, *new_value, points);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;
        assert_eq!(part1(&parse(input)), 15);
    }

    #[test]
    fn part2_example() {
        let input = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;
        assert_eq!(part2(&parse(input)), 1134);
    }
}
