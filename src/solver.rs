use rand::Rng;
use crate::game::game::{Cell, Game};

#[derive(Clone)]
pub enum MoveResult {
    // Depth of the lost
    Lost(u8),
    Neutral,
    // Depth of the win
    Win(u8),
}

impl MoveResult {
    fn score(&self, depth: u8) -> i8 {
        match self {
            MoveResult::Lost(x) => { (depth as i8 + 1) * -1 + *x as i8 }
            MoveResult::Neutral => { 0 }
            MoveResult::Win(x) => { (depth as i8 + 1) - *x as i8 }
        }
    }

    /// Get the score to display
    /// Win = 1 * nb of moves to win
    /// Neutral = 0
    /// Lost = -1 * nb of moves to lose
    /// Example: if the AI can win in 3 moves, the score will be 3
    /// Example: if the AI can lose in 3 moves, the score will be -3
    /// Example: if the AI can win in 3 moves but lose in 2 moves, the score will be - 2
    pub fn get_display_score(&self) -> i8 {
        match self {
            MoveResult::Lost(x) => { *x as i8 * -1 }
            MoveResult::Neutral => { 0 }
            MoveResult::Win(x) => { *x as i8 }
        }
    }
}

pub struct MiniMaxResult {
    pub best_move_id: i8,
    pub move_score: i8,
    pub move_results: Vec<Option<MoveResult>>,
    pub move_result: Option<MoveResult>,
}

pub fn mini_max(game: &mut Game, max_depth: u8, cur_depth: u8, maximizing_player: bool) -> MiniMaxResult {
    let mut scores: Vec<Option<i8>> = vec![None; game.grid[0].len()];
    let mut move_results: Vec<Option<MoveResult>> = vec![None; game.grid[0].len()];
    for col in 0..game.grid[0].len() {
        if game.can_add_token(col) {
            let line = game.insert_token(col).unwrap();
            if game.check_win(col, line) {
                if maximizing_player {
                    let res = MoveResult::Win(cur_depth);
                    scores[col] = Some(res.score(max_depth));
                    move_results[col] = Some(res);
                } else {
                    let res = MoveResult::Lost(cur_depth);
                    scores[col] = Some(res.score(max_depth));
                    move_results[col] = Some(res);
                }
            } else if cur_depth <= max_depth {
                let res = mini_max(game, max_depth, cur_depth + 1, !maximizing_player);
                scores[col] = Some(res.move_score);
                move_results[col] = res.move_result;
            } else {
                let res = MoveResult::Neutral;
                scores[col] = Some(res.score(max_depth));
                move_results[col] = Some(res);
            }
            game.grid[line][col] = Cell::Empty;
            game.next_player();
        } else {
            scores[col] = None;
            move_results[col] = None;
        }
    }


    match if maximizing_player {
        scores.iter().filter_map(|&x| x).max()
    } else {
        scores.iter().filter_map(|&x| x).min()
    } {
        Some(best_value) => {
            let mut rng = rand::thread_rng();

            let indexes: Vec<usize> = scores.iter()
                .enumerate()
                .filter(|&(_, &value)| value == Some(best_value))
                .map(|(index, _)| index)
                .collect();

            let rnd = rng.gen_range(0..indexes.len());
            let random_index = indexes[rnd];
            let move_result = move_results[random_index].clone();
            MiniMaxResult {
                best_move_id: random_index as i8,
                move_score: best_value,
                move_results,
                move_result,
            }
        }
        None => {
            MiniMaxResult {
                best_move_id: -1,
                move_score: -1,
                move_results,
                move_result: None,
            }
        }
    }
}
