//! Represents a camera that keeps track of information regarding what is displayed
pub struct Camera {

    // x and y coordinates for determining the location on the map
    // these are representative of the bottom left corner of the SubImage
    pos_x : i32,
    pos_y: i32,
    width: i32,
    height: i32,

    // map borders
    x_max: i32,
    y_max: i32,

    // buffers 
    tile_buffer: Vec<([f64; 4], [f64; 4])>,
}

impl Camera {
    // Getters
    pub fn get_x(&self) -> i32 {
        self.pos_x
    }

    pub fn get_y(&self) -> i32 {
        self.pos_y
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_x_max(&self) -> i32 {
        self.x_max
    }

    pub fn get_y_max(&self) -> i32 {
        self.y_max
    }

    pub fn get_tile_buffer(&self) -> &Vec<([f64; 4], [f64; 4])> {
        &self.tile_buffer
    }

    pub fn get_mut_tile_buffer(&self) -> &mut Vec<([f64; 4], [f64; 4])> {
        &mut self.tile_buffer
    }

    pub fn new() -> Camera {
        Camera {
            pos_x: 0,
            pos_y: 0,
            width: 0,
            height: 0,
            x_max: 0,
            y_max: 0,
            tile_buffer: vec![],
        }
    }

}

impl Camera {
    /// sets pos_x
    pub fn pos_x(mut self, x: i32) -> Camera {
        self.pos_x = x;
        self
    }

    /// sets pos_y
    pub fn pos_y(mut self, y: i32) -> Camera {
        self.pos_y = y;
        self
    }

    /// sets width
    pub fn width(mut self, w: i32) -> Camera {
        self.width = w;
        self
    }

    /// sets height
    pub fn height(mut self, h: i32) -> Camera {
        self.height = h;
        self
    }

    /// calculates x_max
    pub fn x_max(mut self) -> Camera {
        self.x_max = self.pos_x + self.width;
        self
    }

    /// calculates y_max
    pub fn y_max(mut self) -> Camera {
        self.y_max = self.pos_y + self.height;
        self
    }

    /// clears the tile_buffer
    pub fn clear_tile_buffer(&mut self) {
        self.tile_buffer.clear();
    }

    /// reserves an estimated amount of space
    pub fn tile_buffer_auto_reserve(&mut self) {
        self.tile_buffer.reserve(
            ((self.width / 16) * (self.height / 16)) as usize
        )
    }

    /// reserves specific amounts of space in the buffer
    pub fn tile_buffer_reserve(&mut self, size: usize) {
        self.tile_buffer.reserve(size);
    }
}