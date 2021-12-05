use fxhash::{FxHashMap, FxHashSet};
use regex::Regex;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug)]
struct Line {
    start: Point,
    end: Point,
    dx: i32,
    dy: i32,
    slope: (i32, i32),
    length_sq: i32
}

impl Line {
    /// Uses vectors to calculate whether the line points and the given points are colinear - slower but low memory footprint
    fn vector_intersection(&self, line: &Line, full: bool, intersections: &mut FxHashSet<Point>) {
        if std::ptr::eq(line, self) {
            return;
        }

        if !full {
            if self.start.x != self.end.x && self.start.y != self.end.y {
                return;
            }
            if line.start.x != line.end.x && line.start.y != line.end.y {
                return;
            }
        }

        let mut point = Point { x: line.start.x, y: line.start.y };

        loop {
            if self.contains_vectored(&point) {
                intersections.insert(point);
            }
            if point.x == line.end.x && point.y == line.end.y {
                break;
            }
            point.x += line.slope.0;
            point.y += line.slope.1;
        }
    }

    /// Gets the list of points covered by this line to calculate an intersection - faster but requires more memory
    fn get_points(&self, no_diagonals: bool) -> Option<Vec<Point>> {
        if no_diagonals {
            if self.start.x != self.end.x && self.start.y != self.end.y {
                return None;
            }
        }

        let mut points = Vec::with_capacity((self.dx + self.dy).abs() as usize);
        let mut point = Point { x: self.start.x, y: self.start.y };

        loop {
            points.push(point);
            if point.x == self.end.x && point.y == self.end.y {
                break;
            }
            point.x += self.slope.0;
            point.y += self.slope.1;
        }

        Some(points)
    }

    fn contains_vectored(&self, point: &Point) -> bool {
        let point = *point;
        if (point == self.start) || (point == self.end) {
            return true;
        }
        let cross_product = self.dx * (point.y - self.start.y) - (point.x - self.start.x) * self.dy;
        if cross_product != 0 {
            return false;
        }
        let dot_product = (point.x - self.start.x) * self.dx + (point.y - self.start.y) * self.dy;
        if dot_product < 0 {
            return false;
        }
        dot_product <= self.length_sq
    }
}

impl From<((i32, i32), (i32, i32))> for Line {
    fn from(points: ((i32, i32), (i32, i32))) -> Self {
        let start = Point {
            x: points.0.0,
            y: points.0.1,
        };
        let end = Point {
            x: points.1.0,
            y: points.1.1,
        };

        let len = (end.x - start.x).pow(2) + (end.y - start.y).pow(2);

        let dx = end.x - start.x;
        let dy = end.y - start.y;

        let slope = (if dx == 0 { 0 } else if dx < 0 { -1 } else { 1 }, if dy == 0 { 0 } else if dy < 0 { -1 } else { 1 });

        Line { start, end, slope, dx, dy, length_sq: len }
    }
}


#[aoc_generator(day5)]
fn parse(input: &str) -> Vec<Line> {
    let regex = Regex::new(r#"(\d+),(\d+) -> (\d+),(\d+)"#).unwrap();
    input.lines().map(|l| {
        let caps = regex.captures(l).unwrap();
        ((caps[1].parse().unwrap(), caps[2].parse().unwrap()), (caps[3].parse().unwrap(), caps[4].parse().unwrap())).into()
    }).collect()
}

#[aoc(day5, part1, vectors)]
fn part1_vectors(input: &[Line]) -> usize {
    let mut intersections = FxHashSet::default();
    for line in input {
        for other in input {
            line.vector_intersection(other, false, &mut intersections);
        }
    }
    intersections.len()
}

#[aoc(day5, part1, points)]
fn part1_points(input: &[Line]) -> usize {
    let mut intersections: FxHashMap<Point, u8> = FxHashMap::default();
    for line in input {
        for point in line.get_points(true).into_iter().flatten() {
            *intersections.entry(point).or_insert(0) += 1;
        }
    }
    intersections.into_iter().filter(|(_, v)| *v > 1).count()
}

#[aoc(day5, part2, points)]
fn part2_points(input: &[Line]) -> usize {
    let mut intersections: FxHashMap<Point, u8> = FxHashMap::default();
    for line in input {
        for point in line.get_points(false).into_iter().flatten() {
            *intersections.entry(point).or_insert(0) += 1;
        }
    }
    intersections.into_iter().filter(|(_, v)| *v > 1).count()
}

#[aoc(day5, part2, vectors)]
fn part2_vectors(input: &[Line]) -> usize {
    let mut intersections = FxHashSet::default();
    for line in input {
        for other in input {
            line.vector_intersection(other, true, &mut intersections);
        }
    }
    intersections.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;
        assert_eq!(part1_vectors(&parse(input)), 5);
        assert_eq!(part1_points(&parse(input)), 5);
    }

    #[test]
    fn part2_example() {
        let input = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;
        assert_eq!(part2_vectors(&parse(input)), 12);
        assert_eq!(part2_points(&parse(input)), 12);
    }
}
