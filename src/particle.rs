use sfml::{system::Vector2f, window::Event};

const CLICK_TOLERANCE: f32 = 0.1;

pub struct Particle {
    pub position: Vector2f,
    pub previous_position: Vector2f,
    pub acceleration: Vector2f,
    is_pinned: bool,
}

impl Particle {
    pub fn new(x: f32, y: f32, is_pinned: bool) -> Particle {
        Particle {
            position: Vector2f::new(x, y),
            previous_position: Vector2f::new(x, y),
            acceleration: Vector2f::new(0.0, 0.0),
            is_pinned,
        }
    }

    pub fn apply_force(&mut self, force: Vector2f) {
        if !self.is_pinned {
            self.acceleration += force;
        }
    }

    pub fn update(&mut self, time_step: f32) {
        if !self.is_pinned {
            let velocity = self.position - self.previous_position;
            self.previous_position = self.position;
            self.position += velocity + self.acceleration * time_step * time_step;
            self.acceleration = Vector2f::new(0.0, 0.0);
        }
    }

    pub fn constrain_to_bounds(&mut self, width: f32, height: f32) {
        if self.position.x < 0.0 {
            self.position.x = 0.0;
        }

        if self.position.x > width {
            self.position.x = width;
        }

        if self.position.y < 0.0 {
            self.position.y = 0.0;
        }

        if self.position.y > height {
            self.position.y = height;
        }
    }
}

pub struct Constraint {
    p1_index: usize,
    p2_index: usize,
    initial_length: f32,
    active: bool,
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
            active: true,
        }
    }

    pub fn satisfy(&self, particles: &mut Vec<Particle>) {
        if !self.active {
            return;
        }

        if let Some((p1, p2)) = get_two_particles_mut(particles, self.p1_index, self.p2_index) {
            let delta = p2.position - p1.position;
            let current_length = hypot(delta.x, delta.y);

            if current_length > f32::EPSILON {
                let difference = (current_length - self.initial_length) / current_length;
                let correction = delta * (0.5 * difference);
                if !p1.is_pinned {
                    p1.position += correction;
                }

                if !p2.is_pinned {
                    p2.position -= correction;
                }
            }
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

    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

fn get_two_particles_mut(
    particles: &mut Vec<Particle>,
    idx1: usize,
    idx2: usize,
) -> Option<(&mut Particle, &mut Particle)> {
    if idx1 == idx2 {
        return None;
    }

    if idx1 < idx2 {
        let (first_half, second_half) = particles.split_at_mut(idx1 + 1);
        Some((&mut first_half[idx1], &mut second_half[idx2 - idx1 - 1]))
    } else {
        let (first_half, second_half) = particles.split_at_mut(idx2 + 1);
        Some((&mut second_half[idx1 - idx2 - 1], &mut first_half[idx2]))
    }
}

fn hypot(x: f32, y: f32) -> f32 {
    (x.powi(2) + y.powi(2)).sqrt()
}

struct InputHandler {}

impl InputHandler {
    pub fn handle_mouse_click(
        event: Event,
        particles: Vec<Particle>,
        constraints: &Vec<Constraint>,
    ) {
        match event {
            Event::MouseButtonPressed { button, x, y }
                if button == sfml::window::mouse::Button::Left =>
            {
                println!("Mouse clicked at ({}, {})", x, y);
                // Add your logic here (e.g., check if a particle was clicked)
            }
            _ => {}
        }
    }

    fn find_nearest_constraint(mouse_x: f32, mouse_y: f32, constraints: &Vec<Constraint>) {
        let min_distance = CLICK_TOLERANCE;
        for constraint in constraints {
            let distance = point_to_segment_distance();
        }
    }

    fn tear_cloth(
        mouse_x: f32,
        mouse_y: f32,
        particles: &Vec<Particle>,
        constraints: &Vec<Constraint>,
    ) {
    }
}
