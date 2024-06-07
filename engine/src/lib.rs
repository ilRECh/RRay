use ggez::{
    Context, ContextBuilder, GameResult,
    graphics::{
        self,
        Color
    },
    event::{
        self,
        EventHandler
    },
    conf,
    glam::*
};

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

        let world_map = [
            [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
            [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,0,0,0,0,2,2,2,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
            [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,3,0,0,0,3,0,0,0,1],
            [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,0,0,0,0,2,2,0,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
            [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,4,0,0,0,0,5,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,4,0,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]
        ];

        let (mut pos_x, mut pos_y) = (22.0, 12.0);
        let (mut dir_x, mut dir_y) = (-1.0 , 0.0);
        let (mut plane_x, mut plane_y) = (0.0, 0.66);

        let screen_width = ctx.gfx.window().outer_size().width;
        let screen_height = ctx.gfx.window().outer_size().height;

        for x in 0..screen_width {
            let camera_x = 2.0 * (x as f64) / (screen_width as f64) - 1.0;
            let ray_dir_x = dir_x + plane_x * camera_x;
            let ray_dir_y = dir_y + plane_y * camera_x;

            // which box of the map we're in
            let mut map_x = pos_x as i32;
            let mut map_y = pos_y as i32;

            // length of ray from one x or y-size to next x or y-side correspondingly
            let delta_dist_x = f64::abs(1.0 / ray_dir_x);
            let delta_dist_y = f64::abs(1.0 / ray_dir_y);

            // side_dist_ : length of ray from current position to next x or y-side
            // step_ : what direction to step in x or y-direction (either -1 or 1)
            let (mut side_dist_x, mut step_x) = if ray_dir_x < 0.0 {
                    ((pos_x - map_x as f64) * delta_dist_x, -1)
                } else { 
                    ((map_x as f64 + 1.0 - pos_x) * delta_dist_x, 1)
                };

            let (mut side_dist_y, mut step_y) = if ray_dir_y < 0.0 {
                    ((pos_y - map_y as f64) * delta_dist_y, -1)
                } else { 
                    ((map_y as f64 + 1.0 - pos_y) * delta_dist_y, 1)
                };

            // was a wall hit?
            let mut hit = false;

            // which side was hit? (NS or WE)
            let mut side = 0;

            while !hit {
                if side_dist_x < side_dist_y {
                    side_dist_x += delta_dist_x;
                    map_x += step_x;
                    side = 0;
                } else {
                    side_dist_y += delta_dist_y;
                    map_y += step_y;
                    side = 1;
                }

                if world_map[map_x as usize][map_y as usize] > 0 {
                    hit = true;
                }
            }

            let perp_wall_dist = if side == 0 {
                    side_dist_x - delta_dist_x
                } else {
                    side_dist_y - delta_dist_y
                };

            // calculate height of line to draw on screen
            let line_height = screen_height as i32 / perp_wall_dist as i32;

            //calculate lowest and highest pixel to fill in current stripe
            let mut draw_start = -1 * line_height / 2 + screen_height as i32 / 2;

            if draw_start < 0 {
                draw_start = 0;
            }

            let mut draw_end = line_height / 2 + screen_height as i32 / 2;

            if draw_end >= screen_height as i32 {
                draw_end = screen_height as i32 - 1;
            }

            let mut color = match world_map[map_x as usize][map_y as usize] {
                    1 => Color::RED,
                    2 => Color::GREEN,
                    3 => Color::BLUE,
                    4 => Color::WHITE,
                    _ => Color::YELLOW
                };
            
            if side == 1 {
                color = Color::from_rgb_u32(color.to_rgb_u32() / 2);
            }

            let line = graphics::Mesh::new_line(
                ctx, 
                &[
                    vec2(x as f32, draw_start as f32),
                    vec2(x as f32, draw_end as f32)
                ],
                1.0,
                color
            )?;

            canvas.draw(&line, graphics::DrawParam::new());
        }

        canvas.finish(ctx)
    }
}

pub fn run(game_id: &str, author: &str) {
    let (mut ctx, event_loop) = ContextBuilder::new(game_id, author)
        .window_setup(conf::WindowSetup::default().title("rray"))
        .window_mode(conf::WindowMode::default().dimensions(640.0, 480.0))
        .build()
        .expect("Should be able to create Context");

    let game = MyGame::new(&mut ctx);

    event::run(ctx, event_loop, game);
}
