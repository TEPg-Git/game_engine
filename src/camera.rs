// ============================================================
// CAMERA
// ============================================================

pub struct Camera {
    // Represents the camera in the game.
    // ========================================================
    // POSITION
    // ========================================================
    pub position: [f32; 2], // The position of the camera.

    // ========================================================
    // ZOOM
    // ========================================================
    pub zoom: f32, // The zoom level of the camera.
}

// ============================================================
// IMPLEMENTATION
// ============================================================

impl Camera {
    // ========================================================
    // DEFAULT CAMERA
    // ========================================================

    pub fn new() -> Self {
        // Creates a new camera with default values.
        Self {
            position: [0.0, 0.0],

            zoom: 1.0,
        }
    }

    // ========================================================
    // MOVE
    // ========================================================

    pub fn translate(&mut self, x: f32, y: f32) {
        // Translates the camera by the given amount.
        self.position[0] += x;

        self.position[1] += y;
    }

    // ========================================================
    // SET POSITION
    // ========================================================

    pub fn set_position(&mut self, x: f32, y: f32) {
        // Sets the position of the camera to the given values.
        self.position = [x, y];
    }

    // ========================================================
    // SET ZOOM
    // ========================================================

    pub fn set_zoom(&mut self, zoom: f32) {
        // Sets the zoom level of the camera to the given value.
        self.zoom = zoom.max(0.01); // Clamps the zoom level to a minimum of 0.01 to avoid division by zero.
    }
}
