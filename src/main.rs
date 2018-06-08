extern crate find_folder;
extern crate glium;
extern crate glium_graphics;
extern crate graphics;
extern crate piston;
extern crate tiled;

extern crate image;

use tiled::parse_file;

use glium_graphics::{Flip, Glium2d, GliumWindow, OpenGL, Texture, TextureSettings};

// renamed to avoid confusion
use graphics as piston_graphics;
use piston_graphics::{clear, color, DrawState, ImageSize, SourceRectangled, Transformed};

use piston::event_loop::EventLoop;
use piston::input::{Button, CloseEvent, Key, PressEvent, RenderEvent};
use piston::window::WindowSettings;

struct SubMap {
    // x and y coordinets for determining the location on the map
    // these are representitive of the bottom left corner of the SubImage
    pos_x: i32,
    pos_y: i32,

    // map borders
    x_max: i32,
    y_max: i32,
}

fn main() {
    let opengl = OpenGL::V3_2;
    let (w, h) = (480, 480);

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    // this is also the factory
    let ref mut window: GliumWindow = WindowSettings::new("Uranusbound", [w, h])
        .exit_on_esc(true)
        .opengl(opengl)
        .resizable(true)
        .build()
        .unwrap();

    window.set_lazy(true);

    let mut g2d = Glium2d::new(opengl, window);

    let map = parse_file(&assets.join("better_map.tmx")).unwrap();

    let tileset = map.get_tileset_by_gid(1).unwrap();
    let tile_width = tileset.tile_width;
    let tile_height = tileset.tile_height;

    /*     let tilesheet = assets.join(&tileset.images[0].source);
    let tilesheet =
        Texture::from_path(window, &tilesheet, Flip::None, &TextureSettings::new()).unwrap();
 */

    let tilesheet = assets.join(&tileset.images[0].source);

    // a dynamic image that we can use for type conversions
    let dynamic_map = image::open(&tilesheet).unwrap();

    let tilesheet =
        Texture::from_image(window, &dynamic_map.to_rgba(), &TextureSettings::new()).unwrap();

    let (map_x_max, map_y_max) = tilesheet.get_size();

    let layer: &tiled::Layer = &map.layers[0];

    let map_img = piston_graphics::Image::new();

    // maintains the SubImage of the map and its coordinets
    let mut sub_map = SubMap {
        pos_x: 160, // will be dynamic later
        pos_y: 160, // will be dynamic later
        x_max: map_x_max as i32,
        y_max: map_y_max as i32,
    };

    // event loop
    'game_loop: while let Some(event) = window.next() {
        // render event
        if let Some(args) = event.render_args() {
            let mut target = window.draw();

            // get the dimensions so the render area scales with
            // window size
            let (win_width, win_height) = window.get_max_viewport_dimensions();

            g2d.draw(&mut target, args.viewport(), |context, frame| {
                clear(color::BLACK, frame);

                //let mut rects = vec![];

                for (y, row) in layer.tiles.iter().enumerate().clone() {
                    for (x, &tile) in row.iter().enumerate() {
                        if tile == 0 {
                            continue;
                        }

                        let tile = tile - 1; // tiled counts from 1

                        /*  of the particular tile in the tilesheet
                            tile = tile index
                            map_x_max is the width of the map
                            map_y_max is the height of the map
                        */
                        let tile_rect = [
                            (tile % (map_x_max / tile_width) * tile_width) as f64, // x coordinate
                            (tile / (map_x_max / tile_height) * tile_height) as f64, // y coordinate
                            tile_width as f64,
                            tile_height as f64,
                        ];

                        if is_on_screen(
                            sub_map.pos_x,
                            sub_map.pos_y,
                            win_width as i32,
                            win_height as i32,
                            x as i32 * tile_width as i32,
                            y as i32 * tile_height as i32,
                        ) {
                            //println!("Tile at {} x {} should be drawn to screen", x, y);
                            // Converts to the cartesian plane
                            let trans = context
                                .transform
                                .trans(x as f64 * tile_width as f64, y as f64 * tile_height as f64);

                            map_img.src_rect(tile_rect).draw(
                                &tilesheet,
                                &DrawState::default(),
                                trans,
                                frame,
                            );
                        }
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
                        let temp_pos = sub_map.pos_x - 16;
                        if temp_pos > 0 {
                            sub_map.pos_x = temp_pos;
                        } else {
                            sub_map.pos_x = 0;
                        }
                    }
                    Key::D | Key::Right => {
                        let temp_pos = sub_map.pos_x + 16;
                        if temp_pos < sub_map.x_max {
                            sub_map.pos_x = temp_pos;
                        }
                    }
                    Key::W | Key::Up => {
                        let temp_pos = sub_map.pos_y - 16;
                        if temp_pos < sub_map.y_max {
                            sub_map.pos_y = temp_pos;
                        } else {
                            sub_map.pos_y = sub_map.y_max;
                        }
                    }
                    Key::S | Key::Down => {
                        let temp_pos = sub_map.pos_y + 16;
                        if temp_pos > 0 {
                            sub_map.pos_y = temp_pos;
                        } else {
                            sub_map.pos_y = 0;
                        }
                    }
                    _ => (),
                }
            } else {
                // this covers the mouse events that we will ignore
                ()
            }
        }

        if let Some(_) = event.close_args() {
            // This is where save_game should probably be called
            println!("Game window was closed. Exiting!");
            break 'game_loop;
        }
    }
}

/// Determins if a tile is on screen and should be drawn
fn is_on_screen(
    vp_x: i32,
    vp_y: i32,
    vp_width: i32,
    vp_height: i32,
    tile_x: i32,
    tile_y: i32,
) -> bool {
    if vp_x <= tile_x && tile_x <= (vp_x + vp_width) {
        if vp_y <= tile_y && tile_y <= (vp_y + vp_height) {
            return true;
        }
    }
    false
}
