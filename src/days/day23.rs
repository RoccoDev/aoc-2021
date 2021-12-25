use itertools::Itertools;
use pathfinding::prelude::dijkstra;

#[allow(clippy::ptr_arg)]
fn calc_moves(grid: &Vec<Vec<u8>>) -> Vec<(Vec<Vec<u8>>, usize)> {
    let room_y_len = grid.len() - 2;
    let mut moves = Vec::with_capacity(25);
    let hallway = &grid[1];
    for x in 0..hallway.len() {
        let (destination_x, cost) = match hallway[x] {
            b'A' => (3, 1),
            b'B' => (5, 10),
            b'C' => (7, 100),
            b'D' => (9, 1000),
            _ => continue,
        };
        let (r0, r1) = if x > destination_x { (destination_x, x) } else { (x + 1, destination_x + 1) };
        if (r0..r1).any(|i| hallway[i] != b'.') {
            // Check for obstacles in the path
            continue;
        }
        let destination_y = match (2..=room_y_len).take_while(|&i| grid[i][destination_x] == b'.').last() {
            Some(i) => i,
            _ => continue
        };
        if destination_y != room_y_len && (3..=room_y_len).any(|i| {
            let room = grid[i][destination_x];
            room != b'.' && room != hallway[x]
        }) {
            // We can't move into rooms that are either full or occupied by at least a different amphipod
            continue;
        }
        let mut next_grid = grid.to_owned();
        next_grid[destination_y][destination_x] = hallway[x];
        next_grid[1][x] = b'.';
        moves.push((next_grid, (r1 - r0 + destination_y - 1) * cost));
    }
    for (y, x) in (2..=room_y_len).cartesian_product([3, 5, 7, 9]) {
        let cost = match grid[y][x] {
            b'A' => 1,
            b'B' => 10,
            b'C' => 100,
            b'D' => 1000,
            _ => continue,
        };
        if (2..y).any(|i| grid[i][x] != b'.') || (y + 1..=room_y_len).any(|i| grid[i][x] == b'.') {
            continue;
        }
        for (i, space) in hallway.iter().enumerate().skip(x) {
            if *space != b'.' { break; }
            if ![1, 2, 4, 6, 8, 10, 11].contains(&i) { continue; }
            let mut next_grid = grid.to_owned();
            next_grid[1][i] = grid[y][x];
            next_grid[y][x] = b'.';
            moves.push((next_grid, (y - 1 + i - x) * cost));
        }
        for i in (1..=x).rev() {
            if hallway[i] != b'.' { break; }
            if ![1, 2, 4, 6, 8, 10, 11].contains(&i) { continue; }
            let mut next_grid = grid.to_owned();
            next_grid[1][i] = grid[y][x];
            next_grid[y][x] = b'.';
            moves.push((next_grid, (y - 1 + x - i) * cost));
        }
    }
    moves
}

#[allow(clippy::ptr_arg)]
fn find_path(maze: &Vec<Vec<u8>>) -> usize {
    dijkstra(maze,
             calc_moves,
             |m| {
                 m[2..=(m.len() - 2)].iter().all(|l| itertools::equal(l[3..=9].iter().copied(), "A#B#C#D".bytes()))
             }).unwrap().1
}

#[aoc_generator(day23)]
fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.bytes().collect()).collect()
}

#[aoc(day23, part1)]
#[allow(clippy::ptr_arg)]
fn part1(input: &Vec<Vec<u8>>) -> usize {
    find_path(input)
}

#[aoc(day23, part2)]
#[allow(clippy::ptr_arg)]
fn part2(input: &[Vec<u8>]) -> usize {
    let mut input = input.to_owned();
    input.insert(3, "  #D#B#A#C#".bytes().collect());
    input.insert(3, "  #D#C#B#A#".bytes().collect());
    find_path(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########"#;
        assert_eq!(part1(&parse(input)), 12521);
    }

    #[test]
    fn part2_example() {
        let input = r#"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########"#;
        assert_eq!(part2(&parse(input)), 44169);
    }
}
