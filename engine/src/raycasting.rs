use ggez::winit::dpi::PhysicalSize;

use crate::{
    player::Player,
    world_map::WorldMap
};

pub struct LineWall {
    pub wall_code: i32,
    pub wall_side: u8,
    pub wall_x: f32,
    pub ray_dir_x: f32,
    pub ray_dir_y: f32,
    pub screen_x: f32,
    pub y_start: f32,
    pub y_end: f32
}

pub struct LineFloor {
    pub floor_step_x: f32,
    pub floor_step_y: f32,
    pub floor_x: f32,
    pub floor_y: f32,
    pub screen_y: f32
}

pub fn dda_walls(screen_size: &PhysicalSize<u32>, player: &Player, world_map: &WorldMap) -> Vec<LineWall> {
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

        lines.push(LineWall {
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

pub fn dda_floor(screen_size: &PhysicalSize<u32>, player: &Player) -> Vec<LineFloor> {
    let mut lines = Vec::with_capacity(screen_size.height as usize);

    for y in screen_size.height as i32 / 2 + 1..screen_size.height as i32 {
        let ray_dir_x0 = player.direction.x - player.camera.x;
        let ray_dir_y0 = player.direction.y - player.camera.y;
        let ray_dir_x1 = player.direction.x + player.camera.x;
        let ray_dir_y1 = player.direction.y + player.camera.y;

        // Current y position compared to the center of the screen (the horizon)
        let p = y - screen_size.height as i32 / 2;

        // Vertical position of the camera.
        let pos_z = 0.5 * screen_size.height as f32;

        // Horizontal distance from the camera to the floor for the current row.
        // 0.5 is the z position exactly in the middle between floor and ceiling.
        let row_distance = pos_z / p as f32;

        // calculate the real world step vector we have to add for each x (parallel to camera plane)
        // adding step by step avoids multiplications with a weight in the inner loop
        let floor_step_x = row_distance * (ray_dir_x1 - ray_dir_x0) / screen_size.width as f32;
        let floor_step_y = row_distance * (ray_dir_y1 - ray_dir_y0) / screen_size.width as f32;

        // real world coordinates of the leftmost column. This will be updated as we step to the right.
        let floor_x = player.position.x + row_distance * ray_dir_x0;
        let floor_y = player.position.y + row_distance * ray_dir_y0;

        lines.push(LineFloor {
            floor_step_x,
            floor_step_y,
            floor_x,
            floor_y,
            screen_y: y as f32
        });
    }

    lines
}
