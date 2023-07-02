use std::fmt::{Display, Error, Formatter};
use std::io;
use std::os::unix::raw::gid_t;

#[derive(Clone, Debug, PartialEq)]
pub enum Cell {
    Yellow,
    Red,
    Empty,
}

#[derive(Debug)]
pub struct Game {
    grid: Vec<Vec<Cell>>,
    height: usize,
    width: usize,
}

impl Game {
    pub fn build(height: usize, width: usize) -> Game {
        Game {
            grid: vec![vec![Cell::Empty; width]; height],
            height,
            width,
        }
    }

    pub fn insert_token(&mut self, col: usize, cell: Cell) -> Result<(), &str> {
        self.grid.iter_mut()
            .rev()
            .find(|line| line[col] == Cell::Empty)
            .and_then(|line| {
                line[col] = cell;
                Some(())
            })
            .ok_or("No more room in the column")
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
        let mut game = Game::build(h, w);
        let all_empty = game.grid.iter().all(|row| {
            row.iter().all(|cell| *cell == Cell::Empty)
        });
        game.insert_token(3, Cell::Yellow).expect("Should have worked");
        assert_eq!(game.grid[5][3], Cell::Yellow);
        let non_yellow_count = game.grid.iter().flatten().filter(|&cell| *cell == Cell::Empty).count();
        assert_eq!(non_yellow_count, cell_nb - 1);

        game.insert_token(3, Cell::Red).expect("Should have worked");
        assert_eq!(game.grid[4][3], Cell::Red);
        let non_yellow_count = game.grid.iter().flatten().filter(|&cell| *cell == Cell::Empty).count();
        assert_eq!(non_yellow_count, cell_nb - 2);

        game.insert_token(1, Cell::Yellow).expect("Should have worked");
        let non_yellow_count = game.grid.iter().flatten().filter(|&cell| *cell == Cell::Empty).count();
        assert_eq!(game.grid[5][1], Cell::Yellow);
        assert_eq!(non_yellow_count, cell_nb - 3);
        game.insert_token(3, Cell::Red).expect("Should have worked");
        game.insert_token(3, Cell::Red).expect("Should have worked");
        game.insert_token(3, Cell::Red).expect("Should have worked");
        game.insert_token(3, Cell::Red).expect("Should have worked");
        assert_eq!(game.insert_token(3, Cell::Yellow), Err("No more room in the column"));
    }
}
