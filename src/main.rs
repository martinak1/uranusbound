extern crate find_folder;
extern crate piston;
extern crate piston_window;
extern crate sdl2_window;

// multi-threading
//use std::process;
// build windows
//use piston::window::WindowSettings;
// handle the event loop
//use piston::event_loop::{EventSettings, Events};
// handle kb input
//use piston::input::{Button, Key, PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
// create the window
use piston_window::*;
// for sdl2 window
use sdl2_window::Sdl2Window;
// for loading textures
//use texture::*;

fn main() {
    //let opengl = OpenGL::V3_2;
    //let (width, height) = (300, 300);
    /*
    let mut window: PistonWindow = WindowSettings::new("onett: sprite", [300; 2])
        .exit_on_esc(true)
        .fullscreen(false)
        .opengl(opengl);
        .build()
        .unwrap();
        */

    let mut window: PistonWindow<Sdl2Window> = WindowSettings::new("Uranusbound", [500; 2])
        .exit_on_esc(true)
        .build()
        .unwrap();

    println!("Window was built");

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    // Loads the bg_img of Onett
    let bg_img = assets.join("onett.png");
    let bg_img: G2dTexture = Texture::from_path(
        &mut window.factory,
        &bg_img,
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    /*
    let pc_img = assets.join("pc.png");
    let pc_img: G2dTexture = Texture::from_path(
        &mut window.factory,
        &pc_img,
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();
    */

    window.set_lazy(false);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            image(&bg_img, c.transform, g);
            //image(&pc_img, c.transform, g);
        });
    }
}
