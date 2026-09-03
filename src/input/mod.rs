use winit::keyboard::KeyCode;

#[derive(Default)]
pub struct KeyboardState {
    // ========================================================
    // MOVEMENT
    // ========================================================
    pub w: bool,
    pub s: bool,
    pub a: bool,
    pub d: bool,

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
            // MOVEMENT
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
