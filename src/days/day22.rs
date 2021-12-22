use std::ops::RangeInclusive;
use itertools::Itertools;
use regex::Regex;

struct Instruction {
    on: bool,
    range: Range3D,
}

enum Intersection {
    None(Range3D),
    Some([Option<Range3D>; 6]),
}

#[derive(Clone, Debug)]
struct Range3D {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>,
}

impl Range3D {
    pub fn intersect(&self, inst: &Range3D) -> Intersection {
        let (mut start_x, mut start_y, start_z) = self.start();
        let (mut end_x, mut end_y, end_z) = self.end();
        let (inst_start_x, inst_start_y, inst_start_z) = inst.start();
        let (inst_end_x, inst_end_y, inst_end_z) = inst.end();

        if (start_x <= inst_end_x && end_x >= inst_start_x) && (start_y <= inst_end_y && end_y >= inst_start_y) && (start_z <= inst_end_z && end_z >= inst_start_z) {
            let mut res = [None, None, None, None, None, None];
            if start_x < inst_start_x {
                res[0] = Range3D::new((start_x, inst_start_x - 1), (start_y, end_y), (start_z, end_z)).into();
                start_x = inst_start_x;
            }
            if end_x > inst_end_x {
                res[1] = Range3D::new((inst_end_x + 1, end_x), (start_y, end_y), (start_z, end_z)).into();
                end_x = inst_end_x;
            }
            if start_y < inst_start_y {
                res[2] = Range3D::new((start_x, end_x), (start_y, inst_start_y - 1), (start_z, end_z)).into();
                start_y = inst_start_y;
            }
            if end_y > inst_end_y {
                res[3] = Range3D::new((start_x, end_x), (inst_end_y + 1, end_y), (start_z, end_z)).into();
                end_y = inst_end_y;
            }
            if start_z < inst_start_z {
                res[4] = Range3D::new((start_x, end_x), (start_y, end_y), (start_z, inst_start_z - 1)).into();
            }
            if end_z > inst_end_z {
                res[5] = Range3D::new((start_x, end_x), (start_y, end_y), (inst_end_z + 1, end_z)).into();
            }
            Intersection::Some(res)
        } else {
            Intersection::None(Range3D {
                x: start_x..=end_x,
                y: start_y..=end_y,
                z: start_z..=end_z,
            })
        }
    }

    pub fn new(x: (i32, i32), y: (i32, i32), z: (i32, i32)) -> Range3D {
        Range3D {
            x: x.0..=x.1,
            y: y.0..=y.1,
            z: z.0..=z.1,
        }
    }

    fn start(&self) -> (i32, i32, i32) {
        (*self.x.start(), *self.y.start(), *self.z.start())
    }

    fn end(&self) -> (i32, i32, i32) {
        (*self.x.end(), *self.y.end(), *self.z.end())
    }

    pub fn len(&self) -> isize {
        ((self.x.end() + 1 - self.x.start()) as isize * (self.y.end() + 1 - self.y.start()) as isize * (self.z.end() + 1 - self.z.start()) as isize).abs()
    }
}

#[aoc_generator(day22)]
fn parse(input: &str) -> Vec<Instruction> {
    let regex = Regex::new(r#"(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)"#).unwrap();
    input.lines().map(|l| {
        let caps = regex.captures(l).unwrap();
        let on = caps[1] == *"on";
        let (x1, x2) = [&caps[2], &caps[3]].iter().map(|&s| s.parse::<i32>().unwrap()).collect_tuple().unwrap();
        let (y1, y2) = [&caps[4], &caps[5]].iter().map(|&s| s.parse::<i32>().unwrap()).collect_tuple().unwrap();
        let (z1, z2) = [&caps[6], &caps[7]].iter().map(|&s| s.parse::<i32>().unwrap()).collect_tuple().unwrap();
        Instruction {
            on,
            range: Range3D {
                x: x1.min(x2)..=x1.max(x2),
                y: y1.min(y2)..=y1.max(y2),
                z: z1.min(z2)..=z1.max(z2),
            },
        }
    }).collect()
}

#[aoc(day22, part1)]
fn part1(input: &[Instruction]) -> isize {
    let mut ranges: Vec<Range3D> = vec![];
    for inst in input {
        if (inst.range.x.start() * inst.range.y.start() * inst.range.z.start()).abs() > 50 * 50 * 50 {
            continue;
        }
        let mut tmp = vec![];
        for range in &ranges {
            match range.intersect(&inst.range) {
                Intersection::None(r) => tmp.push(r),
                Intersection::Some(ranges) => tmp.extend(ranges.into_iter().flatten())
            }
        }
        if inst.on {
            tmp.push(inst.range.clone());
        }
        ranges = tmp;
    }
    ranges.into_iter().map(|r| r.len()).sum()
}

#[aoc(day22, part2)]
fn part2(input: &[Instruction]) -> isize {
    // let mut ranges = vec![];
    let mut ranges: Vec<Range3D> = vec![];
    for inst in input {
        let mut tmp = vec![];
        for range in &ranges {
            match range.intersect(&inst.range) {
                Intersection::None(r) => tmp.push(r),
                Intersection::Some(ranges) => tmp.extend(ranges.into_iter().flatten())
            }
        }
        if inst.on {
            tmp.push(inst.range.clone());
        }
        ranges = tmp;
    }
    ranges.into_iter().map(|r| r.len()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10"#;
        assert_eq!(part1(&parse(input)), 39);
    }
}
