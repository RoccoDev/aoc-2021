use fxhash::FxHashSet;
use itertools::Itertools;

#[derive(Clone, Debug)]
struct Image {
    model: Vec<bool>,
    input: FxHashSet<(isize, isize)>,
    infinite_color: bool,

    min_x: isize,
    min_y: isize,
    max_x: isize,
    max_y: isize
}

impl Image {
    fn get_color(&self, pos: (isize, isize)) -> bool {
        if pos.0 < self.min_x || pos.1 < self.min_y || pos.0 > self.max_x || pos.1 > self.max_y {
            return self.infinite_color;
        }
        self.input.contains(&pos)
    }

    #[inline(always)]
    fn flip(&mut self) {
        let mut output = FxHashSet::default();

        for x in (self.min_x - 2)..(self.max_x + 2) {
            for y in (self.min_y - 2)..(self.max_y + 2) {
                let mut bits = 0usize;
                (-1..=1).cartesian_product(-1..=1).enumerate().map(|(i, (dy, dx))| (i, self.get_color((x + dx, y + dy))))
                    .filter(|(_, b)| *b).for_each(|(bit, _)| {
                    bits |= 1 << (8 - bit);
                });
                if self.model[bits] {
                    output.insert((x, y));
                }
            }
        }

        self.infinite_color = self.model[if self.infinite_color { self.model.len() - 1 } else { 0 }];
        self.input = output;

        let (min_x, max_x) = self.input.iter().map(|&(x, _)| x).minmax().into_option().unwrap();
        let (min_y, max_y) = self.input.iter().map(|&(_, y)| y).minmax().into_option().unwrap();
        self.min_x = min_x;
        self.max_x = max_x;
        self.min_y = min_y;
        self.max_y = max_y;
    }
}

#[aoc_generator(day20)]
fn parse(input: &str) -> Image {
    let (model, input) = input.split("\n\n").collect_tuple().unwrap();
    let model = model.chars().map(|c| c == '#').collect();
    let line_len = input.lines().next().unwrap().len();
    let max_y = input.lines().count() as isize - 1;
    let input = input.lines().flat_map(|s| s.chars().map(|c| c == '#')).enumerate().filter(|(_, b)| *b).map(|(i, _)| {
        let (x, y) = (i % line_len, i / line_len);
        (x as isize, y as isize)
    }).collect();
    Image {
        model, input, infinite_color: false, min_x: 0, min_y: 0, max_x: line_len as isize - 1, max_y
    }
}

#[aoc(day20, part1)]
fn part1(input: &Image) -> usize {
    let mut input = input.clone();
    input.flip();
    input.flip();
    input.input.len()
}

#[aoc(day20, part2)]
fn part2(input: &Image) -> usize {
    let mut input = input.clone();
    for _ in 0..50 {
        input.flip();
    }
    input.input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 35);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 3351);
    }
}
