//! The world map and others + tiles management

extern crate find_folder;
extern crate tiled;

use std::path::PathBuf;

// for the map
use glium_graphics::{Flip, GliumWindow, ImageSize, Texture, TextureSettings};
use tiled::parse_file;

// Defines a map in the game
pub struct Map {
    // Tiled Info
    map: tiled::Map,
    tile_set: tiled::Tileset,
    tile_width: i32,
    tile_height: i32,
    tile_sheet: Texture,

    // Dimensions
    width: i32,
    height: i32,
}

impl Map {
    // Getters
    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_tile_width(&self) -> i32 {
        self.tile_width
    }

    pub fn get_tile_height(&self) -> i32 {
        self.tile_height
    }

    pub fn get_map(&self) -> &tiled::Map {
        &self.map
    }

    pub fn get_tile_set(&self) -> &tiled::Tileset {
        &self.tile_set
    }

    pub fn get_tile_sheet(&self) -> &Texture {
        &self.tile_sheet
    }
}

impl Map {
    pub fn load(path: PathBuf, window: &mut GliumWindow) -> Map {
        let map = match parse_file(&path) {
            Ok(map) => map,
            Err(error) => panic!("{:?}", error),
        };

        let tile_set = map.get_tileset_by_gid(1).unwrap().clone();

        let tile_sheet = path.parent().unwrap().join(&tile_set.images[0].source);

        println!("{:?}", &tile_sheet);

        let tile_sheet =
            Texture::from_path(window, tile_sheet, Flip::None, &TextureSettings::new()).unwrap();

        let tile_width = tile_set.tile_width as i32;
        let tile_height = tile_set.tile_height as i32;

        let (width, height) = tile_sheet.get_size();
        let width = width as i32;
        let height = height as i32;

        Map {
            width,
            height,
            tile_width,
            tile_height,
            map,
            tile_set,
            tile_sheet,
        }
    }
}
