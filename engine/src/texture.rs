pub struct Texture {
    walls: WallsTexture
}

use crate::raycasting::Line;

type WallsTexture = Vec<Vec<u32>>;

const TEXTURE_WIDTH: usize = 64;
const TEXTURE_HEIGHT: usize = 64;

const ONE: i32 = 1;

impl Texture {
    pub fn new(_path: &str) -> Self {
        let walls = Self::generate_textures();
        
        Self {
            walls
        }
    }

    pub fn convert_dda_to_pixels(&self, lines: Vec<Line>, pixels: &mut [u8]) {
        for line in lines.iter() {
            let wall = self.code_to_texture(line.wall_code);

            // i can modify the wall brigtness anyhow. this time i'm just dividing it by dimm.1
            // if dimm.0 is true
            let dimm = (line.wall_side > 0, line.wall_side);

            
        }
    }

    fn code_to_texture(&self, code: i32) -> &Vec<u32> {
        match code {
            ONE => {
                &self.walls[0]
            },
            _ => panic!("No such code is associated with any texture!")
        }
    }

    fn generate_textures() -> WallsTexture {
        vec![vec![0; TEXTURE_WIDTH * TEXTURE_WIDTH]; 8]
    }
}
