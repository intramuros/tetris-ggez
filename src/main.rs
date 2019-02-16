use ggez::{event, GameResult};

mod game_state;
mod tetromino;

use crate::tetromino::SCREEN_SIZE;
use game_state::*;

fn main() -> GameResult {
    let (ctx, events_loop) = &mut ggez::ContextBuilder::new("tetris", "me")
        .window_setup(ggez::conf::WindowSetup::default().title("Tetris!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let state = &mut GameState::new();
    event::run(ctx, events_loop, state)
}
