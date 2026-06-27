use eframe::egui::{Color32, Painter, Pos2, Vec2};
use rand::Rng;

struct Particle {
    pos: Pos2,
    vel: Vec2,
    color: Color32,
    lifetime: f32,
    max_lifetime: f32,
}

pub struct FireworkEffect {
    particles: Vec<Particle>,
    elapsed: f32,
    duration: f32,
}

impl FireworkEffect {
    pub fn new(center: Pos2) -> Self {
        let mut rng = rand::rng();
        let palette = [
            Color32::from_rgb(255, 50, 50),
            Color32::from_rgb(255, 200, 50),
            Color32::from_rgb(255, 100, 200),
            Color32::from_rgb(50, 150, 255),
            Color32::from_rgb(100, 255, 100),
            Color32::from_rgb(255, 150, 50),
            Color32::from_rgb(200, 100, 255),
            Color32::from_rgb(255, 255, 100),
        ];
        let count = 120;
        let mut particles = Vec::with_capacity(count);
        for _ in 0..count {
            let angle = rng.random_range(0.0..std::f32::consts::TAU);
            let speed = rng.random_range(120.0..350.0);
            let lifetime = rng.random_range(1.5..3.5);
            particles.push(Particle {
                pos: center,
                vel: Vec2::new(angle.cos() * speed, angle.sin() * speed),
                color: palette[rng.random_range(0..palette.len())],
                lifetime,
                max_lifetime: lifetime,
            });
        }
        FireworkEffect {
            particles,
            elapsed: 0.0,
            duration: 4.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.elapsed += dt;
        for p in &mut self.particles {
            p.pos += p.vel * dt;
            p.vel.y += 200.0 * dt;
            p.vel *= 0.98;
            p.lifetime -= dt;
        }
    }

    pub fn draw(&self, painter: &Painter) {
        for p in &self.particles {
            if p.lifetime <= 0.0 {
                continue;
            }
            let alpha = (p.lifetime / p.max_lifetime).clamp(0.0, 1.0);
            let size = 3.0 + 4.0 * alpha;
            let color = Color32::from_rgba_premultiplied(
                p.color.r(),
                p.color.g(),
                p.color.b(),
                (alpha * 255.0) as u8,
            );
            painter.circle_filled(p.pos, size, color);
        }
    }

    pub fn is_finished(&self) -> bool {
        self.elapsed >= self.duration || self.particles.iter().all(|p| p.lifetime <= 0.0)
    }
}
