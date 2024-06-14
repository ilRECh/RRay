use ggez::winit::dpi::PhysicalSize;

use crate::{
    player::Player,
    world_map::WorldMap
};

pub struct Line {
    pub wall_code: i32,
    pub wall_side: u8,
    pub wall_x: f32,
    pub ray_dir_x: f32,
    pub ray_dir_y: f32,
    pub screen_x: f32,
    pub y_start: f32,
    pub y_end: f32
}

pub fn dda(screen_size: &PhysicalSize<u32>, player: &Player, world_map: &WorldMap) -> Vec<Line> {
    let mut lines = Vec::with_capacity(screen_size.width as usize);

    for x in 0..screen_size.width {
        let screen_x = 2.0 * (x as f32) / (screen_size.width as f32) - 1.0;
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
        let mut wall;

        loop {
            if side_dist_x < side_dist_y {
                side_dist_x += delta_dist_x;
                map_x += step_x;
                side = 0;
            } else {
                side_dist_y += delta_dist_y;
                map_y += step_y;
                side = 1;
            }

            wall = world_map.at(&mut map_x, &mut map_y);

            // todo: interact with player.
            // should be a proper call rather than stupid "wall != b'P'" check
            if wall > 0 && wall != b'P' as i32 {
                break;
            }
        };

        let perp_wall_dist = if side == 0 {
                side_dist_x - delta_dist_x
            } else {
                side_dist_y - delta_dist_y
            };

        // calculate height of line to draw on screen
        let line_height = screen_size.height as f32 / perp_wall_dist;

        //calculate lowest and highest pixel to fill in current stripe
        let (draw_start, draw_end) = (
            -1.0 * line_height / 2.0 + screen_size.height as f32 / 2.0,
            line_height / 2.0 + screen_size.height as f32 / 2.0
        );

        let mut wall_x = if side == 0 {
            player.position.y + perp_wall_dist * ray_dir_y
        } else {
            player.position.x + perp_wall_dist * ray_dir_x
        };

        wall_x -= wall_x.floor();

        lines.push(Line {
            wall_side: side,
            wall_code: wall - 1,
            wall_x,
            ray_dir_x,
            ray_dir_y,
            screen_x: x as f32,
            y_start: draw_start,
            y_end: draw_end
        });
    }

    lines
}
