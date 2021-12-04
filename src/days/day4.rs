use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::LinkedList;
use std::io::BufRead;
use std::rc::Rc;
use fxhash::FxHashMap;

struct Game {
    number_map: FxHashMap<u8, Vec<Rc<RefCell<Board>>>>,
    rolls: Vec<u8>,
}

struct Board {
    inner: Vec<BoardNumber>,
    has_won: bool,
    row_length: usize,
}

#[derive(Copy, Clone)]
enum BoardNumber {
    Marked(u8),
    Unmarked(u8),
}

impl From<u8> for BoardNumber {
    fn from(num: u8) -> Self {
        BoardNumber::Unmarked(num)
    }
}

impl From<BoardNumber> for u8 {
    fn from(board: BoardNumber) -> Self {
        match board {
            BoardNumber::Marked(v) | BoardNumber::Unmarked(v) => v
        }
    }
}

impl<S: AsRef<str>> From<S> for Board {
    fn from(string: S) -> Self {
        let len = string.as_ref().lines().next().unwrap().split_whitespace().count();
        let inner = string.as_ref()
            .lines()
            .flat_map(|s| s.split_whitespace())
            .map(|s| BoardNumber::from(s.parse::<u8>().unwrap()))
            .collect();
        Board {
            inner,
            has_won: false,
            row_length: len,
        }
    }
}

impl BoardNumber {
    pub fn is_marked(&self) -> bool {
        match self {
            BoardNumber::Marked(_) => true,
            BoardNumber::Unmarked(_) => false
        }
    }
}

impl Board {
    pub fn mark(&mut self, num: u8) -> bool {
        let mut index = 0;
        for (i, old) in self.inner.iter_mut().enumerate() {
            let val: u8 = (*old).into();
            if val == num {
                *old = BoardNumber::Marked(num);
                index = i;
                break;
            }
        };
        let row = index / self.row_length;
        let col = index % self.row_length;

        if self.check_col(col) || self.check_row(row) {
            self.has_won = true;
            true
        } else {
            false
        }
    }

    fn check_col(&self, col: usize) -> bool {
        self.inner.iter().skip(col).step_by(self.row_length).all(|b| b.is_marked())
    }

    fn check_row(&self, row: usize) -> bool {
        self.inner.iter().skip(row * self.row_length).take(self.row_length).all(|b| b.is_marked())
    }
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Game {
    let mut split = input.split("\n\n");

    let rolls = split.next().unwrap().split(',').map(|s| s.parse().unwrap()).collect();
    let boards: Vec<_> = split.map(|s| Rc::new(RefCell::new(Board::from(s)))).collect();

    let mut map = FxHashMap::default();
    for board in boards {
        for num in &(*(*board).borrow()).inner {
            let num = (*num).into();
            map.entry(num).or_insert_with(|| Vec::with_capacity(5 * 5)).push(Rc::clone(&board));
        }
    }

    Game { rolls, number_map: map }
}

#[aoc(day4, part1)]
fn part1(input: &Game) -> u32 {
    for num in &input.rolls {
        let boards = &input.number_map[num];
        for board in boards {
            if board.borrow_mut().mark(*num) {
                let unmarked: u32 = sum_unmarked(&(**board).borrow());
                return unmarked * (*num as u32);
            }
        }
    }
    unreachable!()
}

#[aoc(day4, part2)]
fn part2(input: &Game) -> u32 {
    let mut last_score = 0;
    for num in &input.rolls {
        let boards = &input.number_map[num];
        for board in boards {
            if (**board).borrow().has_won {
                continue;
            }

            if board.borrow_mut().mark(*num) {
                let unmarked: u32 = sum_unmarked(&(**board).borrow());
                last_score = unmarked * (*num as u32);
            }
        }
    }
    last_score
}

#[inline(always)]
fn sum_unmarked(board: &Board) -> u32 {
    board.inner.iter()
        .filter(|b| !b.is_marked())
        .map(|num| {
            let num: u8 = (*num).into();
            num as u32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;
        assert_eq!(part1(&parse(input)), 4512);
    }

    #[test]
    fn part2_example() {
        let input = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;
        assert_eq!(part2(&parse(input)), 1924);
    }
}