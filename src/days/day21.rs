use fxhash::FxHashMap;
use itertools::Itertools;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct GameState {
    // current, opponent
    pos: (usize, usize),
    score: (usize, usize),
}

impl Die {
    pub fn generate_tuple(&mut self, max: u8) -> (u8, u8, u8) {
        (0..3).map(|_| self.gen(max)).collect_tuple().unwrap()
    }

    fn gen(&mut self, max: u8) -> u8 {
        self.0 += 1;
        if self.0 > max {
            self.0 = 1;
        }
        self.0
    }
}

#[aoc_generator(day21)]
fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|s| s.split(": ").last().unwrap().parse().unwrap()).collect()
}

#[aoc(day21, part1)]
fn part1(input: &[usize]) -> usize {
    let mut positions = input.to_owned();
    let mut scores = [0, 0];
    let mut player = 0;
    let mut turn = 0;
    for (i1, i2, i3) in (1..=100).cycle().tuples() {
        turn += 1;
        positions[player] += i1 + i2 + i3;
        if positions[player] > 10 {
            if positions[player] % 10 == 0 {
                positions[player] = 10;
            } else {
                positions[player] -= 10 * (positions[player] / 10);
            }
        }
        scores[player] += positions[player];
        if scores[player] >= 1000 {
            return turn * 3 * scores[1 - player] as usize;
        }
        player = 1 - player;
    }
    0
}

#[aoc(day21, part2)]
fn part2(input: &[usize]) -> usize {
    let positions = [input[0], input[1]];
    let scores = [0, 0];

    // Stores win data for known paths
    let mut cache: FxHashMap<GameState, [usize; 2]> = FxHashMap::default();

    let winners = run_game(scores, positions, &mut cache);
    *winners.iter().max().unwrap()
}

fn run_game(scores: [usize; 2], positions: [usize; 2], cache: &mut FxHashMap<GameState, [usize; 2]>) -> [usize; 2] {
    if scores[0] >= 21 {
        return [1, 0]; // P0 won
    } else if scores[1] >= 21 {
        return [0, 1]; // P1 won
    }
    let game_state = GameState {
        pos: (positions[0], positions[1]),
        score: (scores[0], scores[1])
    };
    if let Some(winner) = cache.get(&game_state) {
        return *winner;
    }
    let mut winners = [0usize; 2];
    for roll in (1..=3).cartesian_product(1..=3).cartesian_product(1..=3).map(|((a, b), c)| a + b + c) {
        let mut positions = positions;
        let mut scores = scores;
        let pos = &mut positions[0];
        *pos += roll;
        if *pos > 10 {
            if *pos % 10 == 0 {
                *pos = 10;
            } else {
                *pos -= 10 * (*pos / 10);
            }
        }
        scores[0] += *pos;
        let losing_pos = {
            let mut pos = positions;
            pos.reverse();
            pos
        };
        let losing_score = {
            let mut score = scores;
            score.reverse();
            score
        };
        let losing_game_result = run_game(losing_score, losing_pos, cache);
        winners[0] += losing_game_result[1];
        winners[1] += losing_game_result[0];
    }
    cache.insert(game_state, winners);
    winners
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"Player 1 starting position: 4
Player 2 starting position: 8"#;
        assert_eq!(part1(&parse(input)), 739785);
    }

    #[test]
    fn part2_example() {
        let input = r#"Player 1 starting position: 4
Player 2 starting position: 8"#;
        assert_eq!(part2(&parse(input)), 444356092776315);
    }
}
