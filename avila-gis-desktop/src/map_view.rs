//! Map View Component

pub struct MapView {
    // Viewport
    center_x: f64,
    center_y: f64,
    zoom_level: u8,
    scale: f64,

    // Map extent
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,

    // Rendering
    width: f32,
    height: f32,
}

impl MapView {
    pub fn new() -> Self {
        Self {
            center_x: 0.0,
            center_y: 0.0,
            zoom_level: 5,
            scale: 1.0,

            min_x: -180.0,
            min_y: -90.0,
            max_x: 180.0,
            max_y: 90.0,

            width: 800.0,
            height: 600.0,
        }
    }

    pub fn zoom_in(&mut self) {
        if self.zoom_level < 20 {
            self.zoom_level += 1;
            self.scale *= 2.0;
        }
    }

    pub fn zoom_out(&mut self) {
        if self.zoom_level > 0 {
            self.zoom_level -= 1;
            self.scale /= 2.0;
        }
    }

    pub fn zoom_to_extent(&mut self) {
        self.center_x = (self.min_x + self.max_x) / 2.0;
        self.center_y = (self.min_y + self.max_y) / 2.0;
        self.zoom_level = 5;
        self.scale = 1.0;
    }

    pub fn pan(&mut self, dx: f32, dy: f32) {
        self.center_x += dx as f64 / self.scale;
        self.center_y += dy as f64 / self.scale;
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    pub fn zoom_level(&self) -> u8 {
        self.zoom_level
    }

    pub fn center(&self) -> (f64, f64) {
        (self.center_x, self.center_y)
    }

    pub fn size(&self) -> (f32, f32) {
        (self.width, self.height)
    }
}
