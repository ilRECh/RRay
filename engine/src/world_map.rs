use ggez::{
    mint::Point2,
    GameResult,
    GameError
};

pub struct WorldMap {
    world_map: Vec<Vec<i32>>
}

impl WorldMap {
    pub fn new() -> Self {
        let world_map = vec![
            vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
            vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            vec![1,0,0,0,0,0,2,2,2,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
            vec![1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
            vec![1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,3,0,0,0,3,0,0,0,1],
            vec![1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
            vec![1,0,0,0,0,0,2,2,0,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
            vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            vec![1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            vec![1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            vec![1,4,0,0,0,0,5,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            vec![1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            vec![1,4,0,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            vec![1,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            vec![1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]
        ];

        Self {
            world_map
        }
    }

    pub fn at(&self, x: i32, y: i32) -> GameResult<i32> {
        self.check_limits(x, y)?;

        Ok(self.world_map[x as usize][y as usize])
    }

    pub fn insert(&mut self, value: i32, x: i32, y: i32) -> GameResult {
        self.check_limits(x, y)?;

        self.world_map[x as usize][y as usize] = value;

        Ok(())
    }

    pub fn change_position(&mut self, from: Point2<i32>, to: Point2<i32>) -> GameResult {
        self.check_limits(from.x, from.y)?;
        self.check_limits(to.x, to.y)?;

        Ok(())
    }

    pub fn check_limits(&self, x: i32, y: i32) -> GameResult {
        if !(x >= 0 && (x as usize) < self.world_map.len()) ||
           !(y >= 0 && (y as usize) < self.world_map[0].len()) {
            let mut error_str = String::from("Err with coordintates: (x: ");

            error_str.push_str(x.to_string().as_str());
            error_str.push_str(", y: ");
            error_str.push_str(y.to_string().as_str());
            error_str.push_str("), map has: (x: ");
            error_str.push_str(self.world_map.len().to_string().as_str());
            error_str.push_str(", y: ");
            error_str.push_str(self.world_map[0].len().to_string().as_str());
            error_str.push_str(")");

            return Err(GameError::CustomError(error_str));
        }

        Ok(())
    }
}
