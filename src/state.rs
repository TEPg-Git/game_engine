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
    // Units per second.
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

            ball_velocity: [-0.01, -1.5], //BALL VELOCITY

            camera: Camera::new(),

            // Delta time is measured in seconds, so movement
            // speed is defined as world units per second.
            speed: 1.0,

            next_entity_id: 0,

            text: {
                let mut text = Text::new("EAST ENGINE PONG GAME", 24.0);

                text.set_position(0.0, 0.9);
                text.set_color(1.0, 1.0, 1.0, 1.0);
                text.set_alignment(TextAlignment::Center);
                text.set_line_spacing(1.2);
                text.set_letter_spacing(0.5);
                text.set_scale(1.0, 1.0);
                text.set_rotation(0.0);
                text.set_opacity(1.0);
                text.set_visible(true);
                text.set_max_width(None);

                text
            },
        };

        // ====================================================
        // PLAYERS AND BALL
        // ====================================================

        game_state.player1_id = game_state.create_entity("Player_1");
        game_state.player2_id = game_state.create_entity("Player_2");
        game_state.ball_id = game_state.create_entity("Ball");

        // ====================================================
        // INITIAL PLAYER POSITIONS
        // ====================================================

        // Player positions are expressed in normalized device
        // coordinates (NDC), where the screen goes from -1 to +1.
        if let Some(player_1) = game_state.get_entity_mut(game_state.player1_id) {
            player_1.transform.position = [-0.8, 0.0];
        }

        if let Some(player_2) = game_state.get_entity_mut(game_state.player2_id) {
            player_2.transform.position = [0.8, 0.0];
        }

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
        // PLAYER 1
        // ====================================================

        let key_w = self.keyboard.w;
        let key_s = self.keyboard.s;
        let speed = self.speed;

        if let Some(player_1) = self.get_entity_mut(self.player1_id) {
            if key_w {
                player_1.translate(0.0, speed * delta_time);
            }

            if key_s {
                player_1.translate(0.0, -speed * delta_time);
            }

            // The player sprite is 0.5 units tall, so its half-height
            // is 0.25 units. Keep its center inside those limits so
            // the whole sprite stays visible on screen.
            player_1.transform.position[1] = player_1.transform.position[1].clamp(-0.75, 0.75);
        }

        // ====================================================
        // PLAYER 2
        // ====================================================

        let key_up = self.keyboard.up;
        let key_down = self.keyboard.down;

        if let Some(player_2) = self.get_entity_mut(self.player2_id) {
            if key_up {
                player_2.translate(0.0, speed * delta_time);
            }

            if key_down {
                player_2.translate(0.0, -speed * delta_time);
            }

            // Keep the entire player sprite inside the screen.
            player_2.transform.position[1] = player_2.transform.position[1].clamp(-0.75, 0.75);
        }

        // ====================================================
        // BALL
        // ====================================================

        let ball_velocity = self.ball_velocity;

        if let Some(ball) = self.get_entity_mut(self.ball_id) {
            ball.translate(ball_velocity[0] * delta_time, ball_velocity[1] * delta_time);
        }

        let mut bounced = false;

        if let Some(ball) = self.get_entity_mut(self.ball_id) {
            if ball.transform.position[1] > 1.0 {
                ball.transform.position[1] = 1.0;
                bounced = true;
            }

            if ball.transform.position[1] < -1.0 {
                ball.transform.position[1] = -1.0;
                bounced = true;
            }
        }

        if bounced {
            self.ball_velocity[1] = -self.ball_velocity[1];
        }
    }
}
