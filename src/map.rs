//! The world map and others + tiles management

use find_folder;
use std::fs::PathBuf;

// for the map
use tiled;
use glium_graphics::{Texture, TextureSettings};

// Defines a map in the game
pub struct Map {

    // assets
    assets: PathBuf,

    // Tiled Info
    map: Option<tiled::Map>,
    tile_set: Option<tiled::Tileset>,
    tile_width: i32,
    tile_height: i32,
    tile_sheet: Option<Texture>,

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

    pub fn get_map(&self) -> tiled::Map {
        self.map.unwrap()
    }

    pub fn get_tile_set(&self) -> &tiled::Tileset {
        &self.tile_set.unwrap()
    }

    pub fn get_tile_sheet(&self) -> &Texture {
        &self.tile_sheet.unwrap()
    }

    pub fn new() -> Map {
        let assets = search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();

        Map {
            assets,
            width: 0,
            height: 0,
            tile_width: 0,
            tile_height: 0,
            map: None,
            tile_set: None,
            tile_sheet: None,
        }
    }
}

impl Map {

    pub fn width(&mut self, w: i32) {
        self.width = w;
    }

    pub fn height(&mut self, h: i32) {
        self.height = h;
    }

    pub fn map(&mut self, path: PathBuf) {
        self.map = parse_file(path).unwrap();
    }

    pub fn tile_set(&mut self) {
        self.tile_set = self.map.get_tileset_by_gid(1);
    }

    pub fn tile_sheet(&mut self, window: &Window) {
        
        self.tile_sheet = Texture::from_path(
            window,
            self.assets.join(self.tile_set.image[0].source()),
            Flip::None,
            &TextureSettings::new()
        ).unwrap();
    }
}


