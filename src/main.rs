use std::collections::HashMap;
use std::sync::Mutex;

use actix_web::{App, error, get, HttpServer, post, Responder, Result, web};
use actix_web::middleware::Logger;
use log::info;
use serde::Deserialize;
use serde::Serialize;
use clap::{App as ClapApp, Arg};

use crate::game::game::{Game};

mod game;
mod solver;

#[derive(Serialize)]
struct GridReponse {
    grid: Vec<Vec<u8>>,
    pub current_player_color: u8,
}

#[get("/api/grid")]
async fn grid(data: web::Data<Mutex<Game>>) -> Result<impl Responder> {
    let game = data.lock().unwrap();
    Ok(web::Json(GridReponse { grid: game.grid.clone().iter().map(|x| x.iter().map(|y| y.clone() as u8).collect()).collect(), current_player_color: game.current_player.clone() as u8 }))
}


#[derive(Deserialize)]
struct TokenRequest {
    column: usize,
}

#[derive(Deserialize)]
struct MinimaxRequest {
    depth: u8,
}


#[derive(Serialize)]
struct TokenResponse {
    added_cell: u8,
    column: usize,
    current_player: u8,
    is_grid_full: bool,
    line: usize,
    next_player: u8,
    player_won: bool,
}

#[derive(Serialize)]
struct MiniMaxResponse {
    best_move: i8,
    scores: Vec<i8>,
}

#[post("/api/token")]
async fn add_token(info: web::Query<TokenRequest>, data: web::Data<Mutex<Game>>) -> Result<impl Responder> {
    let mut game = data.lock().unwrap();
    let current_player = game.current_player.clone();
    return match game.insert_token(info.column) {
        Ok(line) => {
            Ok(web::Json(TokenResponse {
                added_cell: current_player.get_cell() as u8,
                column: info.column,
                current_player: current_player.clone() as u8,
                is_grid_full: game.grid_full(),
                line,
                next_player: game.current_player.clone() as u8,
                player_won: game.check_win(info.column, line),
            }))
        }
        Err(_) => { Err(error::ErrorBadRequest("broo")) }
    };
}

#[post("/api/grid/reset")]
async fn reset(data: web::Data<Mutex<Game>>) -> Result<impl Responder> {
    data.lock().unwrap().reset();
    Ok("")
}

#[get("/api/solver/minimax")]
async fn minimax(data: web::Data<Mutex<Game>>, info: web::Query<MinimaxRequest>) -> Result<impl Responder> {
    let mut game = data.lock().unwrap();
    let res = solver::mini_max(&mut game, info.depth, true);
    Ok(web::Json(MiniMaxResponse { best_move: res.best_move_id, scores: res.scores }))
}

#[derive(Serialize)]
struct ConfigResponse {
    github_url: String,
    title: String,
    related_links: HashMap<String, String>,
}

#[get("/api/configuration")]
async fn get_config() -> Result<impl Responder> {
    Ok(web::Json(ConfigResponse {
        github_url: String::from("https://github.com/RomainMichau/Connect4Solver_rust"),
        title: String::from("Connect4 Rust"),
        related_links: HashMap::from([
            (String::from("Connect4 Go"), String::from("https://connect4-go.romainmic.com"))
        ]),
    }))
}


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let port = get_port();
    info!("Starting server on 0.0.0.0:{port}");
    let game = web::Data::new(Mutex::new(Game::build(6, 7, None)));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(game.clone())
            .service(grid)
            .service(add_token)
            .service(reset)
            .service(minimax)
            .service(get_config)
    })
        .bind(("0.0.0.0", 8081))?
        .run()
        .await
}


fn get_port() -> i16 {
    let matches = ClapApp::new("connect4")
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .help("port")
            .takes_value(true)
            .default_value("8081"))
        .get_matches();
    let port: i16 = matches.value_of("port").unwrap().parse().unwrap();
    return port;
}
