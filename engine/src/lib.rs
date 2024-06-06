use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};

struct MyGame {
    // Your state here...
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            // ...
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        // Draw code here...
        canvas.finish(ctx)
    }
}

pub fn run(game_id: &str, author: &str) {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new(game_id, author)
        .build()
        .expect("Should be able to create Context");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, game);
}
