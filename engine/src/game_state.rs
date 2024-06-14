use std::collections::HashSet;

use ggez::{
    event::EventHandler,
    glam::*,
    graphics::{
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

use crate::raycasting;
use crate::Player;
use crate::WorldMap;
use crate::Texture;
use std::{
    rc::Rc,
    cell::RefCell
};

pub struct GameState {
    screen: graphics::ScreenImage,
    screen_size: PhysicalSize<u32>,
    fps: f64,

    world_map: Rc<RefCell<WorldMap>>,
    player: Player,
    texture: Texture
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let world_map = Rc::new(RefCell::new(WorldMap::new()));
        let player = Player::new(2, 5, 270.0, &world_map)?;

        mouse::set_cursor_grabbed(ctx, true)?;
        mouse::set_cursor_hidden(ctx, true);

        let screen = 
            graphics::ScreenImage::new(ctx, graphics::ImageFormat::Rgba8UnormSrgb, 1., 1., 1);

        let texture = Texture::new(ctx);

        Ok(Self {
            screen,
            screen_size: ctx.gfx.window().outer_size(),
            fps: 0.0,

            world_map,
            player,
            texture
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
            self.player.move_rotate(mouse_offset);
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;

        while ctx.time.check_update_time(DESIRED_FPS) {
            self.handle_keyboard(ctx.keyboard.pressed_keys());
            self.handle_mouse(ctx.mouse.position().x);
        }
        
        if ctx.time.ticks() % 100 == 0 {
            self.fps = ctx.time.fps().floor();
        }
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        const RGBA_CHANNELS: usize = 4;
        let mut pixels = vec![0;
            self.screen_size.width as usize * self.screen_size.height as usize * RGBA_CHANNELS
        ];

        let lines_floor = raycasting::dda_floor(
            &self.screen_size, &self.player
        );
        self.texture.convert_dda_floor_to_pixels(&self.screen_size, lines_floor, &mut pixels);

        let lines_walls = raycasting::dda_walls(
            &self.screen_size, &self.player, &self.world_map.borrow()
        );
        self.texture.convert_dda_walls_to_pixels(&self.screen_size, lines_walls, &mut pixels);

        let mut canvas = graphics::Canvas::from_screen_image(
            ctx, &mut self.screen, Color::BLACK
        );
        let image = graphics::Image::from_pixels(
            ctx, pixels.as_slice(),
            graphics::ImageFormat::Rgba8UnormSrgb,
            self.screen_size.width,
            self.screen_size.height
        );

        canvas.draw(&image, graphics::DrawParam::new());
        canvas.draw(
            &graphics::Text::new(
                String::from("FPS: ") + self.fps.to_string().as_str()
            ),
            graphics::DrawParam::new()
        );
        canvas.finish(ctx)?;
        ctx.gfx.present(&self.screen.image(ctx))?;

        ggez::timer::yield_now();
        Ok(())
    }
}
