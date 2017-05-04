
extern crate termion;
extern crate vecmath as vm;

use termion::screen::AlternateScreen;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stderr};
use std::{time, thread};

const WORLD_MAP: [[u8; 24]; 24] =
    [[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
     [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
     [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
     [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
     [1, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1],
     [1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
     [1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 1],
     [1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
     [1, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1],
     [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
     [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
     [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
     [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
     [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
     [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
     [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
     [1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
     [1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
     [1, 4, 0, 0, 0, 0, 5, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
     [1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
     [1, 4, 0, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
     [1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
     [1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
     [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]];

const DISPLAY_SIZE: [isize; 2] = [120, 60];
const MOVE_SPEED: f64 = 0.75;
const TURN_SPEED: f64 = 0.03;

fn main() {
    let mut stdin = termion::async_stdin().keys();
    let mut stderr = stderr();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    let mut pos = [22.0, 1.6, 22.0];
    let mut pitch = 0.0f64;
    let mut yaw = 0.0f64;

    let mut grid = [[('.', termion::color::Rgb(0,0,0)); 120]; 60];
    let mut draw_buffer = String::new();
    let mut moved = true;
    'MAIN: loop {
        loop {
            let next = stdin.next();
            if !next.is_some() {
                break;
            }
            let key = next.unwrap();
            match key {
                Ok(Key::Char('q')) => break 'MAIN,
                Ok(Key::Char('i')) => {
                    let next_x = pos[0] + MOVE_SPEED * pitch.cos();
                    if get_tile_at_pos([next_x, pos[1], pos[2]]) == 0 {
                        pos[0] = next_x;
                    }
                    let next_z = pos[2] + MOVE_SPEED * pitch.sin();
                    if get_tile_at_pos([pos[0], pos[1], next_x]) == 0 {
                        pos[2] = next_z;
                    }
                    moved = true;
                }
                Ok(Key::Char('k')) => {
                    let next_x = pos[0] + -MOVE_SPEED * pitch.cos();
                    if get_tile_at_pos([next_x, pos[1], pos[2]]) == 0 {
                        pos[0] = next_x;
                    }
                    let next_z = pos[2] + -MOVE_SPEED * pitch.sin();
                    if get_tile_at_pos([pos[0], pos[1], next_x]) == 0 {
                        pos[2] = next_z;
                    }
                    moved = true;
                }
                Ok(Key::Char('l')) => {
                    pitch += TURN_SPEED;
                    moved = true;
                }
                Ok(Key::Char('j')) => {
                    pitch -= TURN_SPEED;
                    moved = true;
                }
                Ok(Key::Char('u')) => {
                    yaw -= TURN_SPEED;
                    moved = true;
                }
                Ok(Key::Char('o')) => {
                    yaw += TURN_SPEED;
                    moved = true;
                }
                _ => {}
            }
        }
        if moved {
            draw(pos, pitch, yaw, &mut grid);
            draw_buffer.clear();
            use std::fmt::Write;
            for row in grid.iter() {
                for col in row.iter() {
                    let _ = write!(draw_buffer, "{}{}", termion::color::Fg(col.1), col.0);
                }
                let _ = write!(draw_buffer, "\r\n");
            }
            let _ = write!(screen,
                   "{}{}",
                   //termion::clear::All,
                   termion::cursor::Goto(1, 1), draw_buffer);
            let _ = screen.flush();
            moved = false;
        }
        thread::sleep(time::Duration::from_millis(20));
    }
}

fn draw(pos: [f64; 3], pitch: f64, yaw: f64, grid: &mut [[(char, termion::color::Rgb); 120]; 60]) {
    let dir = [pitch.cos() * yaw.cos(), yaw.sin(), pitch.sin() * yaw.cos()];
    let right = [(pitch + (90.0f64).to_radians()).cos(), 0.0, (pitch + (90.0f64).to_radians()).sin()];
    let up = [0.0, -1.0, 0.0];
    let mut stderr = stderr();
    writeln!(stderr, "dir: {:?}", dir);
    for y in 0..DISPLAY_SIZE[1] as usize {
        for x in 0..DISPLAY_SIZE[0] as usize {
            let u = (x as f64 * 2.0 / DISPLAY_SIZE[0] as f64) - 1.0;
            let v = (y as f64 * 2.0 / DISPLAY_SIZE[1] as f64) - 1.0;
            let f = 1.97;

            let ray_origin = vm::vec3_add(pos, vm::vec3_add(vm::vec3_add(vm::vec3_scale(right, u), vm::vec3_scale(up, v)), vm::vec3_scale(dir, f)));
            let ray_dir = vm::vec3_sub(ray_origin, pos);
            let (tile, side) = raymarch(pos, ray_dir);
            //writeln!(stderr, "up: {:?}, right: {:?}, forward: {:?}", up, right, dir);
            grid[y][x] = match tile {
                1 => ('r', termion::color::Rgb(255/side, 10/side, 10/side)),
                2 => ('g', termion::color::Rgb( 10/side,255/side, 10/side)),
                3 => ('b', termion::color::Rgb( 10/side, 10/side,255/side)),
                4 => ('w', termion::color::Rgb(255/side,255/side,255/side)),
                5 => ('A', termion::color::Rgb(255/side, 10/side,255/side)),
                _ => (' ', termion::color::Rgb(  0/side,  0/side,  0/side)),
            }
        }
    }
}

fn raymarch(pos: [f64; 3], dir: [f64; 3]) -> (u8, u8) {
    let mut map_pos = [pos[0].round(), pos[1].round(), pos[2].round()];
    let dir2 = [dir[0]*dir[0], dir[1]*dir[1], dir[2]*dir[2]];
    let delta_dist = [(1.0             + dir2[1]/dir2[0] + dir2[2]/dir2[0]).sqrt(),
                      (dir2[0]/dir2[1] + 1.0             + dir2[2]/dir2[1]).sqrt(),
                      (dir2[0]/dir2[2] + dir2[1]/dir2[2] + 1.0            ).sqrt(),
    ];
    let mut step = [0.0, 0.0, 0.0];
    let mut side_dist = [0.0, 0.0, 0.0];
    let mut side;
    for i in 0..3 {
        if dir[i] < 0.0 {
            step[i] = -1.0;
            side_dist[i] = (pos[i] - map_pos[i]) * delta_dist[i];
        } else {
            step[i] = 1.0;
            side_dist[i] = (map_pos[i] + 1.0 - pos[i]) * delta_dist[i];
        }
    }
    for _ in 0..50 {
        if side_dist[0] < side_dist[1] && side_dist[0] < side_dist[2] {
            side_dist[0] += delta_dist[0];
            map_pos[0] += step[0];
            side = 1;
        } else if side_dist[1] < side_dist[2] {
            side_dist[1] += delta_dist[1];
            map_pos[1] += step[1];
            side = 3;
        } else {
            side_dist[2] += delta_dist[2];
            map_pos[2] += step[2];
            side = 2;
        }
        let tile = get_tile_at_pos([map_pos[0], map_pos[1], map_pos[2]]);
        if tile > 0 {
            return (tile, side);
        }
    }
    return (0, 1);
}

fn get_tile_at_pos(pos: [f64; 3]) -> u8 {
    // Y is up
    for i in 0..3 {
        if pos[i] < 0.0 || pos[i] >= 24.0 {
            return 1;
        }
    }
    let (x, y, z) = (pos[0].floor() as usize, pos[1].floor() as usize, pos[2].floor() as usize);
    let tile2d = WORLD_MAP[x][z];
    match tile2d {
        //1 => if y <= 1 { 1 } else { 0 },
        2 => if y <= 2 { 2 } else { 0 },
        3 => if y <= 3 { 3 } else { 0 },
        4 => if y <= 1 { 4 } else { 0 },
        5 => if y <= 5 { 5 } else { 0 },
        id => id,
    }
}

fn rotate_y(dir: &[f64; 3], angle: f64) -> [f64; 3] {
    [
        dir[0] * angle.cos() + dir[2] * angle.sin(),
        dir[1],
        - dir[0] * angle.sin() + dir[2] * angle.cos(),
    ]
}

fn rotate_vec_axis(vec: [f64; 3], axis: [f64; 3], angle: f64) -> [f64; 3] {
    let mut stderr = stderr();
    let vec_parallel = vm::vec3_scale(axis, vm::vec3_dot(vec, axis) / vm::vec3_dot(axis, axis));
    let vec_perpendicular = vm::vec3_sub(vec, vec_parallel);
    let perpendicular_magnitude = vm::vec3_len(vec_perpendicular);
    let _ = writeln!(stderr, "v||: {:?}, vT: {:?}, ||vT||: {:?}", vec_parallel, vec_perpendicular, perpendicular_magnitude);

    /* Create a second axis so that we have a plane to rotate on */
    let w = vm::vec3_cross(axis, vec_perpendicular);
    let wx = angle.cos() / perpendicular_magnitude;
    let wy = angle.sin() / vm::vec3_len(w);

    let perpendicular_component = vm::vec3_scale(vm::vec3_add(vm::vec3_scale(vec_perpendicular, wx), vm::vec3_scale(w, wy)), perpendicular_magnitude);

    let rotated_vector = vm::vec3_add(perpendicular_component, vec_parallel);
    rotated_vector
}

