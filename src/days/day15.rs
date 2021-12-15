use fxhash::FxHashMap;
use itertools::Itertools;
use pathfinding::prelude::dijkstra;

#[derive(Debug, Clone)]
struct Grid {
    nodes: FxHashMap<(i32, i32), Node>,
    start: Node,
    end: Node
}

#[derive(Copy, Clone, Eq, Ord, PartialOrd, PartialEq, Hash, Debug, Default)]
struct Node {
    pos: (i32, i32),
    cost: i32
}

impl Grid {
    pub fn parse(nodes: Vec<Node>) -> Grid {
        let start = nodes[0];
        let end = nodes[nodes.len() - 1];
        let nodes = nodes.into_iter().map(|n| (n.pos, n)).collect();
        Grid {
            start, end, nodes
        }
    }

    pub fn successors(&self, node: &Node) -> [(Node, i32); 4] {
        let def = (*node, node.cost);
        let mut res = [def, def, def, def];
        let (node_x, node_y) = node.pos;
        for (i, node) in (-1..=1).cartesian_product(-1..=1).filter(|(x, y)| (*x == 0) ^ (*y == 0))
            .filter_map(|(x, y)| self.nodes.get(&(node_x + x, node_y + y)))
            .enumerate() {
            res[i] = (*node, node.cost);
        }
        res
    }
}

#[aoc_generator(day15)]
fn parse(input: &str) -> Grid {
    let nodes = input.lines().enumerate().flat_map(|(y, l)| l.chars().enumerate().map(move |(x, c)| Node {
        pos: (x as i32, y as i32),
        cost: c.to_digit(10).unwrap() as i32
    })).collect();
    Grid::parse(nodes)
}

#[aoc(day15, part1)]
fn part1(input: &Grid) -> i32 {
    let (_, cost) = dijkstra(&input.start, |n| input.successors(n), |n| *n == input.end).unwrap();
    cost
}

#[aoc(day15, part2)]
fn part2(input: &Grid) -> i32 {
    let mut extended = input.clone();
    let (w, l) = (input.end.pos.0 + 1, input.end.pos.1 + 1);
    extended.nodes.reserve(w as usize * 5 + l as usize * 5);
    for y in 0..5 {
        for x in 0..5 {
            if x == y && x == 0 {
                continue;
            }
            for ((old_x, old_y), node) in &input.nodes {
                let mut node = *node;
                let new_pos = (w * x + *old_x, l * y + *old_y);
                node.pos = new_pos;
                node.cost += x + y;
                if node.cost > 9 {
                    node.cost = (node.cost - 9).abs();
                }
                extended.nodes.insert(new_pos, node);
            }
        }
    }
    extended.end = extended.nodes[&(w * 5 - 1, l * 5 - 1)];
    let (_, cost) = dijkstra(&extended.start, |n| extended.successors(n), |n| *n == extended.end).unwrap();
    cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"#;
        assert_eq!(part1(&parse(input)), 40);
    }

    #[test]
    fn part2_example() {
        let input = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"#;
        assert_eq!(part2(&parse(input)), 315);
    }
}
