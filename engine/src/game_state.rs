use std::collections::HashSet;

use ggez::{
    event::EventHandler, glam::*, graphics::{
        self, Color, DrawParam, Rect
    }, input::{
        keyboard::KeyCode,
        mouse
    },
    winit::dpi::PhysicalSize,
    Context,
    GameResult
};

use crate::player::Player;
use crate::world_map::WorldMap;
use std::{
    rc::Rc,
    cell::RefCell
};

pub struct GameState {
    screen: graphics::ScreenImage,
    screen_size: PhysicalSize<u32>,
    world_map: Rc<RefCell<WorldMap>>,
    player: Player,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let world_map = Rc::new(RefCell::new(WorldMap::new()));
        let player = Player::new(22, 12, &world_map)?;

        mouse::set_cursor_grabbed(ctx, true)?;

        let screen = 
            graphics::ScreenImage::new(ctx, graphics::ImageFormat::Rgba8UnormSrgb, 1., 1., 1);

        Ok(Self {
            screen,
            screen_size: ctx.gfx.window().outer_size(),
            world_map,
            player,
        })
    }
}

impl GameState {
    fn handle_keyboard(&mut self, keyboard: &HashSet<KeyCode>) {
        if keyboard.is_empty() {
           return; 
        }

        for key in keyboard.iter() {
            match key {
                KeyCode::W => {
                    self.player.move_forward();
                },
                KeyCode::A => {
                    self.player.move_strafe_left();
                },
                KeyCode::S => {
                    self.player.move_backward();
                },
                KeyCode::D => {
                    self.player.move_strafe_right();
                },
                _ => ()
            }
        }
    }

    fn handle_mouse(&mut self, mouse_position_x: f32) {
        let mouse_offset = mouse_position_x - self.screen_size.width as f32 / 2.0;

        if mouse_offset.abs() > self.screen_size.width as f32 / 4.0 {
            self.player.move_rotate(-1.0 * mouse_offset);
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // if ctx.time.ticks() % 100 == 0 {
        //     println!("{}  {}", ctx.time.fps(), ctx.time.delta().as_millis());
        // }

        const DESIRED_FPS: u32 = 60;

        while ctx.time.check_update_time(DESIRED_FPS) {
            self.handle_keyboard(ctx.keyboard.pressed_keys());
            self.handle_mouse(ctx.mouse.position().x);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_screen_image(ctx, &mut self.screen, Color::BLACK);
        let player = &self.player;

        let mut lines = graphics::InstanceArray::new(ctx, None);
        lines.resize(ctx, 640);

        for x in 0..self.screen_size.width {
            let screen_x = 2.0 * (x as f32) / (self.screen_size.width as f32) - 1.0;
            let ray_dir_x = player.direction.x + player.camera.x * screen_x;
            let ray_dir_y = player.direction.y + player.camera.y * screen_x;

            // which box of the map we're in
            let mut map_x = player.position.x as i32;
            let mut map_y = player.position.y as i32;

            // length of ray from one x or y-size to next x or y-side correspondingly
            let delta_dist_x = f32::abs(1.0 / ray_dir_x);
            let delta_dist_y = f32::abs(1.0 / ray_dir_y);

            // side_dist_ : length of ray from current position to next x or y-side
            // step_ : what direction to step in x or y-direction (either -1 or 1)
            let (mut side_dist_x, step_x) = if ray_dir_x < 0.0 {
                    ((player.position.x - map_x as f32) * delta_dist_x, -1)
                } else { 
                    ((map_x as f32 + 1.0 - player.position.x) * delta_dist_x, 1)
                };

            let (mut side_dist_y, step_y) = if ray_dir_y < 0.0 {
                    ((player.position.y - map_y as f32) * delta_dist_y, -1)
                } else { 
                    ((map_y as f32 + 1.0 - player.position.y) * delta_dist_y, 1)
                };

            // which side was hit? (NS or WE)
            let mut side;

            let color = loop {
                if side_dist_x < side_dist_y {
                    side_dist_x += delta_dist_x;
                    map_x += step_x;
                    side = 0;
                } else {
                    side_dist_y += delta_dist_y;
                    map_y += step_y;
                    side = 1;
                }

                let maybe_wall = self.world_map.borrow_mut().at(&mut map_x, &mut map_y);

                if maybe_wall > 0 && maybe_wall != b'P' as i32 {
                    let mut color = match maybe_wall {
                        1 => Color::RED,
                        2 => Color::GREEN,
                        3 => Color::BLUE,
                        4 => Color::YELLOW,
                        _ => Color::WHITE
                    };

                    if side == 1 {
                        color.a /= 2.0;
                    }

                    break color;
                }
            };

            let perp_wall_dist = if side == 0 {
                    side_dist_x - delta_dist_x
                } else {
                    side_dist_y - delta_dist_y
                };

            // calculate height of line to draw on screen
            let line_height = self.screen_size.height as f32 / perp_wall_dist;

            //calculate lowest and highest pixel to fill in current stripe
            let mut draw_start = -1.0 * line_height / 2.0 + self.screen_size.height as f32 / 2.0;

            if draw_start < 0.0 {
                draw_start = 0.0;
            }

            let mut draw_end = line_height / 2.0 + self.screen_size.height as f32 / 2.0;

            if draw_end >= self.screen_size.height as f32 {
                draw_end = self.screen_size.height as f32 - 1.0;
            }

            lines.push(DrawParam::new()
                .color(color)
                .dest(vec2(x as f32, draw_start))
                .scale(vec2(1.0, (draw_end - draw_start) / self.screen_size.height as f32))
            );
        }

        let def_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), Rect::new(0.0, 0.0, 1.0, 480.0), Color::WHITE)?;
        canvas.draw_instanced_mesh(def_mesh, &lines, DrawParam::new());

        canvas.finish(ctx)?;
        ctx.gfx.present(&self.screen.image(ctx))?;

        ggez::timer::yield_now();
        Ok(())
    }
}
