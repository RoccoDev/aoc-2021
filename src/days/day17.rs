use std::cmp::Ordering;
use std::ops::RangeInclusive;
use regex::Regex;

#[aoc_generator(day17)]
fn parse(input: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let regex = Regex::new(r#"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)"#).unwrap();
    let caps = regex.captures(input).unwrap();
    let (x1, x2) = (caps[1].parse::<i32>().unwrap(), caps[2].parse::<i32>().unwrap());
    let (y1, y2) = (caps[3].parse::<i32>().unwrap(), caps[4].parse::<i32>().unwrap());
    (x1.min(x2)..=x1.max(x2), y1.min(y2)..=y1.max(y2))
}

#[aoc(day17, part1)]
fn part1((target_x, target_y): &(RangeInclusive<i32>, RangeInclusive<i32>)) -> i32 {
    let mut max = 0;
    let x_end = *target_x.end();

    // upper y bound is x_end because once it reaches the peak it needs at least that many steps to enter the target zone
    // same for the lower bound, the y initial velocity influences the steps required
    for vy in *target_y.start()..x_end {
        for vx in 0..=x_end {
            let mut pos = (0, 0);
            let mut vel = (vx, vy);
            let mut max_y = None;
            loop {
                pos.0 += vel.0;
                if vel.1 == 0 {
                    max_y = Some(pos.1);
                }
                pos.1 += vel.1;
                vel.0 = match vel.0.cmp(&0) {
                    Ordering::Less => vel.0 + 1,
                    Ordering::Equal => vel.0,
                    Ordering::Greater => vel.0 - 1
                };
                vel.1 -= 1;

                if pos.0 > *target_x.end() || pos.1 < *target_y.start() {
                    break;
                }

                if target_x.contains(&pos.0) && target_y.contains(&pos.1) {
                    if let Some(max_y) = max_y {
                        if max_y > max {
                            max = max_y;
                        }
                    }
                    break;
                }
            }
        }
    }
    max
}

#[aoc(day17, part2)]
fn part2((target_x, target_y): &(RangeInclusive<i32>, RangeInclusive<i32>)) -> usize {
    let mut count = 0;
    let x_end = *target_x.end();

    for vy in *target_y.start()..x_end {
        for vx in 0..=x_end {
            let mut pos = (0, 0);
            let mut vel = (vx, vy);
            loop {
                pos.0 += vel.0;
                pos.1 += vel.1;
                vel.0 = match vel.0.cmp(&0) {
                    Ordering::Less => vel.0 + 1,
                    Ordering::Equal => vel.0,
                    Ordering::Greater => vel.0 - 1
                };
                vel.1 -= 1;

                if pos.0 > *target_x.end() || pos.1 < *target_y.start() {
                    break;
                }

                if target_x.contains(&pos.0) && target_y.contains(&pos.1) {
                    count += 1;
                    break;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"target area: x=20..30, y=-10..-5"#;
        assert_eq!(part1(&parse(input)), 45);
    }

    #[test]
    fn part2_example() {
        let input = r#"target area: x=20..30, y=-10..-5"#;
        assert_eq!(part2(&parse(input)), 112);
    }
}
