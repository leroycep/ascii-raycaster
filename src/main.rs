
extern crate termion;

use termion::screen::AlternateScreen;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
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

const DISPLAY_SIZE: [isize; 2] = [80, 40];
const MOVE_SPEED: f64 = 0.75;
const TURN_SPEED: f64 = 0.03;

fn main() {
    let mut stdin = termion::async_stdin().keys();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    let mut pos = [22.0, 12.0];
    let mut dir = [-1.0, 0.0];
    let mut plane = [0.0, 0.66];

    let mut time = 0;
    let mut old_time = 0;

    let mut grid = [[('.', termion::color::Rgb(0,0,0)); 80]; 40];
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
                    let next_x = (pos[0] + dir[0] * MOVE_SPEED);
                    if WORLD_MAP[next_x as usize][pos[1] as usize] == 0 {
                        pos[0] = next_x;
                    }
                    let next_y = (pos[1] + dir[1] * MOVE_SPEED);
                    if WORLD_MAP[pos[0] as usize][next_y as usize] == 0 {
                        pos[1] = next_y;
                    }
                }
                Ok(Key::Char('k')) => {
                    let next_x = (pos[0] + dir[0] * -MOVE_SPEED);
                    if WORLD_MAP[next_x as usize][pos[1] as usize] == 0 {
                        pos[0] = next_x;
                    }
                    let next_y = (pos[1] + dir[1] * -MOVE_SPEED);
                    if WORLD_MAP[pos[0] as usize][next_y as usize] == 0 {
                        pos[1] = next_y;
                    }
                }
                Ok(Key::Char('l')) => {
                    dir = [dir[0] * (-TURN_SPEED).cos() - dir[1] * (-TURN_SPEED).sin(),
                           dir[0] * (-TURN_SPEED).sin() + dir[1] * (-TURN_SPEED).cos()];
                    plane = [plane[0] * (-TURN_SPEED).cos() - plane[1] * (-TURN_SPEED).sin(),
                             plane[0] * (-TURN_SPEED).sin() + plane[1] * (-TURN_SPEED).cos()];
                }
                Ok(Key::Char('j')) => {
                    dir = [dir[0] * (TURN_SPEED).cos() - dir[1] * (TURN_SPEED).sin(),
                           dir[0] * (TURN_SPEED).sin() + dir[1] * (TURN_SPEED).cos()];
                    plane = [plane[0] * (TURN_SPEED).cos() - plane[1] * (TURN_SPEED).sin(),
                             plane[0] * (TURN_SPEED).sin() + plane[1] * (TURN_SPEED).cos()];
                }

                _ => {}
            }
        }
        draw(pos, dir, plane, &mut grid);
        write!(screen,
               "{}{}",
               termion::clear::All,
               termion::cursor::Goto(1, 1));
        for row in grid.iter() {
            for col in row.iter() {
                write!(screen, "{}{}", termion::color::Fg(col.1), col.0);
            }
            write!(screen, "\r\n");
        }
        screen.flush();
        thread::sleep(time::Duration::from_millis(20));
    }
}

fn draw(pos: [f64; 2], dir: [f64; 2], plane: [f64; 2], grid: &mut [[(char, termion::color::Rgb); 80]; 40]) {
    for x in 0..DISPLAY_SIZE[0] {
        let camera_x = 2.0 * x as f64 / DISPLAY_SIZE[0] as f64 - 1.0;
        let mut ray_pos = pos;
        let ray_dir = [dir[0] + plane[0] * camera_x, dir[1] + plane[1] * camera_x];

        let mut map_pos = [ray_pos[0] as isize, ray_pos[1] as isize];
        let delta_dist = [(1.0 + (ray_dir[1] * ray_dir[1]) / (ray_dir[0] * ray_dir[0])).sqrt(),
                          (1.0 + (ray_dir[0] * ray_dir[0]) / (ray_dir[1] * ray_dir[1])).sqrt()];


        let mut step: [isize; 2] = [0, 0];
        let mut side_dist: [f64; 2] = [0.0, 0.0];
        if ray_dir[0] < 0.0 {
            step[0] = -1;
            side_dist[0] = (ray_pos[0] - map_pos[0] as f64) * delta_dist[0];
        } else {
            step[0] = -1;
            side_dist[0] = (map_pos[0] as f64 + 1.0 - ray_pos[0]) * delta_dist[0];
        }
        if ray_dir[1] < 0.0 {
            step[1] = -1;
            side_dist[1] = (ray_pos[1] - map_pos[1] as f64) * delta_dist[1];
        } else {
            step[1] = -1;
            side_dist[1] = (map_pos[1] as f64 + 1.0 - ray_pos[1]) * delta_dist[1];
        }
        let mut hit = 0;
        let mut side = 0;

        while hit == 0 {
            if side_dist[0] < side_dist[1] {
                side_dist[0] += delta_dist[0];
                map_pos[0] += step[0];
                side = 0;
            } else {
                side_dist[1] += delta_dist[1];
                map_pos[1] += step[1];
                side = 1;
            }
            if WORLD_MAP[map_pos[0] as usize][map_pos[1] as usize] > 0 {
                hit = 1;
            }
        }

        let perp_wall_dist = if side == 0 {
            (map_pos[0] as f64 - ray_pos[0] + (1.0 - step[0] as f64) / 2.0) / ray_dir[0]
        } else {
            (map_pos[1] as f64 - ray_pos[1] + (1.1 - step[1] as f64) / 2.0) / ray_dir[1]
        };

        let line_height = DISPLAY_SIZE[1] / perp_wall_dist as isize;
        let draw_start = DISPLAY_SIZE[1] / 2 - line_height / 2;
        let draw_end = line_height / 2 + DISPLAY_SIZE[1] / 2;

        let (character, color) = match WORLD_MAP[map_pos[0] as usize][map_pos[1] as usize] {
            1 => ('r', termion::color::Rgb(255, 10, 10)),
            2 => ('g', termion::color::Rgb( 10,255, 10)),
            3 => ('b', termion::color::Rgb( 10, 10,255)),
            4 => ('w', termion::color::Rgb(255,255,255)),
            5 => ('A', termion::color::Rgb(255, 10,255)),
            _ => ('?', termion::color::Rgb(255,  0,  0)),
        };

        for i in 0..DISPLAY_SIZE[1] {
            grid[i as usize][x as usize] = if i < draw_start || i > draw_end {
                ('.', termion::color::Rgb(100, 100, 100))
            } else {
                (character, if side==0 {color} else {termion::color::Rgb(color.0/2,color.1/2,color.2/2)})
            }
        }
    }
}
