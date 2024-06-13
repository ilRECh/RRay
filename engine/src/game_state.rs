use std::collections::HashSet;

use ggez::{
    event::EventHandler,
    glam::*, graphics::{
        self,
        Color
    },
    input::{
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

const UPDATE_TIME_SHIFT: u128 = 10;

pub struct GameState {
    screen_size: PhysicalSize<u32>,
    world_map: Rc<RefCell<WorldMap>>,
    player: Player,

    time_next_update: u128,
    redraw: bool
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut world_map = Rc::new(RefCell::new(WorldMap::new()));
        let player = Player::new(22, 12, &world_map)?;

        mouse::set_cursor_grabbed(ctx, true)?;

        Ok(Self {
            screen_size: ctx.gfx.window().outer_size(),
            world_map,
            player,
            time_next_update: 0,
            redraw: true
        })
    }
}

impl GameState {
    fn handle_keyboard(&mut self, keyboard: &HashSet<KeyCode>) -> bool {
        if keyboard.is_empty() {
           return false; 
        }

        let mut status = false;

        for key in keyboard.iter() {
            status = match key {
                KeyCode::W => {
                    self.player.move_forward();

                    true
                },
                KeyCode::A => {
                    self.player.move_strafe_left();

                    true
                },
                KeyCode::S => {
                    self.player.move_backward();

                    true
                },
                KeyCode::D => {
                    self.player.move_strafe_right();

                    true
                },
                _ => status
            };
        }

        status
    }

    fn handle_mouse(&mut self, mouse_position_x: f32) -> bool {
        let mouse_offset = mouse_position_x - self.screen_size.width as f32 / 2.0;

        if mouse_offset.abs() > self.screen_size.width as f32 / 4.0 {
            self.player.move_rotate(-1.0 * mouse_offset);
        
            return true;
        }

        false
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let current_time = ctx.time.time_since_start().as_millis();

        if current_time < self.time_next_update {
            return Ok(());
        }

        self.time_next_update = current_time + UPDATE_TIME_SHIFT;

        if self.handle_keyboard(ctx.keyboard.pressed_keys()) {
            self.redraw = true;
        }

        if self.handle_mouse(ctx.mouse.position().x) {
            self.redraw = true;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if !self.redraw {
            return Ok(());
        }

        self.redraw = false;

        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        let player = &self.player;

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

                match self.world_map.borrow_mut().at(map_x, map_y) {
                    Ok(maybe_wall) if maybe_wall > 0 && maybe_wall != b'P' as i32 => {
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
                    },
                    Err(e) => return Err(e),
                    _ => ()
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

            canvas.draw(
                &graphics::Mesh::new_line(
                    ctx, 
                    &[
                        vec2(x as f32, draw_start),
                        vec2(x as f32, draw_end),
                    ],
                    1.,
                    color
                )?,
            graphics::DrawParam::new());
        }

        canvas.finish(ctx)?;

        ggez::timer::yield_now();

        Ok(())
    }
}
