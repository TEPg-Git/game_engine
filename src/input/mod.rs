use winit::keyboard::KeyCode;

#[derive(Default)]
pub struct KeyboardState {
    // ========================================================
    // OBJECT MOVEMENT
    // ========================================================
    pub w: bool,
    pub s: bool,
    pub a: bool,
    pub d: bool,

    // ========================================================
    // CAMERA MOVEMENT
    // ========================================================
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,

    // ========================================================
    // CAMERA ZOOM
    // ========================================================
    pub zoom_in: bool,
    pub zoom_out: bool,

    // ========================================================
    // SPEED
    // ========================================================
    pub i: bool,
    pub o: bool,

    // ========================================================
    // ROTATION
    // ========================================================
    pub q: bool,
    pub e: bool,

    // ========================================================
    // SCALE
    // ========================================================
    pub z: bool,
    pub x: bool,
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
