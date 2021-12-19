use std::ops::{Add, Sub};
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

// Could be optimized

#[derive(Clone, Debug)]
struct Scanner {
    beacons: Vec<Pos>,
}

impl Scanner {
    pub fn rotate_and_make_relative(&mut self, p0: Pos, rotation: i32) {
        for pos in &mut self.beacons {
            *pos = pos.rotate(rotation) + p0;
        }
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl Pos {
    pub fn distance(&self, p0: Pos) -> i32 {
        (self.x - p0.x).abs() + (self.y - p0.y).abs() + (self.z - p0.z).abs()
    }
}

const ZERO: Pos = Pos { x: 0, y: 0, z: 0 };

impl From<(i32, i32, i32)> for Pos {
    fn from(t: (i32, i32, i32)) -> Self {
        Pos { x: t.0, y: t.1, z: t.2 }
    }
}

impl From<Vec<Pos>> for Scanner {
    fn from(beacons: Vec<Pos>) -> Self {
        Scanner { beacons }
    }
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Pos {
    pub fn rotate(&self, rotation: i32) -> Pos {
        let Self { x, y, z } = *self;
        match rotation {
            0 => Pos { x, y, z },
            1 => Pos { x: y, y: -x, z },
            2 => Pos { x: -x, y: -y, z },
            3 => Pos { x: -y, y: x, z },
            4 => Pos { x: z, y, z: -x },
            5 => Pos { x: y, y: -z, z: -x },
            6 => Pos { x: -z, y: -y, z: -x },
            7 => Pos { x: -y, y: z, z: -x },
            8 => Pos { x: z, y: -x, z: -y },
            9 => Pos { x: -x, y: -z, z: -y },
            10 => Pos { x: -z, y: x, z: -y },
            11 => Pos { x, y: z, z: -y },
            12 => Pos { x: z, y: -y, z: x },
            13 => Pos { x: -y, y: -z, z: x },
            14 => Pos { x: -z, y, z: x },
            15 => Pos { x: y, y: z, z: x },
            16 => Pos { x: z, y: x, z: y },
            17 => Pos { x, y: -z, z: y },
            18 => Pos { x: -z, y: -x, z: y },
            19 => Pos { x: -x, y: z, z: y },
            20 => Pos { x: -x, y, z: -z },
            21 => Pos { x: y, y: x, z: -z },
            22 => Pos { x, y: -y, z: -z },
            23 => Pos { x: -y, y: -x, z: -z },
            _ => unreachable!()
        }
    }
}

impl <'a> Sub for &'a Scanner {
    type Output = Option<(Pos, i32)>;

    fn sub(self, rhs: Self) -> Self::Output {
        for i in 0..24 {
            let counts = self.beacons.iter().copied().map(|p| p.rotate(i))
                .cartesian_product(rhs.beacons.iter().copied()).map(|(pos1, pos2)| pos2 - pos1).counts();
            for (k, v) in counts {
                if v >= 12 {
                    return Some((k, i));
                }
            }
        }
        None
    }
}

#[aoc_generator(day19)]
fn parse(input: &str) -> Vec<Scanner> {
    input.split("\n\n").into_iter()
        .map(|b| b.lines().skip(1).map(|s| s.split(',')
            .map(|s| s.parse().unwrap())
            .collect_tuple::<(i32, i32, i32)>().unwrap().into())
            .collect::<Vec<Pos>>().into())
        .collect()
}

#[aoc(day19, part1)]
fn part1(input: &[Scanner]) -> usize {
    let mut scanners = input.iter().cloned().collect_vec();
    let mut distances: FxHashMap<usize, FxHashMap<usize, (Pos, i32)>> = FxHashMap::default();
    for scanner in scanners.iter_mut().enumerate().skip(1) {
        for other in input.iter().enumerate() {
            if scanner.0 == other.0 {
                continue;
            }
            let dist = &*scanner.1 - other.1;
            if let Some(pos) = dist {
                distances.entry(scanner.0).or_default().insert(other.0, pos);
            }
        }
    }
    for scanner in scanners.iter_mut().enumerate().skip(1) {
        let mut progress = FxHashSet::default();
        *scanner.1 = rotate_to_zero(&distances, scanner.1.clone(), scanner.0, &mut progress).unwrap();
    }
    scanners.into_iter().flat_map(|s| s.beacons.into_iter()).unique().count()
}

#[aoc(day19, part2)]
fn part2(input: &[Scanner]) -> i32 {
    let mut scanners = input.iter().cloned().collect_vec();
    let mut distances: FxHashMap<usize, FxHashMap<usize, (Pos, i32)>> = FxHashMap::default();
    for scanner in scanners.iter_mut().enumerate().skip(1) {
        for other in input.iter().enumerate() {
            if scanner.0 == other.0 {
                continue;
            }
            let dist = &*scanner.1 - other.1;
            if let Some(pos) = dist {
                distances.entry(scanner.0).or_default().insert(other.0, pos);
            }
        }
    }
    for scanner in scanners.iter_mut().enumerate().skip(1) {
        let mut progress = FxHashSet::default();
        *scanner.1 = rotate_to_zero(&distances, scanner.1.clone(), scanner.0, &mut progress).unwrap();
    }

   scanners
        .into_iter()
        .enumerate()
        .skip(1)
        .map(|s| get_scanner_loc(&s.1, &input[s.0]).unwrap())
        .permutations(2)
        .map(|p| p[0].distance(p[1]))
        .max()
        .unwrap()
}

fn get_scanner_loc(scanner: &Scanner, input_scanner: &Scanner) -> Option<Pos> {
    for i in 0..24 {
        let d1 = ZERO - (input_scanner.beacons[0].rotate(i) - scanner.beacons[0]);
        let d2 = ZERO - (input_scanner.beacons[1].rotate(i) - scanner.beacons[1]);
        if d1 == d2 {
            return Some(d1);
        }
    }
    None
}


fn rotate_to_zero(distances: &FxHashMap<usize, FxHashMap<usize, (Pos, i32)>>, mut scanner: Scanner, i: usize, progress: &mut FxHashSet<usize>) -> Option<Scanner> {
    let links = &distances[&i];
    if links.contains_key(&0) {
        let dir = &links[&0];
        scanner.rotate_and_make_relative(dir.0, dir.1);
        Some(scanner)
    } else {
        for (k, v) in links {
            if !progress.contains(k) {
                progress.insert(*k);
                let mut scanner = scanner.clone();
                scanner.rotate_and_make_relative(v.0, v.1);
                if let Some(s) = rotate_to_zero(distances, scanner, *k, progress) {
                    return Some(s);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 79);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 3621);
    }

    static INPUT: &str = r#"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14"#;
}
