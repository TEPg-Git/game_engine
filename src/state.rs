use crate::camera::Camera;
use crate::entity::Entity;
use crate::input::KeyboardState;
use crate::particle::Particle;
use crate::text::{Text, TextAlignment};

pub struct GameState {
    pub particle: Particle,
    pub score_1: u32,
    pub score_2: u32,
    pub keyboard: KeyboardState,
    pub entities: Vec<Entity>,
    pub player1_id: u32,
    pub player2_id: u32,
    pub ball_id: u32,
    pub ball_velocity: [f32; 2],
    pub camera: Camera,
    pub speed: f32,
    next_entity_id: u32,
    pub text: Text,
    pub score_text: Text,
    last_score_1: u32,
    last_score_2: u32,
}

impl GameState {
    pub fn new() -> Self {
        let mut game_state = Self {
            particle: Particle::new([0.0, 0.0], [1.0, 0.5], 1.0),
            score_1: 0,
            score_2: 0,
            keyboard: KeyboardState::default(),
            entities: Vec::with_capacity(3),
            player1_id: 0,
            player2_id: 0,
            ball_id: 0,
            ball_velocity: [-0.4, -0.5],
            camera: Camera::new(),
            speed: 1.0,
            next_entity_id: 0,
            text: Text::new("", 24.0),
            score_text: Text::new("", 24.0),
            last_score_1: 0,
            last_score_2: 0,
        };

        game_state.player1_id = game_state.create_entity("Player_1");
        game_state.player2_id = game_state.create_entity("Player_2");
        game_state.ball_id = game_state.create_entity("Ball");

        if let Some(player_1) = game_state.get_entity_mut(game_state.player1_id) {
            player_1.transform.position = [-0.8, 0.0];
        }
        if let Some(player_2) = game_state.get_entity_mut(game_state.player2_id) {
            player_2.transform.position = [0.8, 0.0];
        }

        game_state
    }

    pub fn create_entity(&mut self, name: &str) -> u32 {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        self.entities.push(Entity::new(id, name));
        id
    }

    pub fn get_entity(&self, id: u32) -> Option<&Entity> {
        self.entities
            .get(id as usize)
            .filter(|entity| entity.id == id)
    }

    pub fn get_entity_mut(&mut self, id: u32) -> Option<&mut Entity> {
        self.entities
            .get_mut(id as usize)
            .filter(|entity| entity.id == id)
    }

    pub fn update(&mut self, delta_time: f32) {
        self.particle.update(delta_time);

        let speed = self.speed;
        let key_w = self.keyboard.w;
        let key_s = self.keyboard.s;
        if let Some(player_1) = self.get_entity_mut(self.player1_id) {
            if key_w {
                player_1.translate(0.0, speed * delta_time);
            }
            if key_s {
                player_1.translate(0.0, -speed * delta_time);
            }
            player_1.transform.position[1] = player_1.transform.position[1].clamp(-0.75, 0.75);
        }

        let key_up = self.keyboard.up;
        let key_down = self.keyboard.down;
        if let Some(player_2) = self.get_entity_mut(self.player2_id) {
            if key_up {
                player_2.translate(0.0, speed * delta_time);
            }
            if key_down {
                player_2.translate(0.0, -speed * delta_time);
            }
            player_2.transform.position[1] = player_2.transform.position[1].clamp(-0.75, 0.75);
        }

        let ball_velocity = self.ball_velocity;
        if let Some(ball) = self.get_entity_mut(self.ball_id) {
            ball.translate(ball_velocity[0] * delta_time, ball_velocity[1] * delta_time);
        }

        let ball_id = self.ball_id;
        let (ball_half_width, ball_half_height) = {
            let ball = self.get_entity(ball_id).expect("Ball entity missing");
            let sprite = ball.sprite.as_ref().expect("Ball sprite missing");
            (sprite.size[0] * 0.5, sprite.size[1] * 0.5)
        };

        let mut bounced = false;
        if let Some(ball) = self.get_entity_mut(ball_id) {
            if ball.transform.position[1] + ball_half_height > 1.0 {
                ball.transform.position[1] = 1.0 - ball_half_height;
                bounced = true;
            }
            if ball.transform.position[1] - ball_half_height < -1.0 {
                ball.transform.position[1] = -1.0 + ball_half_height;
                bounced = true;
            }
        }
        if bounced {
            self.ball_velocity[1] = -self.ball_velocity[1];
        }

        if let Some(ball) = self.get_entity_mut(ball_id)
            && ball.transform.position[0] - ball_half_width > 1.0
        {
            ball.transform.position = [0.0, 0.0];
            self.ball_velocity[0] = -self.ball_velocity[0];
            self.score_2 += 1;
        }

        if let Some(ball) = self.get_entity_mut(ball_id)
            && ball.transform.position[0] + ball_half_width < -1.0
        {
            ball.transform.position = [0.0, 0.0];
            self.ball_velocity[0] = -self.ball_velocity[0];
            self.score_1 += 1;
        }

        let (player_2_y, player_2_half_width, player_2_half_height) = {
            let player_2 = self
                .get_entity(self.player2_id)
                .expect("Player 2 entity missing");
            let sprite = player_2.sprite.as_ref().expect("Player 2 sprite missing");
            (
                player_2.transform.position[1],
                sprite.size[0] * 0.5,
                sprite.size[1] * 0.5,
            )
        };
        let ball_velocity_x = self.ball_velocity[0];
        if let Some(ball) = self.get_entity_mut(ball_id) {
            let ball_x = ball.transform.position[0];
            let ball_y = ball.transform.position[1];
            let player_x = 0.8;
            let x_collision = ball_x + ball_half_width >= player_x - player_2_half_width;
            let y_collision = ball_y + ball_half_height >= player_2_y - player_2_half_height
                && ball_y - ball_half_height <= player_2_y + player_2_half_height;
            if x_collision && y_collision && ball_velocity_x > 0.0 {
                ball.transform.position[0] = player_x - player_2_half_width - ball_half_width;
                self.ball_velocity[0] = -self.ball_velocity[0];
            }
        }

        let (player_1_y, player_1_half_width, player_1_half_height) = {
            let player_1 = self
                .get_entity(self.player1_id)
                .expect("Player 1 entity missing");
            let sprite = player_1.sprite.as_ref().expect("Player 1 sprite missing");
            (
                player_1.transform.position[1],
                sprite.size[0] * 0.5,
                sprite.size[1] * 0.5,
            )
        };
        let ball_velocity_x = self.ball_velocity[0];
        if let Some(ball) = self.get_entity_mut(ball_id) {
            let ball_x = ball.transform.position[0];
            let ball_y = ball.transform.position[1];
            let player_x = -0.8;
            let x_collision = ball_x - ball_half_width <= player_x + player_1_half_width;
            let y_collision = ball_y + ball_half_height >= player_1_y - player_1_half_height
                && ball_y - ball_half_height <= player_1_y + player_1_half_height;
            if x_collision && y_collision && ball_velocity_x < 0.0 {
                ball.transform.position[0] = player_x + player_1_half_width + ball_half_width;
                self.ball_velocity[0] = -self.ball_velocity[0];
            }
        }

        if self.score_1 != self.last_score_1 || self.score_2 != self.last_score_2 {
            self.score_text
                .set_content(&format!("{} - {}", self.score_1, self.score_2));
            self.last_score_1 = self.score_1;
            self.last_score_2 = self.score_2;
        }
    }
}
