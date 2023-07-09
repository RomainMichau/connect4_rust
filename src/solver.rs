use rand::Rng;
use crate::game::game::{Cell, Game};

enum MoveResult {
    ImpossibleMove,
    Lost,
    Neutral,
    Win,
}

impl MoveResult {
    fn score(&self) -> i8 {
        match self {
            MoveResult::ImpossibleMove => { -1 }
            MoveResult::Lost => { 0 }
            MoveResult::Neutral => { 1 }
            MoveResult::Win => { 2 }
        }
    }
}

pub struct MiniMaxResult {
    pub best_move_id: i8,
    pub move_score: i8,
    pub scores: Vec<i8>,
}

pub fn mini_max(game: &mut Game, depth: u8, maximizing_player: bool) -> MiniMaxResult {
    let mut scores = vec![MoveResult::ImpossibleMove.score(); game.grid[0].len()];
    for col in 0..game.grid[0].len() {
        if game.can_add_token(col) {
            let line = game.insert_token(col).unwrap();
            if game.check_win(col, line) {
                if maximizing_player {
                    // println!("{}", game);
                    scores[col] = MoveResult::Win.score()
                } else {
                    scores[col] = MoveResult::Lost.score()
                }
            } else if depth > 0 {
                scores[col] = mini_max(game, depth - 1, !maximizing_player).move_score;
            } else {
                scores[col] = MoveResult::Neutral.score();
            }
            game.grid[line][col] = Cell::Empty;
            game.next_player();
        } else {
            scores[col] = MoveResult::ImpossibleMove.score()
        }
    }


    match if maximizing_player {
        scores.iter().cloned().filter(|&x| x != MoveResult::ImpossibleMove.score()).max()
    } else {
        scores.iter().cloned().filter(|&x| x != MoveResult::ImpossibleMove.score()).min()
    } {
        Some(best_value) => {
            let mut rng = rand::thread_rng();

            let indexes: Vec<usize> = scores.iter()
                .enumerate()
                .filter(|&(_, &value)| value == best_value)
                .map(|(index, _)| index)
                .collect();

            let rnd = rng.gen_range(0..indexes.len());
            let random_index = indexes[rnd];
            MiniMaxResult {
                best_move_id: random_index as i8,
                move_score: best_value,
                scores,
            }
        }
        None => {
            MiniMaxResult {
                best_move_id: -1,
                move_score: -1,
                scores,
            }
        }
    }
}
