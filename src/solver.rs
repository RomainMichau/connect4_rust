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

    return if maximizing_player {
        match scores.iter().enumerate().filter(|(_, x)| **x != MoveResult::ImpossibleMove.score()).max_by(|&(_, x), &(_, y)| x.cmp(y)) {
            None => {
                MiniMaxResult {
                    best_move_id: -1,
                    move_score: -1,
                    scores,
                }
            }
            Some((i, score)) => {
                MiniMaxResult {
                    best_move_id: i as i8,
                    move_score: *score,
                    scores,
                }
            }
        }
    } else {
        match scores.iter().enumerate().filter(|&(_, x)| *x != MoveResult::ImpossibleMove.score()).min_by(|&(_, x), &(_, y)| x.cmp(y)) {
            None => {
                MiniMaxResult {
                    best_move_id: -1,
                    move_score: -1,
                    scores,
                }
            }
            Some((i, score)) => {
                MiniMaxResult {
                    best_move_id: i as i8,
                    move_score: *score,
                    scores,
                }
            }
        }
    };
}