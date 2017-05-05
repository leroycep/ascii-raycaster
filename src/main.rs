
#[macro_use]
extern crate glium;
extern crate vecmath as vm;
extern crate image;

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

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
    color: [f32; 3],
}

implement_vertex!(Vertex, position, tex_coords, color);

fn main() {
    use glium::DisplayBuild;
    let gl_request = glium::glutin::GlRequest::Specific(glium::glutin::Api::OpenGl, (2,1));
    let display = glium::glutin::WindowBuilder::new().with_gl(gl_request).build_glium().unwrap();

    use std::io::Cursor;
    let image = image::load(Cursor::new(&include_bytes!("../assets/Potash_10x10.png")[..]),
                           image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let tile_size = (image_dimensions.0 / 16, image_dimensions.1 / 16);
    let image = glium::texture::RawImage2d::from_raw_rgba(image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    let vertex_shader_src = r#"
        #version 120

        attribute vec2 position;
        attribute vec2 tex_coords;
        attribute vec3 color;

        varying vec2 v_tex_coords;
        varying vec3 v_color;

        void main() {
            v_tex_coords = tex_coords;
            v_color = color;
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 120

        varying vec2 v_tex_coords;
        varying vec3 v_color;

        uniform sampler2D tex;

        void main() {
            vec4 pixel = texture2D(tex, v_tex_coords);
            gl_FragColor = vec4(pixel.rgb * v_color.rgb, pixel.a);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut window_size = (1, 1);

    let mut shapes = vec![];
    let mut indices = vec![];

    let mut pos = [22.0, 1.6, 22.0];
    let mut pitch = 0.0f64;
    let mut yaw = 0.0f64;

    let mut grid = [[('.', [0.0; 3]); 120]; 60];
    let mut moved = true;

    loop {
        shapes.clear();
        indices.clear();
        for y in 0..(window_size.1/tile_size.1) {
            for x in 0..(window_size.0/tile_size.0) {
                let x0 = ((x * tile_size.0) as f32 / window_size.0 as f32) * 2.0 - 1.0;
                let x1 = (((x+1) * tile_size.0) as f32 / window_size.0 as f32) * 2.0 - 1.0;
                let y0 = ((y * tile_size.1) as f32 / window_size.1 as f32) * -2.0 + 1.0;
                let y1 = (((y+1) * tile_size.1) as f32 / window_size.1 as f32) * -2.0 + 1.0;
                let tex_tile_size = [tile_size.0 as f32 / image_dimensions.0 as f32, tile_size.1 as f32 / image_dimensions.1 as f32];

                let tile = if x < 120 && y < 60 { grid[y as usize][x as usize] } else { (' ',[0.0,0.0,0.0]) };
                let tile_index = tile.0 as u8;
                let color = tile.1;

                let tile_coords = [(tile_index % 16) as f32, ((tile_index >> 4)) as f32];
                let tx0 = tile_coords[0] * tex_tile_size[0];
                let tx1 = (tile_coords[0]+1.0) * tex_tile_size[0];
                let ty0 = tile_coords[1] * tex_tile_size[1];
                let ty1 = (tile_coords[1]+1.0) * tex_tile_size[1];
                let index = shapes.len() as u16;
                shapes.push(Vertex { position: [x0, y0], tex_coords: [tx0, ty0], color: color });
                shapes.push(Vertex { position: [x1, y0], tex_coords: [tx1, ty0], color: color });
                shapes.push(Vertex { position: [x0, y1], tex_coords: [tx0, ty1], color: color });
                shapes.push(Vertex { position: [x1, y1], tex_coords: [tx1, ty1], color: color });
                indices.extend_from_slice(&[index, index+1, index+2, index+3, index+1, index+2]);
            }
        }

        let vertex_buffer = glium::VertexBuffer::new(&display, &shapes).unwrap();
        let indices = glium::index::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

        use glium::Surface;
        let mut target = display.draw();
        let uniforms = uniform! {
            tex: texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
        };
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            use glium::glutin::{Event,ElementState,VirtualKeyCode};
            match ev {
                Event::Closed |
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => return,
                Event::Resized(w, h) => window_size = (w, h),
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::I)) => {
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
,
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::K)) => {
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

                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::L)) => {
                    pitch += TURN_SPEED;
                    moved = true;
                }
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::J)) => {
                    pitch -= TURN_SPEED;
                    moved = true;
                }
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::U)) => {
                    yaw -= TURN_SPEED;
                    moved = true;
                }
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::O)) => {
                    yaw += TURN_SPEED;
                    moved = true;
                }
                _ => ()
            }
        }

        if moved {
            draw(pos, pitch, yaw, &mut grid);
            moved = false;
        }
    }

    /*
    let mut draw_buffer = String::new();
    'MAIN: loop {
        thread::sleep(time::Duration::from_millis(20));
    }
    */
}

fn draw(pos: [f64; 3], pitch: f64, yaw: f64, grid: &mut [[(char, [f32; 3]); 120]; 60]) {
    let dir = [pitch.cos() * yaw.cos(), yaw.sin(), pitch.sin() * yaw.cos()];
    let right = [(pitch + (90.0f64).to_radians()).cos(), 0.0, (pitch + (90.0f64).to_radians()).sin()];
    let up = [0.0, -1.0, 0.0];
    for y in 0..DISPLAY_SIZE[1] as usize {
        for x in 0..DISPLAY_SIZE[0] as usize {
            let u = (x as f64 * 2.0 / DISPLAY_SIZE[0] as f64) - 1.0;
            let v = (y as f64 * 2.0 / DISPLAY_SIZE[1] as f64) - 1.0;
            let f = 1.97;

            let ray_origin = vm::vec3_add(pos, vm::vec3_add(vm::vec3_add(vm::vec3_scale(right, u), vm::vec3_scale(up, v)), vm::vec3_scale(dir, f)));
            let ray_dir = vm::vec3_sub(ray_origin, pos);
            let (tile, side) = raymarch(pos, ray_dir);
            let side = side as f32;
            grid[y][x] = match tile {
                1 => ('r', [1.0/side,0.02/side,0.02/side]),
                2 => ('g', [0.02/side,1.0/side,0.02/side]),
                3 => ('b', [0.02/side,0.02/side,1.0/side]),
                4 => ('w', [1.0/side,1.0/side,1.0/side]),
                5 => ('A', [1.0/side,0.02/side,1.0/side]),
                _ => (' ', [0.0/side,0.0/side,0.0/side]),
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
    let vec_parallel = vm::vec3_scale(axis, vm::vec3_dot(vec, axis) / vm::vec3_dot(axis, axis));
    let vec_perpendicular = vm::vec3_sub(vec, vec_parallel);
    let perpendicular_magnitude = vm::vec3_len(vec_perpendicular);

    /* Create a second axis so that we have a plane to rotate on */
    let w = vm::vec3_cross(axis, vec_perpendicular);
    let wx = angle.cos() / perpendicular_magnitude;
    let wy = angle.sin() / vm::vec3_len(w);

    let perpendicular_component = vm::vec3_scale(vm::vec3_add(vm::vec3_scale(vec_perpendicular, wx), vm::vec3_scale(w, wy)), perpendicular_magnitude);

    let rotated_vector = vm::vec3_add(perpendicular_component, vec_parallel);
    rotated_vector
}

