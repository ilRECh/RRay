mod player;
mod world_map;
mod game_state;

use ggez::{
    GameResult,
    ContextBuilder,
    conf,
    event::{
        self
    }
};

use game_state::GameState;

pub fn run(game_id: &str, author: &str) -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new(game_id, author)
        .window_setup(conf::WindowSetup::default().title("rray"))
        .window_mode(conf::WindowMode::default().dimensions(640.0, 480.0))
        .build()
        .expect("Should be able to create Context");

    let game = GameState::new(&mut ctx)?;

    event::run(ctx, event_loop, game);
}
