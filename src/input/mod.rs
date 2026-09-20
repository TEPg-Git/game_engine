use winit::keyboard::KeyCode;

#[derive(Default)]
pub struct KeyboardState {
    //Represents the state of the keyboard.
    // ========================================================
    // OBJECT MOVEMENT
    // ========================================================
    pub w: bool, // Moves the object forward.
    pub s: bool, // Moves the object backward.
    pub a: bool, // Moves the object left.
    pub d: bool, // Moves the object right.

    // ========================================================
    // CAMERA MOVEMENT
    // ========================================================
    pub up: bool,    // Moves the camera up.
    pub down: bool,  // Moves the camera down.
    pub left: bool,  // Moves the camera left.
    pub right: bool, // Moves the camera right.

    // ========================================================
    // CAMERA ZOOM
    // ========================================================
    pub zoom_in: bool,  // Zooms in the camera.
    pub zoom_out: bool, // Zooms out the camera.

    // ========================================================
    // SPEED
    // ========================================================
    pub i: bool, // Increases the speed of the object.
    pub o: bool, // Decreases the speed of the object.

    // ========================================================
    // ROTATION
    // ========================================================
    pub q: bool, // Rotates the object counter-clockwise.
    pub e: bool, // Rotates the object clockwise.

    // ========================================================
    // SCALE
    // ========================================================
    pub z: bool, // Increases the scale of the object.
    pub x: bool, // Decreases the scale of the object.
}

impl KeyboardState {
    pub fn handle_keyboard(&mut self, key_code: KeyCode, pressed: bool) {
        match key_code {
            // ------------------------------------------------
            // OBJECT MOVEMENT
            // ------------------------------------------------
            KeyCode::KeyW => {
                self.w = pressed;
            }

            KeyCode::KeyS => {
                self.s = pressed;
            }

            KeyCode::KeyA => {
                self.a = pressed;
            }

            KeyCode::KeyD => {
                self.d = pressed;
            }

            // ------------------------------------------------
            // CAMERA MOVEMENT
            // ------------------------------------------------
            KeyCode::ArrowUp => {
                self.up = pressed;
            }

            KeyCode::ArrowDown => {
                self.down = pressed;
            }

            KeyCode::ArrowLeft => {
                self.left = pressed;
            }

            KeyCode::ArrowRight => {
                self.right = pressed;
            }

            // ------------------------------------------------
            // CAMERA ZOOM
            // ------------------------------------------------
            KeyCode::Equal => {
                self.zoom_in = pressed;
            }

            KeyCode::Minus => {
                self.zoom_out = pressed;
            }

            // ------------------------------------------------
            // SPEED
            // ------------------------------------------------
            KeyCode::KeyI => {
                self.i = pressed;
            }

            KeyCode::KeyO => {
                self.o = pressed;
            }

            // ------------------------------------------------
            // ROTATION
            // ------------------------------------------------
            KeyCode::KeyQ => {
                self.q = pressed;
            }

            KeyCode::KeyE => {
                self.e = pressed;
            }

            // ------------------------------------------------
            // SCALE
            // ------------------------------------------------
            KeyCode::KeyZ => {
                self.z = pressed;
            }

            KeyCode::KeyX => {
                self.x = pressed;
            }

            _ => {}
        }
    }
}
