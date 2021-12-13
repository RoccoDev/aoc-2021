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
    inner: Vec<bool>,
    line_len: usize
}

impl Grid {
    fn parse(points: &[(usize, usize)]) -> Self {
        let line_len = points.iter().map(|(x, _)| x).copied().max().unwrap() as usize + 1;
        let max_y = points.iter().map(|(_, y)| y).copied().max().unwrap() as usize;
        let mut grid = vec![false; line_len * (max_y + 1)];

        for point in points {
            grid[point.1 * line_len + point.0] = true;
        }

        Grid {
            inner: grid,
            line_len
        }
    }

    fn fold(&mut self, fold: &Fold) {
        match fold {
            Fold::Up(v) => self.fold_up(*v),
            Fold::Left(v) => self.fold_left(*v)
        }
    }

    fn fold_up(&mut self, y: usize) {
        let y_start = y * self.line_len;
        let removed = self.inner.split_off(y_start);
        for (i, val) in removed.into_iter().skip(self.line_len).enumerate() {
            let index = (i % self.line_len) + self.line_len * (y - 1 - (i / self.line_len));
            self.inner[index] = self.inner[index] || val;
        }
    }

    fn fold_left(&mut self, x: usize) {
        for i in (0..self.inner.len())
            .filter(|i| i % self.line_len >= x)
            .rev() {
            let val = self.inner.remove(i);
            let col = i % self.line_len;
            let row_0 = i - col;
            let index = row_0 + self.line_len - 1 - col;
            if index == self.inner.len() {
                continue;
            }
            self.inner[index] = self.inner[index] || val;
        }
        self.line_len -= x + 1;
    }

    fn print(&self) {
        let mut index = 0;
        loop {
            for x in 0..self.line_len {
                let ind = index + x;
                if ind >= self.inner.len() {
                    return;
                }
                print!("{}", if self.inner[ind] {'#'} else {'.'});
            }
            println!();
            index += self.line_len;
        }
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
    paper.grid.inner.into_iter().filter(|&b| b).count()
}

#[aoc(day13, part2)]
fn part2(input: &Paper) -> usize {
    let mut paper = input.clone();
    for fold in &paper.folds {
        paper.grid.fold(fold);
    }
    paper.grid.print();
    0
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