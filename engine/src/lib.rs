mod player;
mod world_map;
mod game_state;
mod raycasting;
mod texture;

pub use crate::player::Player;
pub use crate::world_map::WorldMap;
pub use crate::texture::Texture;

use std::path;

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
    let resource_dir = path::PathBuf::from("/home/ildar/coding/rray/game/textures");

    let (mut ctx, event_loop) = ContextBuilder::new(game_id, author)
        .window_setup(conf::WindowSetup::default().title("rray"))
        .window_mode(conf::WindowMode::default().dimensions(640.0, 480.0))
        .add_resource_path(resource_dir)
        .build()
        .expect("Should be able to create Context");

    let game = GameState::new(&mut ctx)?;

    event::run(ctx, event_loop, game);
}
