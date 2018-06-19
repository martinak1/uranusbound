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
use piston::window::WindowSettings;

// for convenience
// renamed to avoid confusion
extern crate find_folder;
use graphics as piston_graphics;
use piston_graphics::DrawState;

// for handling events
use piston::event_loop::EventLoop;
use piston::input::{Button, CloseEvent, Key, PressEvent, RenderEvent};

mod camera;
use camera::Camera;

mod map;
use map::Map;

fn main() {
    let opengl = OpenGL::V3_2;
    let (w, h) = (800, 800);

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

    let mut map = Map::new()
        .map(&assets.join("better_map.tmx"))
        .tileset()
        .tile_width()
        .tile_height()
        .tile_sheet(&window);

/*     let map: Map = {
        let map = parse_file(&assets.join("better_map.tmx")).unwrap();

        let tileset = map.get_tileset_by_gid(1).unwrap();
        let tile_width = tileset.tile_width as i32;
        let tile_height = tileset.tile_height as i32;


        let tilesheet = assets.join(&tileset.images[0].source);
        let tilesheet =
            Texture::from_path(window, &tilesheet, Flip::None, &TextureSettings::new()).unwrap();

        let (width, height) = tilesheet.get_size();

        Map {
            width: width as i32,
            height: height as i32,
            tile_width: tile_width as i32,
            tile_height: tile_height as i32,
            map,
            tileset,
            tilesheet,
        }
    }; */
    //let tilesheet = assets.join(&tileset.images[0].source);

    // a dynamic image that we can use for type conversions
    //let map_img = image::open(&tilesheet).unwrap();

    //let tilesheet =
        //Texture::from_image(window, &map_img.to_rgba(), &TextureSettings::new()).unwrap();

    //let (map_x_max, map_y_max) = map.get_tilesheet().get_size();


    //let map_img = piston_graphics::Image::new();

    let (win_width, win_height) = window.get_max_viewport_dimensions();

    // maintains the SubImage of the map and its coordinates
    //let mut sub_map = SubMap {
        //pos_x: 0, // will be dynamic later
        //pos_y: 0, // will be dynamic later
        //x_max: win_width as i32,
        //y_max: win_height as i32,
    //};

    //let layer: &tiled::Layer = &map.layers[0];

    let mut camera: Camera = Camera::new()
        .width(win_width as i32)
        .height(win_height as i32)
        .x_max()
        .y_max()
        .tile_buffer_auto_reserve();

    

    // event loop
    'game_loop: while let Some(event) = window.next() {
        // render event
        if let Some(args) = event.render_args() {
            let mut target = window.draw();

            // get the dimensions so the render area scales with
            // window size

            g2d.draw(&mut target, args.viewport(), |context, frame| {
                //clear(color::BLACK, frame);

                let cam_pos_x = camera.get_pos_x();
                let cam_pos_y = camera.get_pos_y();

                // iter through rows of map texture
                for (y, row) in map.get_map().layers[0].tiles.iter().enumerate() {

                    // bounds checking
                    match y {
                        0 .. camera.get_pos_y() => continue,
                        _ => break,
                        camera.get_pos_y() ..= camera.get_y_max() => {
                            // iter through columns of map texture
                            for (x, &tile) in row.iter().enumerate() {


                                match x {
                                    0 .. camera.get_pos_x() => continue,
                                    _ => break, 
                                    camera.get_pos_x() ..= camera.get_y_max() => {
                           
                                        // skip if tile is zero, we need to be one ahead of it
                                    if tile == 0 {
                                        continue;
                                    }

                                    let tile = tile - 1; // tiled counts from 1

                                    /*  of the particular tile in the tilesheet
                                        tile = tile index x_max is the width of the screen 
                                        y_max is the height of the screen
                                    */

                                    let src_rect = [
                                        (tile % (camera.get_x_max() / map.get_tile_width()) * map.get_tile_width()) as f64, // x coordinate
                                        (tile / (camera.get_y_max() / map.get_tile_height()) * map.get_tile_height()) as f64, // y coordinate
                                        map.get_tile_width() as f64,
                                        map.get_tile_height() as f64,
                                    ];

                                    let rect = [
                                        x as f64 * map.get_tile_width() as f64 - camera.get_pos_x() as f64,
                                        y as f64 * map.get_tile_height() as f64 - camera.get_pos_y() as f64,
                                        map.get_tile_width() as f64,
                                        map.get_tile_height() as f64
                                    ];

                                    tile_buffer.push((rect, src_rect));

/*
                        // Converts to the cartesian plane
                        let trans = context.transform.trans(
                            x as f64 * tile_width as f64 - sub_map.pos_x as f64,
                            y as f64 * tile_height as f64 - sub_map.pos_y as f64,
                        );

                        map_img.src_rect(tile_rect).draw(
                            &tilesheet,
                            &DrawState::default(),
                            trans,
                            frame,
                            );
*/
                                    }
                                }
                            }
                        }
                    }
                }

                // Push the tile buffer to the gpu
                piston_graphics::image::draw_many(
                    camera.get_tile_buffer().as_slice(), 
                    [0.0, 0.0, 0.0, 0.0],
                    map.get_tilesheet(), 
                    &DrawState::default(), 
                    context.transform, 
                    frame
                );
            });
            // swaps the back buffer with the front buffer consuming the frame
            target.finish().unwrap();

            // cleanup
            camera.clear_tile_buffer();
        }

        if let Some(button) = event.press_args() {
            if let Button::Keyboard(key) = button {
                match key {
                    Key::A | Key::Left => {
                        let temp_pos = camera.get_pos_x() - 1;
                        if temp_pos > 0 {
                            camera.pos_x(temp_pos);
                        } else {
                            camera.pos_x(0);
                        }
                    }
                    Key::D | Key::Right => {
                        let temp_pos = camera.get_pos_x() + 1;
                        if temp_pos < camera.get_x_max() {
                            camera.pos_x(temp_pos);
                        } else {
                            camera.pos_x(camera.get_x_max());
                        }
                    }
                    Key::W | Key::Up => {
                        let temp_pos = camera.get_pos_y() - 1;
                        if temp_pos > 0 {
                            camera.pos_y(temp_pos);
                        } else {
                            camera.pos_y(0);
                        }
                    }
                    Key::S | Key::Down => {
                        let temp_pos = camera.get_pos_y() + 1;
                        if temp_pos <= camera.get_y_max() {
                            camera.pos_y(temp_pos);
                        } else {
                            camera.pos_y(camera.get_y_max());
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