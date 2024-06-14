use ggez::{
    glam::{
        vec2,
        Vec2
    },
    mint::Point2,
    GameResult
};

const MODIFIER_MOVE_SPEED: f32 = 0.09;
const MODIFIER_ROTATION_SPEED: f32 = 0.005;

use crate::world_map::WorldMap;
use std::{
    cell::RefCell,
    rc::Rc
};

const MODIFIER_CAMERA: f32 = 2.0 / 3.0;

pub struct Player {
    pub position: Vec2,
    pub direction: Vec2,
    pub camera: Vec2,
    world_map: Rc<RefCell<WorldMap>>
}

impl Player {
    pub fn new(x: i32, y: i32, direction_deg: f32, world_map: &Rc<RefCell<WorldMap>>) -> GameResult<Self> {
        world_map.borrow_mut().insert(b'P' as i32, x, y);

        let direction_rad = direction_deg.to_radians();
        let direction = vec2(direction_rad.cos().round(), direction_rad.sin().round());

        let pl = Self {
            position: vec2(x as f32, y as f32),
            direction,
            camera: vec2(
                -1.0 * direction.y * MODIFIER_CAMERA,
                direction.x * MODIFIER_CAMERA
            ),
            world_map: Rc::clone(&world_map)
        };

        Ok(pl)
    }

    pub fn move_rotate(&mut self, rotation_speed: f32) {
        let rotation_speed = (rotation_speed * MODIFIER_ROTATION_SPEED).to_radians();
        
        let old_direction_x = self.direction.x;
        self.direction.x = old_direction_x * f32::cos(rotation_speed) - self.direction.y * f32::sin(rotation_speed);
        self.direction.y = old_direction_x * f32::sin(rotation_speed) + self.direction.y * f32::cos(rotation_speed);
        
        let old_camera_x = self.camera.x;
        self.camera.x = old_camera_x * f32::cos(rotation_speed) - self.camera.y * f32::sin(rotation_speed);
        self.camera.y = old_camera_x * f32::sin(rotation_speed) + self.camera.y * f32::cos(rotation_speed);
    }

    pub fn move_forward(&mut self) {
        self.step(
            self.position.x + self.direction.x * MODIFIER_MOVE_SPEED,
            self.position.y + self.direction.y * MODIFIER_MOVE_SPEED
        );
    }

    pub fn move_backward(&mut self) {
        self.step(
            self.position.x - self.direction.x * MODIFIER_MOVE_SPEED,
            self.position.y - self.direction.y * MODIFIER_MOVE_SPEED
        );
    }

    pub fn move_strafe_right(&mut self) {
        let (perp_direction_x, perp_direction_y) = (
            self.direction.y * -1.0,
            self.direction.x
        );

        self.step(
            self.position.x + perp_direction_x * MODIFIER_MOVE_SPEED,
            self.position.y + perp_direction_y * MODIFIER_MOVE_SPEED
        );
    }

    pub fn move_strafe_left(&mut self) {
        let (perp_direction_x, perp_direction_y) = (
            self.direction.y,
            self.direction.x * -1.0
        );

        self.step(
            self.position.x + perp_direction_x * MODIFIER_MOVE_SPEED,
            self.position.y + perp_direction_y * MODIFIER_MOVE_SPEED
        );
    }

    fn step(&mut self, next_position_x: f32, next_position_y: f32) {
        if self.world_map.borrow_mut().change_position(
            Point2::from_slice(&[self.position.x as i32, self.position.y as i32]), 
            Point2::from_slice(&[next_position_x as i32, next_position_y as i32])
        ) {
            self.position.x = next_position_x;
            self.position.y = next_position_y;
        }
    }
}


