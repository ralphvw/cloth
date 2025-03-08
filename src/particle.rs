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
        if self.position.x < radius {
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

pub struct Constraint {
    p1_index: usize,
    p2_index: usize,
    initial_length: f32,
}

impl Constraint {
    pub fn new(particles: &Vec<Particle>, p1_index: usize, p2_index: usize) -> Self {
        let p1 = &particles[p1_index];
        let p2 = &particles[p2_index];

        let x = p2.position.x - p1.position.x;
        let y = p2.position.y - p1.position.y;
        let initial_length = hypot(x, y);

        Self {
            p1_index,
            p2_index,
            initial_length,
        }
    }

    pub fn satisfy(&self, particles: &mut Vec<Particle>) {
        let (p1, p2) = get_two_particles_mut(particles, self.p1_index, self.p2_index);

        let delta = p2.position - p1.position;
        let current_length = hypot(delta.x, delta.y);

        if current_length > f32::EPSILON {
            let difference = (current_length - self.initial_length) / current_length;
            let correction = delta * (0.5 * difference);
            p1.position += correction;
            p2.position -= correction;
        }
    }

    pub fn get_p1_position(&self, particles: &Vec<Particle>) -> Vector2f {
        particles[self.p1_index].position
    }

    pub fn get_p2_position(&self, particles: &Vec<Particle>) -> Vector2f {
        particles[self.p2_index].position
    }

    pub fn get_particle_indices(&self) -> (usize, usize) {
        (self.p1_index, self.p2_index)
    }
}

fn get_two_particles_mut(
    particles: &mut Vec<Particle>,
    idx1: usize,
    idx2: usize,
) -> (&mut Particle, &mut Particle) {
    assert!(idx1 != idx2, "Cannot get the same particle twice");

    if idx1 < idx2 {
        let (first_half, second_half) = particles.split_at_mut(idx1 + 1);
        (&mut first_half[idx1], &mut second_half[idx2 - idx1 - 1])
    } else {
        let (first_half, second_half) = particles.split_at_mut(idx2 + 1);
        (&mut second_half[idx1 - idx2 - 1], &mut first_half[idx2])
    }
}

fn hypot(x: f32, y: f32) -> f32 {
    (x.powi(2) + y.powi(2)).sqrt()
}
