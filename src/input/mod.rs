use winit::keyboard::KeyCode;

#[derive(Default)]
pub struct KeyboardState {
    pub w: bool,
    pub s: bool,
    pub a: bool,
    pub d: bool,

    pub i: bool,
    pub o: bool,
}

impl KeyboardState {
    pub fn handle_keyboard(&mut self, key_code: KeyCode, pressed: bool) {
        match key_code {
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

            KeyCode::KeyI => {
                self.i = pressed;
            }

            KeyCode::KeyO => {
                self.o = pressed;
            }

            _ => {}
        }
    }
}
