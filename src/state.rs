use crate::input::KeyboardState;
use crate::transform::Transform;

// ============================================================
// GAME STATE
// ============================================================

pub struct GameState {
    pub keyboard: KeyboardState,

    pub transform: Transform,

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
        // SCREEN LIMIT
        // ----------------------------------------------------

        self.transform.position[0] = self.transform.position[0].clamp(-1.0, 1.0);

        self.transform.position[1] = self.transform.position[1].clamp(-1.0, 1.0);

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
    }
}
