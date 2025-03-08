use cloth::particle::Particle;
use sfml::{
    graphics::{CircleShape, Color, RenderTarget, RenderWindow, Shape, Transformable},
    system::Vector2f,
    window::{Event, Style},
};

const WIDTH: f32 = 1080.0;
const HEIGHT: f32 = 640.0;
const PARTICLE_RADIUS: f32 = 30.0;
const GRAVITY: f32 = 10.0;
const TIME_STEP: f32 = 0.1;

fn main() {
    let mut window = RenderWindow::new(
        (WIDTH as u32, HEIGHT as u32),
        "cloth",
        Style::CLOSE,
        &Default::default(),
    )
    .expect("Error Rendering Window");

    window.set_vertical_sync_enabled(true);

    window.set_framerate_limit(60);

    let mut particles: Vec<Particle> = Vec::new();
    particles.push(Particle::new(WIDTH / 2.0, HEIGHT / 2.0));

    while window.is_open() {
        while let Some(e) = window.poll_event() {
            if e == Event::Closed {
                window.close();
            }
        }

        for particle in &mut particles {
            particle.apply_force(Vector2f::new(0.0, GRAVITY));
            particle.update(TIME_STEP);
        }

        window.clear(Color::BLACK);

        for particle in &particles {
            let mut circle = CircleShape::new(PARTICLE_RADIUS, 30);
            circle.set_fill_color(Color::WHITE);
            circle.set_position(particle.position);
            window.draw(&circle);
        }

        window.display();
    }
}
