use winit::keyboard::KeyCode;

// ============================================================
// KEYBOARD STATE
// ============================================================

#[derive(Default)]
pub struct KeyboardState {
    // ========================================================
    // PLAYER 1
    // ========================================================
    pub w: bool,
    pub s: bool,

    // ========================================================
    // PLAYER 2
    // ========================================================
    pub up: bool,
    pub down: bool,
}

impl KeyboardState {
    pub fn handle_keyboard(&mut self, key_code: KeyCode, pressed: bool) {
        match key_code {
            // ------------------------------------------------
            // PLAYER 1
            // ------------------------------------------------
            KeyCode::KeyW => {
                self.w = pressed;
            }

            KeyCode::KeyS => {
                self.s = pressed;
            }

            // ------------------------------------------------
            // PLAYER 2
            // ------------------------------------------------
            KeyCode::ArrowUp => {
                self.up = pressed;
            }

            KeyCode::ArrowDown => {
                self.down = pressed;
            }

            _ => {}
        }
    }
}
