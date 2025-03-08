use sfml::system::Vector2f;

pub struct Particle {
    pub position: Vector2f,
    pub previous_position: Vector2f,
    pub acceleration: Vector2f,
}

impl Particle {
    pub fn new(x: f32, y: f32) -> Particle {
        Particle {
            position: Vector2f::new(x, y),
            previous_position: Vector2f::new(x, y),
            acceleration: Vector2f::new(0.0, 0.0),
        }
    }

    pub fn apply_force(&mut self, force: Vector2f) {
        self.acceleration += force;
    }

    pub fn update(&mut self, time_step: f32) {
        let velocity = self.position - self.previous_position;
        self.previous_position = self.position;
        self.position += velocity + self.acceleration * time_step * time_step;
        self.acceleration = Vector2f::new(0.0, 0.0);
    }

    pub fn constrain_to_bounds(&mut self, width: f32, height: f32, radius: f32) {
        if self.position.x > radius {
            self.position.x = radius;
        }

        if self.position.x > width - radius {
            self.position.x = width - radius;
        }

        if self.position.y < radius {
            self.position.y = radius;
        }

        if self.position.y > height - radius {
            self.position.y = height - radius;
        }
    }
}
