use fxhash::FxHashMap;
use crate::days::day25::Cucumber::{East, South};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Cucumber {
    East,
    South,
    None
}

#[derive(Clone)]
struct Grid {
    positions: FxHashMap<(i32, i32), Cucumber>,
    len: (i32, i32)
}

impl Grid {
    pub fn from(raw: Vec<Vec<Cucumber>>) -> Self {
        let y_len = raw.len() as i32;
        let x_len = raw[0].len() as i32;
        let positions: FxHashMap<_, _> = raw.into_iter().enumerate()
            .flat_map(|(y, v)| v.into_iter().enumerate().filter_map(move |(x, c)| match c {
                Cucumber::None => None,
                c => Some(((x as i32, y as i32), c))
            })).collect();
        Grid {
            positions,
            len: (x_len, y_len)
        }
    }

    fn move_cucs(&mut self) -> usize {
        let mut moved = 0;
        for ty in ORDER {
            let check_for_adj = self.clone();
            for cuc in check_for_adj.positions.iter().
                filter_map(|(pos, c)| if *c == ty { Some(*pos) } else { None }) {
                let next_pos = match ty {
                    East => ((cuc.0 + 1) % self.len.0, cuc.1),
                    South => (cuc.0, (cuc.1 + 1) % self.len.1),
                    Cucumber::None => continue
                };
                if !check_for_adj.positions.contains_key(&next_pos) {
                    // move
                    self.positions.remove(&cuc);
                    self.positions.insert(next_pos, ty);
                    moved += 1;
                }
            }
        }
        moved
    }
}

static ORDER: [Cucumber; 2] = [East, South];

#[aoc_generator(day25)]
fn parse(input: &str) -> Grid {
    Grid::from(input.lines().map(|s| s.chars().map(|c| match c {
        '>' => East,
        'v' => South,
        '.' => Cucumber::None,
        _ => unreachable!()
    }).collect()).collect())
}

#[aoc(day25, part1)]
fn part1(input: &Grid) -> usize {
    let mut input = input.clone();
    for i in 1.. {
        let moved = input.move_cucs();
        if moved == 0 {
            return i;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"#;
        assert_eq!(part1(&parse(input)), 58);
    }
}
