mod game;
mod server;
use crate::game::game::{Cell, Game};

fn main() {
    println!("Hello, world!");
    let mut game = Game::build(6, 7);
    println!("{}", game);
    game.insert_token(1, Cell::Red);
    game.insert_token(2, Cell::Red);
    game.insert_token(2, Cell::Red);
    game.insert_token(2, Cell::Red);
    game.insert_token(2, Cell::Red);
    game.insert_token(5, Cell::Red);
    println!("{}", game);

}
