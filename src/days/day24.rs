use std::collections::VecDeque;
use itertools::Itertools;

#[derive(Eq, PartialEq, Copy, Clone)]
enum Instruction {
    Input,
    Add(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    Div(char, Value),
    Eql(char, Value)
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Value {
    Literal(isize),
    Ref(char)
}

impl Value {
    fn unwrap(&self) -> isize {
        match self {
            Value::Literal(v) => *v,
            Value::Ref(_) => panic!()
        }
    }
}

struct Subroutine {
    instructions: Vec<Instruction>
}

struct Program {
    subroutines: Vec<Subroutine>
}

impl <'a> From<&'a str> for Value {
    fn from(s: &'a str) -> Self {
        let first_char = s.chars().next().unwrap();
        if first_char == '-' || first_char.is_ascii_digit() {
            Value::Literal(s.parse().unwrap())
        } else {
            Value::Ref(first_char)
        }
    }
}

#[aoc_generator(day24)]
fn parse(input: &str) -> Program {
    let instructions = input.lines().map(|l| {
        let mut split = l.split_whitespace();
        let first = split.next().unwrap();
        let second = split.next().unwrap();
        let third = split.next();
        match first {
            "inp" => Instruction::Input,
            "add" => Instruction::Add(second.chars().next().unwrap(), third.unwrap().into()),
            "mul" => Instruction::Mul(second.chars().next().unwrap(), third.unwrap().into()),
            "eql" => Instruction::Eql(second.chars().next().unwrap(), third.unwrap().into()),
            "div" => Instruction::Div(second.chars().next().unwrap(), third.unwrap().into()),
            "mod" => Instruction::Mod(second.chars().next().unwrap(), third.unwrap().into()),
            _ => unreachable!()
        }
    }).collect_vec();
    let subroutines = instructions.split(|&i| i == Instruction::Input).map(|i| Subroutine {
        instructions: i.iter().copied().collect_vec()
    }).collect_vec();
    Program {subroutines}
}

#[aoc(day24, part1)]
fn part1(input: &Program) -> usize {
    let mut res = [0; 14];
    let mut buf: VecDeque<(isize, isize)> = VecDeque::new();
    for (i, sub) in input.subroutines.iter().skip(1).enumerate() {
        if sub.instructions.contains(&Instruction::Div('z', Value::Literal(26))) {
            let offset = sub.instructions.iter().filter_map(|&i| match i {
                Instruction::Add('x', v) => Some(v),
                _ => None
            }).last().unwrap().unwrap();
            let (last_index, last_offset) = buf.pop_front().unwrap();
            let difference = offset + last_offset;
            if difference >= 0 {
                res[last_index as usize] = 9 - difference;
                res[i] = 9;
            } else {
                res[last_index as usize] = 9;
                res[i] = 9 + difference;
            }
        } else {
            let last_y_pos = sub.instructions.iter().filter_map(|&i| match i {
                Instruction::Add('y', v) => Some(v),
                _ => None
            }).last().unwrap().unwrap();
            buf.push_front((i as isize, last_y_pos as isize));
        }
    }
    res.into_iter().fold(0usize, |acc, x| acc * 10 + x as usize)
}

#[aoc(day24, part2)]
fn part2(input: &Program) -> usize {
    let mut res = [0; 14];
    let mut buf: VecDeque<(isize, isize)> = VecDeque::new();
    for (i, sub) in input.subroutines.iter().skip(1).enumerate() {
        if sub.instructions.contains(&Instruction::Div('z', Value::Literal(26))) {
            let offset = sub.instructions.iter().filter_map(|&i| match i {
                Instruction::Add('x', v) => Some(v),
                _ => None
            }).last().unwrap().unwrap();
            let (last_index, last_offset) = buf.pop_front().unwrap();
            let difference = offset + last_offset;
            if difference >= 0 {
                res[last_index as usize] = 1;
                res[i] = 1 + difference;
            } else {
                res[last_index as usize] = 1 - difference;
                res[i] = 1;
            }
        } else {
            let last_y_pos = sub.instructions.iter().filter_map(|&i| match i {
                Instruction::Add('y', v) => Some(v),
                _ => None
            }).last().unwrap().unwrap();
            buf.push_front((i as isize, last_y_pos as isize));
        }
    }
    res.into_iter().fold(0usize, |acc, x| acc * 10 + x as usize)
}