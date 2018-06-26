extern crate glium;
extern crate glium_graphics;
extern crate graphics;
extern crate piston;
extern crate tiled;

extern crate image;

// for integration with tiled
//use tiled::parse_file;

// for creating the window, textures, and rendering
use glium_graphics::{Flip, Glium2d, GliumWindow, OpenGL, Texture, TextureSettings};
use graphics::Transformed;
use piston::window::WindowSettings;

// for convenience
// renamed to avoid confusion
extern crate find_folder;
use graphics as piston_graphics;
use piston_graphics::DrawState;

// for handling events
use piston::event_loop::EventLoop;
use piston::input::{Button, CloseEvent, Key, PressEvent, RenderEvent, ResizeEvent};

mod camera;
use camera::{Camera, Tile};

mod map;
use map::Map;

fn main() {
    let opengl = OpenGL::V3_2;
    let (w, h) = (800, 800);

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    println!("{:?}", &assets);

    // this is also the factory
    let ref mut window: GliumWindow = WindowSettings::new("Uranusbound", [w, h])
        .exit_on_esc(true)
        .opengl(opengl)
        .resizable(false)
        .build()
        .unwrap();

    let mut g2d = Glium2d::new(opengl, window);

    let map = Map::load(assets.join("best_map_large.tmx"), window);

    let (win_width, win_height) = window.get_max_viewport_dimensions();

    let ref mut camera = Camera::load(80, 80, win_width as i32, win_height as i32);
    camera.tile_buffer_auto_reserve();

    let tile_img = piston_graphics::image::Image::new();

    // event loop
    'game_loop: while let Some(event) = window.next() {
        // render event
        if let Some(args) = event.render_args() {
            let mut target = window.draw();

            // get the dimensions so the render area scales with
            // window size

            g2d.draw(&mut target, args.viewport(), |context, frame| {
                // iter through rows of map texture
                piston_graphics::clear([0.0, 0.0, 0.0, 0.0], frame);

                let (c_x, c_y, c_x_max, c_y_max) = camera.get_rect();

                for (y, row) in map.get_map().layers[0]
                    .tiles
                    .iter()
                    .enumerate()
                    .filter(|(y, _)| c_y <= *y as i32 * 16 && *y as i32 * 16 <= c_y_max)
                {
                    for (x, &tile) in row
                        .iter()
                        .enumerate()
                        .filter(|(x, _)| c_x <= *x as i32 * 16 && *x as i32 * 16 <= c_x_max)
                    {
                        println!("X: {}, Y: {}, Tile: {}", x, y, tile);
                        // skip if tile is zero, we need to be one ahead of it
                        if tile == 0 {
                            continue;
                        }

                        let tile = tile - 1; // tiled counts from 1

                        /*  of the particular tile in the tilesheet
                            tile = tile index x_max is the width of the screen 
                            y_max is the height of the screen
                        */
                        let tex_rect = [
                            (tile as i32 % (map.get_width() / map.get_tile_width())
                                * map.get_tile_width()) as f64, // x coordinate
                            (tile as i32 / (map.get_width() / map.get_tile_height())
                                * map.get_tile_height()) as f64, // y coordinate
                            map.get_tile_width() as f64,
                            map.get_tile_height() as f64,
                        ];

                        // Converts to the cartesian plane
                        let trans = context.transform.trans(
                            x as f64 * map.get_tile_width() as f64 - camera.get_x() as f64,
                            y as f64 * map.get_tile_height() as f64 - camera.get_y() as f64,
                        );

                        tile_img.src_rect(tex_rect).draw(
                            map.get_tile_sheet(),
                            &DrawState::default(),
                            trans,
                            frame,
                        );
                    }
                }
            });

            // swaps the back buffer with the front buffer consuming the frame
            target.finish().unwrap();
        } // end render event

        if let Some(button) = event.press_args() {
            if let Button::Keyboard(key) = button {
                match key {
                    Key::A | Key::Left => {
                        let temp_pos = camera.get_x() - 16;
                        if temp_pos > 0 {
                            camera.pos_x(temp_pos);
                        } else {
                            camera.pos_x(0);
                        }
                    }
                    Key::D | Key::Right => {
                        let temp_pos = camera.get_x() + 16;
                        if temp_pos < camera.get_x_max() {
                            camera.pos_x(temp_pos);
                        } else {
                            camera.x_to_max()
                        }
                    }
                    Key::W | Key::Up => {
                        let temp_pos = camera.get_y() - 16;
                        if temp_pos > 0 {
                            camera.pos_y(temp_pos);
                        } else {
                            camera.pos_y(0);
                        }
                    }
                    Key::S | Key::Down => {
                        let temp_pos = camera.get_y() + 16;
                        if temp_pos <= camera.get_y_max() {
                            camera.pos_y(temp_pos);
                        } else {
                            camera.y_to_max();
                        }
                    }
                    _ => (),
                }
            } else {
                // this covers the mouse events that we will ignore
                ()
            }
        }

        if let Some(size) = event.resize_args() {
            camera.resize(size[0] as i32, size[1] as i32);
        }

        if let Some(_) = event.close_args() {
            // This is where save_game should probably be called
            println!("Game window was closed. Exiting!");
            break 'game_loop;
        }
    }
}
