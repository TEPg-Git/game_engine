use crate::input::KeyboardState;

pub struct GameState {
    pub keyboard: KeyboardState,
    pub position: [f32; 2],
    pub speed: f32,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            keyboard: KeyboardState::default(),
            position: [0.0, 0.0],
            speed: 0.001,
        }
    }

    pub fn update(&mut self) {
        // ----------------------------------------------------
        // MOVEMENT
        // ----------------------------------------------------

        if self.keyboard.w {
            self.position[1] += self.speed;
        }

        if self.keyboard.s {
            self.position[1] -= self.speed;
        }

        if self.keyboard.a {
            self.position[0] -= self.speed;
        }

        if self.keyboard.d {
            self.position[0] += self.speed;
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

        self.position[0] = self.position[0].clamp(-1.0, 1.0);

        self.position[1] = self.position[1].clamp(-1.0, 1.0);
    }
}
