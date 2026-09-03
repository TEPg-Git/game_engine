use crate::camera::Camera;
use crate::input::KeyboardState;
use crate::transform::Transform;

// ============================================================
// GAME STATE
// ============================================================

pub struct GameState {
    pub keyboard: KeyboardState,

    pub transform: Transform,

    pub camera: Camera,

    pub speed: f32,
}

// ============================================================
// IMPLEMENTATION
// ============================================================

impl GameState {
    pub fn new() -> Self {
        Self {
            keyboard: KeyboardState::default(),

            transform: Transform::new(),

            camera: Camera::new(),

            speed: 0.001,
        }
    }

    // ========================================================
    // UPDATE
    // ========================================================

    pub fn update(&mut self) {
        // ----------------------------------------------------
        // MOVEMENT
        // ----------------------------------------------------

        if self.keyboard.w {
            self.transform.position[1] += self.speed;
        }

        if self.keyboard.s {
            self.transform.position[1] -= self.speed;
        }

        if self.keyboard.a {
            self.transform.position[0] -= self.speed;
        }

        if self.keyboard.d {
            self.transform.position[0] += self.speed;
        }

        // ----------------------------------------------------
        // SPEED
        // ----------------------------------------------------

        if self.keyboard.i {
            self.speed += 0.0001;
        }

        if self.keyboard.o {
            self.speed -= 0.0001;
        }

        self.speed = self.speed.max(0.0001);

        // ----------------------------------------------------
        // ROTATION
        // ----------------------------------------------------

        if self.keyboard.q {
            self.transform.rotate(0.02);
        }

        if self.keyboard.e {
            self.transform.rotate(-0.02);
        }

        // ----------------------------------------------------
        // SCALE
        // ----------------------------------------------------

        if self.keyboard.z {
            self.transform.scale[0] -= 0.01;
            self.transform.scale[1] -= 0.01;
        }

        if self.keyboard.x {
            self.transform.scale[0] += 0.01;
            self.transform.scale[1] += 0.01;
        }

        // ----------------------------------------------------
        // SCALE LIMIT
        // ----------------------------------------------------

        self.transform.scale[0] = self.transform.scale[0].clamp(0.1, 3.0);

        self.transform.scale[1] = self.transform.scale[1].clamp(0.1, 3.0);

        // ========================================================
        // CAMERA MOVEMENT
        // ========================================================
        if self.keyboard.up {
            self.camera.translate(0.0, self.speed);
        }

        if self.keyboard.down {
            self.camera.translate(0.0, -self.speed);
        }

        if self.keyboard.left {
            self.camera.translate(-self.speed, 0.0);
        }

        if self.keyboard.right {
            self.camera.translate(self.speed, 0.0);
        }

        // ========================================================
        // CAMERA ZOOM
        // ========================================================
        if self.keyboard.zoom_in {
            self.camera.set_zoom(self.camera.zoom + 0.01);
        }

        if self.keyboard.zoom_out {
            self.camera.set_zoom(self.camera.zoom - 0.01);
        }
    }
}
