use crate::camera::Camera;
use crate::entity::Entity;
use crate::input::KeyboardState;
use crate::text::Text;

// ============================================================
// GAME STATE
// ============================================================

pub struct GameState {
    // ========================================================
    // INPUT
    // ========================================================
    pub keyboard: KeyboardState,

    // ========================================================
    // ENTITIES
    // ========================================================
    pub entities: Vec<Entity>,

    // ========================================================
    // CAMERA
    // ========================================================
    pub camera: Camera,

    // ========================================================
    // MOVEMENT SPEED
    // ========================================================
    pub speed: f32,

    // ========================================================
    // NEXT ENTITY ID
    // ========================================================
    next_entity_id: u32,

    // ========================================================
    // TEXT
    // ========================================================
    pub text: Text,
}

// ============================================================
// IMPLEMENTATION
// ============================================================

impl GameState {
    // ========================================================
    // CREATE GAME STATE
    // ========================================================

    pub fn new() -> Self {
        let mut game_state = Self {
            keyboard: KeyboardState::default(),

            entities: Vec::new(),

            camera: Camera::new(),

            speed: 0.001,

            next_entity_id: 0,

            text: Text::new("Hello East Engine", 24.0),
        };

        // ========================================================
        // PLAYER
        // ========================================================

        game_state.create_entity("Player");

        game_state
    }

    // ========================================================
    // CREATE ENTITY
    // ========================================================

    pub fn create_entity(&mut self, name: &str) -> u32 {
        let id = self.next_entity_id;

        self.next_entity_id += 1;

        let entity = Entity::new(id, name);

        self.entities.push(entity);

        id
    }

    // ========================================================
    // GET ENTITY
    // ========================================================

    pub fn get_entity(&self, id: u32) -> Option<&Entity> {
        self.entities.iter().find(|entity| entity.id == id)
    }

    // ========================================================
    // GET ENTITY MUTABLY
    // ========================================================

    pub fn get_entity_mut(&mut self, id: u32) -> Option<&mut Entity> {
        self.entities.iter_mut().find(|entity| entity.id == id)
    }

    // ========================================================
    // UPDATE
    // ========================================================

    pub fn update(&mut self) {
        // ----------------------------------------------------
        // PLAYER
        // ----------------------------------------------------

        if let Some(player) = self
            .entities
            .iter_mut()
            .find(|entity| entity.name == "Player")
        {
            // ------------------------------------------------
            // MOVEMENT
            // ------------------------------------------------

            if self.keyboard.w {
                player.translate(0.0, self.speed);
            }

            if self.keyboard.s {
                player.translate(0.0, -self.speed);
            }

            if self.keyboard.a {
                player.translate(-self.speed, 0.0);
            }

            if self.keyboard.d {
                player.translate(self.speed, 0.0);
            }

            // ------------------------------------------------
            // ROTATION
            // ------------------------------------------------

            if self.keyboard.q {
                player.rotate(0.02);
            }

            if self.keyboard.e {
                player.rotate(-0.02);
            }

            // ------------------------------------------------
            // SCALE
            // ------------------------------------------------

            if self.keyboard.z {
                player.transform.scale[0] -= 0.01;
                player.transform.scale[1] -= 0.01;
            }

            if self.keyboard.x {
                player.transform.scale[0] += 0.01;
                player.transform.scale[1] += 0.01;
            }

            // ------------------------------------------------
            // SCALE LIMIT
            // ------------------------------------------------

            player.transform.scale[0] = player.transform.scale[0].clamp(0.1, 3.0);

            player.transform.scale[1] = player.transform.scale[1].clamp(0.1, 3.0);
        }

        // ====================================================
        // SPEED
        // ====================================================

        if self.keyboard.i {
            self.speed += 0.0001;
        }

        if self.keyboard.o {
            self.speed -= 0.0001;
        }

        self.speed = self.speed.max(0.0001);

        // ====================================================
        // CAMERA MOVEMENT
        // ====================================================

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

        // ====================================================
        // CAMERA ZOOM
        // ====================================================

        if self.keyboard.zoom_in {
            self.camera.set_zoom(self.camera.zoom + 0.01);
        }

        if self.keyboard.zoom_out {
            self.camera.set_zoom(self.camera.zoom - 0.01);
        }
    }
}
