use std::ops::Deref;

use cloth::particle::{Constraint, InputHandler, Particle};
use sfml::{
    graphics::{
        Color, PrimitiveType, RenderTarget, RenderWindow, Vertex, VertexBuffer, VertexBufferUsage,
    },
    system::Vector2f,
    window::{Event, Style},
};

const WIDTH: f32 = 1080.0;
const HEIGHT: f32 = 640.0;
// const PARTICLE_RADIUS: f32 = 10.0;
const GRAVITY: f32 = 10.0;
const TIME_STEP: f32 = 0.5;
const ROW: i32 = 10;
const COL: i32 = 10;
const REST_DISTANCE: f32 = 30.0;

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

    for row in 0..ROW {
        for col in 0..COL {
            let x = col as f32 * REST_DISTANCE + WIDTH / 3.0;
            let y = row as f32 * REST_DISTANCE + HEIGHT / 3.0;
            let pinned = row == 0;
            particles.push(Particle::new(x, y, pinned));
        }
    }

    let mut constraints: Vec<Constraint> = Vec::new();

    for row in 0..ROW {
        for col in 0..COL {
            if col < COL - 1 {
                constraints.push(Constraint::new(
                    &particles,
                    (row * COL + col) as usize,
                    (row * COL + col + 1) as usize,
                ));
            }

            if row < ROW - 1 {
                constraints.push(Constraint::new(
                    &particles,
                    (row * COL + col) as usize,
                    ((row + 1) * COL + col) as usize,
                ));
            }
        }
    }

    while window.is_open() {
        while let Some(e) = window.poll_event() {
            if e == Event::Closed {
                window.close();
            }

            InputHandler::handle_mouse_click(e, &particles, constraints.as_mut());
        }

        for particle in &mut particles {
            particle.apply_force(Vector2f::new(0.0, GRAVITY));
            particle.update(TIME_STEP);
            particle.constrain_to_bounds(WIDTH, HEIGHT);
            println!(
                "Previous position: {}, Current Position: {}",
                particle.previous_position.y, particle.position.y
            );
        }

        for _ in 0..5 {
            for constraint in &constraints {
                constraint.satisfy(&mut particles);
            }
        }

        window.clear(Color::BLACK);

        // for particle in &particles {
        //     let mut circle = CircleShape::new(PARTICLE_RADIUS, 30);
        //     circle.set_fill_color(Color::WHITE);
        //     circle.set_position((
        //         particle.position.x - PARTICLE_RADIUS,
        //         particle.position.y - PARTICLE_RADIUS,
        //     ));
        //     window.draw(&circle);
        // }

        for particle in &particles {
            let points = vec![Vertex::with_pos_color(particle.position, Color::WHITE)];

            let mut buffer =
                VertexBuffer::new(PrimitiveType::POINTS, 1, VertexBufferUsage::STREAM).unwrap();
            _ = buffer.update(&points, 0);

            window.draw(buffer.deref());
        }

        for constraint in &constraints {
            if !constraint.active {
                continue;
            }
            let lines = vec![
                Vertex::with_pos_color(constraint.get_p1_position(&particles), Color::WHITE),
                Vertex::with_pos_color(constraint.get_p2_position(&particles), Color::WHITE),
            ];

            let mut buffer =
                VertexBuffer::new(PrimitiveType::LINES, 2, VertexBufferUsage::DYNAMIC).unwrap();
            _ = buffer.update(&lines, 0);

            window.draw(buffer.deref());
        }

        window.display();
    }
}
