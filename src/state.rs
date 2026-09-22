use crate::camera::Camera;
use crate::entity::Entity;
use crate::input::KeyboardState;
use crate::text::{Text, TextAlignment};

// ============================================================
// GAME STATE
// ============================================================

pub struct GameState {
    // ========================================================
    // SCORE
    // ========================================================
    pub score_1: u32,
    pub score_2: u32,

    // ========================================================
    // MAX SCORE
    // ========================================================
    pub max_score: u32,

    // ========================================================
    // INPUT
    // ========================================================
    pub keyboard: KeyboardState,

    // ========================================================
    // ENTITIES
    // ========================================================
    pub entities: Vec<Entity>,

    // ========================================================
    // TWO PLAYER AND BALL ID
    // ========================================================
    pub player1_id: u32,
    pub player2_id: u32,
    pub ball_id: u32,
    // ========================================================
    // BALL VELOCITY
    // ========================================================
    pub ball_velocity: [f32; 2],

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
            score_1: 0,
            score_2: 0,
            max_score: 5,

            keyboard: KeyboardState::default(),

            entities: Vec::new(),

            player1_id: 0,
            player2_id: 0,
            ball_id: 0,

            ball_velocity: [-0.001, 0.0005],

            camera: Camera::new(),

            speed: 0.001,

            next_entity_id: 0,

            text: {
                // ------------------------------------------------
                // TEXT CONTENT
                // ------------------------------------------------

                let mut text = Text::new("EAST ENGINE\nPONG GAME", 24.0);

                // ------------------------------------------------
                // POSITION
                // ------------------------------------------------

                text.set_position(-0.5, 0.7);

                // ------------------------------------------------
                // COLOR
                // ------------------------------------------------

                text.set_color(1.0, 0.0, 0.0, 1.0);

                // ------------------------------------------------
                // ALIGNMENT
                // ------------------------------------------------

                text.set_alignment(TextAlignment::Center);

                // ------------------------------------------------
                // LINE SPACING
                // ------------------------------------------------

                text.set_line_spacing(1.2);

                // ------------------------------------------------
                // LETTER SPACING
                // ------------------------------------------------

                text.set_letter_spacing(0.5);

                // ------------------------------------------------
                // SCALE
                // ------------------------------------------------

                text.set_scale(1.0, 1.0);

                // ------------------------------------------------
                // ROTATION
                // ------------------------------------------------

                text.set_rotation(0.0);

                // ------------------------------------------------
                // OPACITY
                // ------------------------------------------------

                text.set_opacity(1.0);

                // ------------------------------------------------
                // VISIBILITY
                // ------------------------------------------------

                text.set_visible(true);

                // ------------------------------------------------
                // MAX WIDTH
                // ------------------------------------------------
                //
                // Wrapping is supported by the text system.
                // Keep it disabled for this basic demo so the
                // two explicit lines remain easy to see.
                // ------------------------------------------------

                text.set_max_width(None);

                text
            },
        };

        // ========================================================
        // PLAYERS AND BALL
        // ========================================================

        game_state.player1_id = game_state.create_entity("Player_1");
        game_state.player2_id = game_state.create_entity("Player_2");
        game_state.ball_id = game_state.create_entity("Ball");

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

    pub fn update(&mut self, delta_time: f32) {
        // ====================================================
        // PLAYER_1
        // ====================================================
        let key_w = self.keyboard.w;
        let key_s = self.keyboard.s;
        let speed = self.speed;
        if let Some(player_1) = self.get_entity_mut(self.player1_id) {
            // ------------------------------------------------
            // MOVEMENT     ONLY UP AND DOWN
            // ------------------------------------------------

            if key_w {
                player_1.translate(0.0, speed * delta_time);
            }

            if key_s {
                player_1.translate(0.0, speed * delta_time);
            }
        }
        // ====================================================
        // PLAYER_2
        // ====================================================
        let key_up = self.keyboard.up;
        let key_down = self.keyboard.down;
        if let Some(player_2) = self.get_entity_mut(self.player2_id) {
            if key_up {
                player_2.translate(0.0, speed * delta_time);
            }

            if key_down {
                player_2.translate(0.0, speed * delta_time);
            }
        }

        // ====================================================
        // BALL VELOCITY
        // ====================================================
        let ball_velocity = self.ball_velocity;
        if let Some(ball) = self.get_entity_mut(self.ball_id) {
            ball.translate(ball_velocity[0], ball_velocity[1]);
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
    }
}
