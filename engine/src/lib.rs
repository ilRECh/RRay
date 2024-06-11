mod player;
mod world_map;

use ggez::{
    conf, event::{
        self,
        EventHandler
    }, glam::*, graphics::{
        self,
        Color
    },
    input::keyboard::KeyCode,
    mint::Point2,
    Context,
    ContextBuilder,
    GameResult
};

use player::Player;
use world_map::WorldMap;

struct GameState {
    world_map: WorldMap,
    player: Player,

    mouse_last_position: Point2<f32>,
    time_next_update: u128,
    redraw: bool
}

impl GameState {
    pub fn new(_ctx: &mut Context) -> GameResult<Self> {
        let mut world_map = WorldMap::new();
        let player = Player::new(22, 12, &mut world_map)?;

        Ok(Self {
            world_map,
            player,

            mouse_last_position: Point2::from_slice(&[0.0, 0.0]),
            time_next_update: 0,
            redraw: true
        })
    }
}

impl EventHandler for GameState {  
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const UPDATE_TIME_SHIFT: u128 = 10;
        const MODIFIER_MOVE_SPEED: f64 = 0.99;
        const MODIFIER_ROTATION_SPEED: f64 = 0.01;

        let current_time = ctx.time.time_since_start().as_millis();

        if current_time > self.time_next_update {
            self.time_next_update = current_time + UPDATE_TIME_SHIFT;

            let keyboard = ctx.keyboard.pressed_keys();
            let move_speed = MODIFIER_MOVE_SPEED;

            if !keyboard.is_empty() {
                self.redraw = true;

                for key in keyboard.iter() {
                    match key {
                        KeyCode::W => {
                            let player = &mut self.player;
                            let (next_position_x, next_position_y) = (
                                player.position.x + player.direction.x * move_speed,
                                player.position.y + player.direction.y * move_speed
                            );

                            if self.world_map.at(
                                next_position_x as i32,
                                player.position.y as i32
                            ).unwrap() == 0 {
                                player.position.x = next_position_x;
                            }

                            if self.world_map.at(
                                player.position.x as i32,
                                next_position_y as i32
                            ).unwrap() == 0 {
                                player.position.y = next_position_y;
                            }
                        },
                        KeyCode::A => {
                            let player = &mut self.player;
                            let (perp_direction_x, perp_direction_y) = (
                                player.direction.y * -1.0,
                                player.direction.x
                            );
                            let (next_position_x, next_position_y) = (
                                player.position.x + perp_direction_x * move_speed,
                                player.position.y + perp_direction_y * move_speed
                            );

                            if self.world_map.at(
                                next_position_x as i32,
                                player.position.y as i32
                            ).unwrap() == 0 {
                                player.position.x = next_position_x;
                            }

                            if self.world_map.at(
                                player.position.x as i32,
                                next_position_y as i32
                            ).unwrap() == 0 {
                                player.position.y = next_position_y;
                            }
                        },
                        KeyCode::S => {
                            let player = &mut self.player;
                            let (next_position_x, next_position_y) = (
                                player.position.x - player.direction.x * move_speed,
                                player.position.y - player.direction.y * move_speed
                            );

                            if self.world_map.at(
                                next_position_x as i32,
                                player.position.y as i32
                            ).unwrap() == 0 {
                                player.position.x = next_position_x;
                            }

                            if self.world_map.at(
                                player.position.x as i32,
                                next_position_y as i32
                            ).unwrap() == 0 {
                                player.position.y = next_position_y;
                            }
                        },
                        KeyCode::D => {
                            let player = &mut self.player;
                            let (perp_direction_x, perp_direction_y) = (
                                player.direction.y,
                                player.direction.x * -1.0
                            );
                            let (next_position_x, next_position_y) = (
                                player.position.x + perp_direction_x * move_speed,
                                player.position.y + perp_direction_y * move_speed
                            );

                            if self.world_map.at(
                                next_position_x as i32,
                                player.position.y as i32
                            ).unwrap() == 0 {
                                player.position.x = next_position_x;
                            }

                            if self.world_map.at(
                                player.position.x as i32,
                                next_position_y as i32
                            ).unwrap() == 0 {
                                player.position.y = next_position_y;
                            }
                        },
                        _ => ()
                    }
                }
            }

            let new_mouse_position = ctx.mouse.position();
            let mouse_offset = self.mouse_last_position.x - new_mouse_position.x;
            let mouse_move = if mouse_offset > 0.1 { mouse_offset.signum() } else { 0.0 };

            println!("{} {} {}", self.mouse_last_position.x, new_mouse_position.x, mouse_move);

            self.mouse_last_position = new_mouse_position;

            if mouse_move > 0.0 {
                self.redraw = true;
                let old_direction_x = self.player.direction.x;
                self.player.direction.x = old_direction_x * f64::cos(MODIFIER_ROTATION_SPEED) - self.player.direction.y * f64::sin(MODIFIER_ROTATION_SPEED);
                self.player.direction.y = old_direction_x * f64::sin(MODIFIER_ROTATION_SPEED) + self.player.direction.y * f64::cos(MODIFIER_ROTATION_SPEED);

                let old_camera_x = self.player.camera.x;
                self.player.camera.x = old_camera_x * f64::cos(MODIFIER_ROTATION_SPEED) - self.player.camera.y * f64::sin(MODIFIER_ROTATION_SPEED);
                self.player.camera.y = old_camera_x * f64::sin(MODIFIER_ROTATION_SPEED) + self.player.camera.y * f64::cos(MODIFIER_ROTATION_SPEED);
            } else if mouse_move < 0.0 {
                // //both camera direction and camera plane must be rotated
                // double oldDirX = dirX;
                // dirX = dirX * cos(rotSpeed) - dirY * sin(rotSpeed);
                // dirY = oldDirX * sin(rotSpeed) + dirY * cos(rotSpeed);
                // double oldPlaneX = planeX;
                // planeX = planeX * cos(rotSpeed) - planeY * sin(rotSpeed);
                // planeY = oldPlaneX * sin(rotSpeed) + planeY * cos(rotSpeed);
            }
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

        let screen_width = ctx.gfx.window().outer_size().width;
        let screen_height = ctx.gfx.window().outer_size().height;

        for x in 0..screen_width {
            let screen_x = 2.0 * (x as f64) / (screen_width as f64) - 1.0;
            let ray_dir_x = player.direction.x + player.camera.x * screen_x;
            let ray_dir_y = player.direction.y + player.camera.y * screen_x;

            // which box of the map we're in
            let mut map_x = player.position.x as i32;
            let mut map_y = player.position.y as i32;

            // length of ray from one x or y-size to next x or y-side correspondingly
            let delta_dist_x = f64::abs(1.0 / ray_dir_x);
            let delta_dist_y = f64::abs(1.0 / ray_dir_y);

            // side_dist_ : length of ray from current position to next x or y-side
            // step_ : what direction to step in x or y-direction (either -1 or 1)
            let (mut side_dist_x, step_x) = if ray_dir_x < 0.0 {
                    ((player.position.x - map_x as f64) * delta_dist_x, -1)
                } else { 
                    ((map_x as f64 + 1.0 - player.position.x) * delta_dist_x, 1)
                };

            let (mut side_dist_y, step_y) = if ray_dir_y < 0.0 {
                    ((player.position.y - map_y as f64) * delta_dist_y, -1)
                } else { 
                    ((map_y as f64 + 1.0 - player.position.y) * delta_dist_y, 1)
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

                match self.world_map.at(map_x, map_y) {
                    Ok(maybe_wall) if maybe_wall > 0 => {
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
            let line_height = screen_height as f64 / perp_wall_dist;

            //calculate lowest and highest pixel to fill in current stripe
            let mut draw_start = -1.0 * line_height / 2.0 + screen_height as f64 / 2.0;

            if draw_start < 0.0 {
                draw_start = 0.0;
            }

            let mut draw_end = line_height / 2.0 + screen_height as f64 / 2.0;

            if draw_end >= screen_height as f64 {
                draw_end = screen_height as f64 - 1.0;
            }

            canvas.draw(
                &graphics::Mesh::new_line(
                    ctx, 
                    &[
                        Point2::from_slice(&[x as f32, draw_start as f32]),
                        Point2::from_slice(&[x as f32, draw_end as f32])
                    ],
                    1.,
                    color
                )?,
            graphics::DrawParam::new());
        }

        canvas.finish(ctx)?;

        // ggez::timer::yield_now();

        Ok(())
    }
}

pub fn run(game_id: &str, author: &str) -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new(game_id, author)
        .window_setup(conf::WindowSetup::default().title("rray"))
        .window_mode(conf::WindowMode::default().dimensions(640.0, 480.0))
        .build()
        .expect("Should be able to create Context");

    let game = GameState::new(&mut ctx)?;

    event::run(ctx, event_loop, game);
}
