use ggez::{
    mint::*,
    glam::*,
    GameResult
};

use super::world_map::WorldMap;

pub struct Player {
    pub position: Point2<f64>,
    pub direction: Vector2<f64>,
    pub camera: Vector2<f64>
}

impl Player {
    pub fn new(x: i32, y: i32, map: &mut WorldMap) -> GameResult<Self> {
        map.insert(b'P' as i32, x, y)?;

        Ok(Self {
            position: Point2::from_slice(&[x as f64, y as f64]),
            direction: Vector2::from_slice(&[-1., 0.]),
            camera: Vector2::from_slice(&[0., 0.66])
        })
    }
}


