use std::borrow::Borrow;

#[derive(Default)]
struct Submarine {
    pos: i32,
    /// Depth in part 1, aim in part 2
    depth_aim: i32,
    /// Depth for part 2, because the old field is used for aim
    new_depth: i32,
}

enum Instruction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl Instruction {
    fn apply(&self, submarine: &mut Submarine, advanced: bool) {
        match self {
            Instruction::Forward(val) => {
                submarine.pos += *val;
                if advanced {
                    submarine.new_depth += submarine.depth_aim * val;
                }
            }
            Instruction::Up(val) => submarine.depth_aim -= *val,
            Instruction::Down(val) => submarine.depth_aim += *val,
        }
    }
}

impl<I: Iterator<Item = impl Borrow<str>>> From<I> for Instruction {
    fn from(mut iter: I) -> Self {
        let op = iter.next().unwrap();
        let op = op.borrow();
        let val = iter.next().unwrap();
        let val = val.borrow().parse::<i32>().unwrap();
        (match op {
            "forward" => Instruction::Forward,
            "up" => Instruction::Up,
            "down" => Instruction::Down,
            s => panic!("unknown instruction {}", s),
        })(val)
    }
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| l.split(' ').into()).collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Instruction]) -> i32 {
    let mut submarine = Submarine::default();
    for inst in input {
        inst.apply(&mut submarine, false);
    }
    submarine.pos * submarine.depth_aim
}

#[aoc(day2, part2)]
fn part2(input: &[Instruction]) -> i32 {
    let mut submarine = Submarine::default();
    for inst in input {
        inst.apply(&mut submarine, true);
    }
    submarine.pos * submarine.new_depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;
        assert_eq!(part1(&parse(input)), 150);
    }

    #[test]
    fn part2_example() {
        let input = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;
        assert_eq!(part2(&parse(input)), 900);
    }
}
