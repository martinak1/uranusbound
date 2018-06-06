extern crate find_folder;
extern crate glium;
extern crate glium_graphics;
extern crate graphics;
extern crate piston;
extern crate tiled;

use tiled::*;

use glium_graphics::{Flip, Glium2d, GliumWindow, OpenGL, Texture, TextureSettings};

use graphics::*;

use piston::event_loop::EventLoop;
use piston::input::{Button, CloseEvent, Key, PressEvent, RenderEvent};
use piston::window::WindowSettings;

struct Pc {
    // x and y coordinets for determining the location on the map
    pos_x: i32,
    pos_y: i32,

    // map borders
    x_max: i32,
    x_min: i32,
    y_max: i32,
    y_min: i32,
}

fn main() {
    let opengl = OpenGL::V3_2;
    let (w, h) = (400, 400);

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let ref mut window: GliumWindow = WindowSettings::new("Uranusbound", [w, h])
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut g2d = Glium2d::new(opengl, window);

    let map = parse_file(&assets.join("better_map.tmx")).unwrap();

    let tileset = map.get_tileset_by_gid(1).unwrap();
    let tile_width = tileset.tile_width;
    let tile_height = tileset.tile_height;

    let tilesheet = assets.join(&tileset.images[0].source);
    let tilesheet =
        Texture::from_path(window, &tilesheet, Flip::None, &TextureSettings::new()).unwrap();

    let (xMax, yMax) = tilesheet.get_size();

    let layer: &tiled::Layer = &map.layers[0];

    let map = image::Image::new();

    window.set_lazy(false);

    let mut pc = Pc {
        pos_x: 200,
        pos_y: 200,
        x_max: xMax as i32,
        x_min: 0,
        y_max: yMax as i32,
        y_min: 0,
    };

    // event loop
    'game_loop: while let Some(event) = window.next() {
        // render event
        if let Some(args) = event.render_args() {
            let mut target = window.draw();
            g2d.draw(&mut target, args.viewport(), |context, gl| {
                clear(color::WHITE, gl);

                for (y, row) in layer.tiles.iter().enumerate().clone() {
                    for (x, &tile) in row.iter().enumerate() {
                        if tile == 0 {
                            continue;
                        }

                        let tile = tile - 1; // tiled counts from 1

                        // rect of the particular tile in the tilesheet
                        let map_rect = [
                            (tile % (xMax / tile_width) * tile_width) as f64,
                            (tile / (xMax / tile_height) * tile_height) as f64,
                            tile_width as f64,
                            tile_height as f64,
                        ];

                        // Converts to the cartesian plane
                        let trans = context
                            .transform
                            .trans(x as f64 * tile_width as f64, y as f64 * tile_height as f64);

                        // get the dimensions so the render area scales with
                        // window size
                        let (win_width, win_height) = window.get_max_viewport_dimensions();

                        // this line is how I got the camera to work
                        map.src_rect(map_rect).draw(
                            &tilesheet,
                            &DrawState::default().scissor([
                                pc.pos_x as u32,
                                pc.pos_y as u32,
                                win_width,
                                win_height,
                            ]),
                            trans,
                            gl,
                        );
                    }
                }
            });
            // swaps the back buffer with the front buffer consuming the frame
            target.finish().unwrap();
        }

        if let Some(button) = event.press_args() {
            if let Button::Keyboard(key) = button {
                match key {
                    Key::A | Key::Left => {
                        let temp_pos = pc.pos_x - 16;
                        if temp_pos > pc.x_min {
                            pc.pos_x = temp_pos;
                        } else {
                            pc.pos_x = pc.x_min;
                        }
                    }
                    Key::D | Key::Right => {
                        let temp_pos = pc.pos_x + 16;
                        if temp_pos < pc.x_max {
                            pc.pos_x = temp_pos;
                        }
                    }
                    Key::W | Key::Up => {
                        let temp_pos = pc.pos_y + 16;
                        if temp_pos < pc.y_max {
                            pc.pos_y = temp_pos;
                        } else {
                            pc.pos_y = pc.y_max;
                        }
                    }
                    Key::S | Key::Down => {
                        let temp_pos = pc.pos_y - 16;
                        if temp_pos > pc.y_min {
                            pc.pos_y = temp_pos;
                        } else {
                            pc.pos_y = temp_pos;
                        }
                    }
                    _ => (),
                }
            } else {
                ()
            }
        }

        if let Some(_) = event.close_args() {
            println!("Game window was closed. Exiting!");
            break 'game_loop;
        }
    }
}
