use fxhash::{FxBuildHasher, FxHashSet};
use std::cell::Cell;

#[derive(Clone)]
struct Grid {
    inner: Vec<Cell<u8>>,
    line_len: isize,
}

impl Grid {
    pub fn from(raw: Vec<Vec<u8>>) -> Self {
        Grid {
            line_len: raw[0].len() as isize,
            inner: raw
                .into_iter()
                .flat_map(|v| v.into_iter())
                .map(Cell::new)
                .collect(),
        }
    }

    pub fn window(&self, center_pos: isize) -> [Option<(usize, &Cell<u8>)>; 9] {
        let mut window = [None, None, None, None, None, None, None, None, None];

        let center_x = center_pos % self.line_len;
        let center_z = center_pos / self.line_len;

        let mut i = 0;
        for x in -1isize..=1 {
            for z in -1isize..=1 {
                let x = center_x + x;
                let z = center_z + z;
                if x < 0 || z < 0 || x >= self.line_len {
                    window[i] = None;
                    continue;
                }
                let index = (x + z * self.line_len) as usize;
                window[i] = self.inner.get(index).map(|c| (index, c));
                i += 1;
            }
        }

        window
    }
}

#[aoc_generator(day11)]
fn parse(input: &str) -> Grid {
    Grid::from(
        input
            .lines()
            .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            .collect(),
    )
}

#[aoc(day11, part1)]
fn part1(input: &Grid) -> usize {
    let mut flashes = 0;
    for _ in 0..100 {
        for octopus in &input.inner {
            octopus.set(octopus.get() + 1);
        }
        let mut flashed =
            FxHashSet::with_capacity_and_hasher(input.inner.len(), FxBuildHasher::default());
        for (i, _) in input.inner.iter().enumerate().filter(|(_, v)| v.get() > 9) {
            flash(i, input, &mut flashed);
        }
        for pos in &flashed {
            input.inner[*pos].set(0);
        }
        flashes += flashed.len();
    }
    flashes
}

#[aoc(day11, part2)]
fn part2(input: &Grid) -> usize {
    let mut i = 1;
    loop {
        for octopus in &input.inner {
            octopus.set(octopus.get() + 1);
        }
        let mut flashed =
            FxHashSet::with_capacity_and_hasher(input.inner.len(), FxBuildHasher::default());
        for (i, _) in input.inner.iter().enumerate().filter(|(_, v)| v.get() > 9) {
            flash(i, input, &mut flashed);
        }
        for pos in &flashed {
            input.inner[*pos].set(0);
        }
        if flashed.len() == input.inner.len() {
            return i;
        }
        i += 1;
    }
}

fn flash(pos: usize, grid: &Grid, flashed: &mut FxHashSet<usize>) {
    if flashed.contains(&pos) || grid.inner[pos].get() <= 9 {
        return;
    }
    flashed.insert(pos);
    for (new_pos, v) in grid.window(pos as isize).into_iter().flatten() {
        if new_pos != pos {
            v.set(v.get() + 1);
            if v.get() > 9 && !flashed.contains(&new_pos) {
                flash(new_pos, grid, flashed);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;
        assert_eq!(part1(&parse(input)), 1656);
    }

    #[test]
    fn part2_example() {
        let input = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;
        assert_eq!(part2(&parse(input)), 195);
    }
}
