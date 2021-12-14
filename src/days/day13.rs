use fxhash::FxHashSet;
use itertools::Itertools;

#[derive(Clone)]
struct Paper {
    grid: Grid,
    folds: Vec<Fold>
}

#[derive(Clone)]
enum Fold {
    Up(usize),
    Left(usize)
}

#[derive(Clone)]
struct Grid {
    inner: Vec<(usize, usize)>,
}

impl Grid {
    fn parse(points: &[(usize, usize)]) -> Self {
        Grid {
            inner: points.iter().copied().collect()
        }
    }

    fn fold(&mut self, fold: &Fold) {
        match fold {
            Fold::Up(v) => self.fold_up(*v),
            Fold::Left(v) => self.fold_left(*v)
        }
    }

    fn fold_up(&mut self, fold_y: usize) {
        for (_, y) in &mut self.inner {
            if *y > fold_y {
                *y = fold_y - (*y - fold_y);
            }
        }
    }

    fn fold_left(&mut self, fold_x: usize) {
        for (x, _) in &mut self.inner {
            if *x > fold_x {
                *x = fold_x - (*x - fold_x);
            }
        }
    }

    fn sort(&mut self) {
        self.inner.sort_unstable();
        self.inner.dedup();
    }

    fn print(self) -> String {
        let max_x = self.inner.iter().map(|(x, _)| x).copied().max().unwrap();
        let max_y = self.inner.iter().map(|(_, y)| y).copied().max().unwrap();
        let set: FxHashSet<_> = self.inner.into_iter().collect();
        let mut buf = String::with_capacity(1 + (max_x + 1) * (max_y + 1) + max_y + 1);
        buf += "\n";
        for y in 0..=max_y {
            for x in 0..=max_x {
                buf += if set.contains(&(x, y)) {"#"} else {" "};
            }
            buf += "\n";
        }
        buf
    }
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Paper {
    let (coords, folds) = input.split("\n\n").collect_tuple().unwrap();
    let coords = coords.lines().map(|l| l.split(',').map(|s| s.parse().unwrap()).collect_tuple().unwrap()).collect::<Vec<_>>();
    let folds = folds.lines().map(|l| {
        let (dir, val) = l.split('=').collect_tuple().unwrap();
        (match &dir[(dir.len() - 1)..] {
            "y" => Fold::Up,
            "x" => Fold::Left,
            _ => unreachable!()
        })(val.parse().unwrap())
    }).collect();
    Paper {grid: Grid::parse(&coords), folds}
}

#[aoc(day13, part1)]
fn part1(input: &Paper) -> usize {
    let mut paper = input.clone();
    paper.grid.fold(&input.folds[0]);
    paper.grid.sort();
    paper.grid.inner.len()
}

#[aoc(day13, part2)]
fn part2(input: &Paper) -> String {
    let mut paper = input.clone();
    for fold in &paper.folds {
        paper.grid.fold(fold);
    }
    paper.grid.sort();
    paper.grid.print()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#;
        assert_eq!(part1(&parse(input)), 17);
    }
}
