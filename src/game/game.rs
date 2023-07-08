use std::fmt::{Display, Formatter};

use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum Cell {
    Empty = 0,
    Yellow = 1,
    Red = 2,
}

#[derive(Debug, Clone)]
pub enum Player {
    Yellow = 0,
    Red = 1,
}

impl Player {
    pub fn get_cell(&self) -> Cell {
        match self {
            Player::Yellow => { Cell::Yellow }
            Player::Red => { Cell::Red }
        }
    }
}

#[derive(Debug)]
pub struct Game {
    pub grid: Vec<Vec<Cell>>,
    height: usize,
    width: usize,
    pub current_player: Player,
}


impl Game {
    pub fn build(height: usize, width: usize, starting_player: Option<Player>) -> Game {
        Game {
            grid: vec![vec![Cell::Empty; width]; height],
            height,
            width,
            current_player: starting_player.unwrap_or(Player::Red),
        }
    }

    pub fn reset(&mut self) {
        self.grid = vec![vec![Cell::Empty; self.width]; self.height];
    }

    pub fn check_win(&self, col: usize, line: usize) -> bool {
        let grid = &self.grid;
        let player = grid[line][col].clone();

        // Check horizontal
        if (0..=grid[0].len() - 4).any(|c| (0..4).all(|i| grid[line][c + i] == player)) {
            return true;
        }

        // Check vertical
        if (0..=grid.len() - 4).any(|r| (0..4).all(|i| grid[r + i][col] == player)) {
            return true;
        }

        // Check diagonal (top-left to bottom-right)
        if (0..=grid.len() - 4).any(|r| {
            (0..=grid[0].len() - 4).any(|c| (0..4).all(|i| grid[r + i][c + i] == player))
        }) {
            return true;
        }

        // Check diagonal (bottom-left to top-right)
        if (3..grid.len()).rev().any(|r| {
            (0..=grid[0].len() - 4).any(|c| (0..4).all(|i| grid[r - i][c + i] == player))
        }) {
            return true;
        }

        false
    }


    pub fn grid_full(&self) -> bool {
        self.grid[0].iter().all(|x| *x != Cell::Empty)
    }

    pub fn insert_token(&mut self, col: usize) -> Result<usize, &str> {
        let cell = self.current_player.get_cell();
        let res = self.grid.iter_mut()
            .enumerate()
            .rev()
            .find(|(_, line)| line[col] == Cell::Empty)
            .and_then(|(i, lines)| {
                lines[col] = cell;
                Some(i)
            })
            .ok_or("No more room in the column");
        if res.is_ok() {
            self.next_player();
        }
        res
    }

    pub fn can_add_token(&self, col: usize) -> bool {
        self.grid[0][col] == Cell::Empty
    }

    pub fn next_player(&mut self) {
        match self.current_player {
            Player::Yellow => { self.current_player = Player::Red }
            Player::Red => { self.current_player = Player::Yellow }
        }
    }
}

impl Display for Game {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        for line in self.grid.iter() {
            for cell in line {
                print!("{} ", cell);
            }
            println!()
        }
        Ok(())
    }
}

impl Display for Cell {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Yellow => { print!("Y") }
            Cell::Red => { print!("R") }
            Cell::Empty => { print!("E") }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_token() {
        let h = 6;
        let w = 7;
        let cell_nb = h * w;
        let mut game = Game::build(h, w, Some(Player::Yellow));
        game.grid.iter().all(|row| {
            row.iter().all(|cell| *cell == Cell::Empty)
        });
        game.insert_token(3).expect("Should have worked");
        assert_eq!(game.grid[5][3], Cell::Yellow);
        let non_yellow_count = game.grid.iter().flatten().filter(|&cell| *cell == Cell::Empty).count();
        assert_eq!(non_yellow_count, cell_nb - 1);

        game.insert_token(3).expect("Should have worked");
        assert_eq!(game.grid[4][3], Cell::Red);
        let non_yellow_count = game.grid.iter().flatten().filter(|&cell| *cell == Cell::Empty).count();
        assert_eq!(non_yellow_count, cell_nb - 2);

        game.insert_token(1).expect("Should have worked");
        let non_yellow_count = game.grid.iter().flatten().filter(|&cell| *cell == Cell::Empty).count();
        assert_eq!(game.grid[5][1], Cell::Yellow);
        assert_eq!(non_yellow_count, cell_nb - 3);
        game.insert_token(3).expect("Should have worked");
        game.insert_token(3).expect("Should have worked");
        game.insert_token(3).expect("Should have worked");
        assert!(game.can_add_token(3));
        game.insert_token(3).expect("Should have worked");
        assert!(!game.can_add_token(3));
        assert_eq!(game.insert_token(3), Err("No more room in the column"));
    }

    #[test]
    fn test_next_player() {
        fn match_red(player: &Player) {
            match player {
                Player::Yellow => { panic!("should be red") }
                Player::Red => {}
            }
        }
        fn match_yellow(player: &Player) {
            match player {
                Player::Yellow => {}
                Player::Red => { panic!("should be red") }
            }
        }
        let h = 6;
        let w = 7;
        let mut game = Game::build(h, w, Some(Player::Yellow));
        match_yellow(&game.current_player);
        game.next_player();
        match_red(&game.current_player);
        game.next_player();
        match_yellow(&game.current_player);
        game.next_player();
        match_red(&game.current_player);
    }


    #[test]
    fn test_grid_full() {
        let h = 6;
        let w = 7;
        let mut game = Game::build(h, w, Some(Player::Yellow));
        for i in 0..w {
            for _ in 0..h {
                game.insert_token(i).unwrap();
            }
        }
        assert!(game.grid_full())
    }

    #[test]
    fn test_grid_not_full() {
        let h = 6;
        let w = 7;
        let mut game = Game::build(h, w, Some(Player::Yellow));
        for i in 0..w {
            for _ in 0..h - 1 {
                game.insert_token(i).unwrap();
            }
        }
        assert!(!game.grid_full())
    }

    #[test]
    fn test_player_won_vertically() {
        let mut game = Game::build(6, 7, Some(Player::Red));
        game.insert_token(0).unwrap();
        game.insert_token(1).unwrap();
        game.insert_token(0).unwrap();
        game.insert_token(1).unwrap();
        game.insert_token(0).unwrap();
        let line = game.insert_token(1).unwrap();
        print!("{}", game);
        assert!(!game.check_win(1, line));
        let line = game.insert_token(0).unwrap();
        assert!(game.check_win(0, line)); // Player switched, so Red won
    }

    #[test]
    fn test_player_won_horizontal() {
        let mut game = Game::build(6, 7, Some(Player::Yellow));
        game.insert_token(0).unwrap();
        game.insert_token(1).unwrap();
        game.insert_token(0).unwrap();
        game.insert_token(2).unwrap();
        game.insert_token(0).unwrap();
        game.insert_token(3).unwrap();
        let line = game.insert_token(5).unwrap();
        assert!(!game.check_win(5, line));
        let line = game.insert_token(4).unwrap();
        print!("{}", game);
        assert!(game.check_win(4, line));
    }

    #[test]
    fn test_player_won_diagonal() {
        let mut game = Game::build(6, 7, Some(Player::Yellow));
        game.insert_token(0).unwrap();// Y
        game.insert_token(1).unwrap();// R
        game.insert_token(1).unwrap();// Y
        game.insert_token(2).unwrap();// R
        game.insert_token(2).unwrap();// Y
        game.insert_token(3).unwrap();// R
        game.insert_token(2).unwrap();// Y
        game.insert_token(3).unwrap();// R
        game.insert_token(3).unwrap();// Y
        let line = game.insert_token(5).unwrap();// R
        assert!(!game.check_win(5, line));

        let line = game.insert_token(3).unwrap();// Y
        assert!(game.check_win(3, line));
    }
}
