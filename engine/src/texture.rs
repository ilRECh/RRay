pub struct Texture {
    walls: WallsTexture
}

use ggez::winit::dpi::PhysicalSize;

use crate::raycasting::Line;

type WallsTexture = Vec<Vec<u32>>;

const TEXTURE_WIDTH: u32 = 64;
const TEXTURE_HEIGHT: u32 = 64;

// const WALL_TEXTURE_BLACK_CROSS: i32 = 1;
// const WALL_TEXTURE_SLOPED_GREYSCALE: i32 = 2;
// const WALL_TEXTURE_SLOPED_YELLOW_GRADIENT: i32 = 3;
// const WALL_TEXTURE_XOR_GREYSCALE: i32 = 4;
// const WALL_TEXTURE_XOR_GREEN: i32 = 5;
// const WALL_TEXTURE_RED_BRICKS: i32 = 6;
// const WALL_TEXTURE_RED_GRADIENT: i32 = 7;
const WALL_TEXTURE_FLAT_GREY: i32 = 8;

impl Texture {
    pub fn new(_path: &str) -> Self {
        let walls = Self::generate_textures();
        
        Self {
            walls
        }
    }

    pub fn convert_dda_to_pixels(
        &self,
        screen_size: &PhysicalSize<u32>,
        lines: Vec<Line>,
        pixels: &mut [u8]
    ) {
        for line in lines.iter() {
            let wall = self.code_to_texture(line.wall_code);

            // i can modify the wall brigtness however i want. this time i'm just dividing it
            let dimm = if line.wall_side > 0 { 4 } else { 1 };
            let mut texture_x = (line.wall_x * TEXTURE_WIDTH as f32) as u32;

            if (line.wall_side == 0 && line.ray_dir_x > 0.0) || (line.wall_side == 1 && line.ray_dir_y < 0.0) {
                texture_x = TEXTURE_WIDTH - texture_x - 1;
            }

            let step = TEXTURE_HEIGHT as f32 / (line.y_end - line.y_start);
            let mut texture_pos = (line.y_start - screen_size.height as f32 / 2.0 + (line.y_end - line.y_start) / 2.0) * step;

            let start = if line.y_start < 0.0 {
                texture_pos += -1.0 * line.y_start * step;
                0
            } else {
                line.y_start as u32
            };

            let end = if line.y_end >= screen_size.height as f32 {
                screen_size.height - 1
            } else {
                line.y_end as u32
            };

            for y in start..end {
                let texture_y = texture_pos as u32 & (TEXTURE_HEIGHT - 1);
                texture_pos += step;
                let mut pixel = wall[(TEXTURE_HEIGHT * texture_y + texture_x) as usize].to_be_bytes();
                pixel[0] = 255 / dimm;

                let index_pixel = (y * screen_size.width + line.screen_x as u32) as usize * 4;

                pixels[index_pixel + 0] = pixel[1];
                pixels[index_pixel + 1] = pixel[2];
                pixels[index_pixel + 2] = pixel[3];
                pixels[index_pixel + 3] = pixel[0];
            }
        }

        // list all textures
        // for t in 0..8 {
        //     let wall = self.code_to_texture(t as i32);

        //     for i in 0..TEXTURE_WIDTH as usize {
        //         for j in 0..TEXTURE_HEIGHT as usize {
        //             let bytes = wall[i * TEXTURE_WIDTH as usize + j].to_be_bytes();
        //             let index_pixel = (i * screen_size.width as usize + j + lines[0].y_start as usize * 640) * 4 + TEXTURE_WIDTH as usize * 4 * t;
    
        //             pixels[index_pixel + 0] = bytes[1];
        //             pixels[index_pixel + 1] = bytes[2];
        //             pixels[index_pixel + 2] = bytes[3];
        //             pixels[index_pixel + 3] = 255;
        //         }
        //     }
        // }
    }

    fn code_to_texture(&self, code: i32) -> &Vec<u32> {
        if code < 0 || code > WALL_TEXTURE_FLAT_GREY {
            panic!("No such code is associated with any texture!");
        }

        &self.walls[code as usize]
    }

    fn generate_textures() -> WallsTexture {
        let mut texture = vec![vec![0; (TEXTURE_WIDTH * TEXTURE_HEIGHT) as usize]; 8];

        for x in 0..TEXTURE_WIDTH {
            for y in 0..TEXTURE_HEIGHT {
                let xorcolor = (x * 256 / TEXTURE_WIDTH) ^ (y * 256 / TEXTURE_HEIGHT);
                let ycolor = y * 256 / TEXTURE_HEIGHT;
                let xycolor = y * 128 / TEXTURE_HEIGHT + x * 128 / TEXTURE_WIDTH;

                //flat red texture with black cross
                texture[0][(TEXTURE_WIDTH * y + x) as usize] = 65536 * 254 * if x != y && x != TEXTURE_WIDTH - y { 1 } else { 0 };
                //sloped greyscale
                texture[1][(TEXTURE_WIDTH * y + x) as usize] = xycolor + 256 * xycolor + 65536 * xycolor;
                //sloped yellow gradient
                texture[2][(TEXTURE_WIDTH * y + x) as usize] = 256 * xycolor + 65536 * xycolor;
                //xor greyscale
                texture[3][(TEXTURE_WIDTH * y + x) as usize] = xorcolor + 256 * xorcolor + 65536 * xorcolor;
                //xor green
                texture[4][(TEXTURE_WIDTH * y + x) as usize] = 256 * xorcolor;
                //red bricks
                texture[5][(TEXTURE_WIDTH * y + x) as usize] = 65536 * 192 * if x % 16 != 0 && y % 16 != 0 { 1 } else { 0 };
                //red gradient
                texture[6][(TEXTURE_WIDTH * y + x) as usize] = 65536 * ycolor;
                //flat grey texture
                texture[7][(TEXTURE_WIDTH * y + x) as usize] = 128 + 256 * 128 + 65536 * 128;
            }
        }

        texture
    }
}
