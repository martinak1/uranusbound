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
use piston::input::RenderEvent;
use piston::window::WindowSettings;

struct Camera {
    // in pixels
    pos_x: i32,
    pos_y: i32,
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

    let (width, _) = tilesheet.get_size();

    let layer: &tiled::Layer = &map.layers[0];

    let map = image::Image::new();

    window.set_lazy(false);

    // event loop
    while let Some(event) = window.next() {
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
                            (tile % (width / tile_width) * tile_width) as f64,
                            (tile / (width / tile_height) * tile_height) as f64,
                            tile_width as f64,
                            tile_height as f64,
                        ];

                        // Converts to the cartesian plane
                        let trans = context
                            .transform
                            .trans(x as f64 * tile_width as f64, y as f64 * tile_height as f64);

                        // draws everything to the buffer

                        let (win_width, win_height) = window.get_max_viewport_dimensions();

                        // this line is how I got the camera to work
                        map.src_rect(map_rect).draw(
                            &tilesheet,
                            &DrawState::default().scissor([0, 0, win_width, win_height]),
                            trans,
                            gl,
                        );
                    }
                }
            });
            // swaps the back buffer with the front buffer consuming the frame
            target.finish().unwrap();
        }
    }
}
