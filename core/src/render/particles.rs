use glam::Vec3;
use rand::Rng;

/// A single ambient particle — tiny, faint, drifting upward.
pub struct Particle {
    pub pos: Vec3,
    pub vel: Vec3,
    pub life: f32,
    pub max_life: f32,
    pub alpha: f32,
    pub size: f32,
}

pub struct ParticleSystem {
    pub particles: Vec<Particle>,
}

impl ParticleSystem {
    pub fn new(count: usize) -> Self {
        let mut rng = rand::thread_rng();
        let particles = (0..count)
            .map(|_| {
                let max_life = rng.gen_range(6.0..14.0);
                Particle {
                    pos: Vec3::new(rng.gen_range(0.0..1920.0), rng.gen_range(0.0..1080.0), 0.0),
                    vel: Vec3::new(
                        rng.gen_range(-4.0..4.0),
                        rng.gen_range(-14.0..-4.0), // drift upward
                        0.0,
                    ),
                    life: rng.gen_range(0.0..max_life),
                    max_life,
                    alpha: rng.gen_range(0.04..0.18),
                    size: rng.gen_range(1.0..2.5),
                }
            })
            .collect();
        Self { particles }
    }

    pub fn update(&mut self, dt: f32, width: f32, height: f32) {
        let mut rng = rand::thread_rng();
        for p in &mut self.particles {
            p.pos += p.vel * dt;
            p.life -= dt;

            // Reset when dead or off-screen
            if p.life <= 0.0 || p.pos.y < -20.0 || p.pos.x < -20.0 || p.pos.x > width + 20.0 {
                let ml = rng.gen_range(6.0..14.0);
                p.pos = Vec3::new(
                    rng.gen_range(0.0..width),
                    rng.gen_range(height * 0.4..height + 20.0),
                    0.0,
                );
                p.vel = Vec3::new(rng.gen_range(-4.0..4.0), rng.gen_range(-14.0..-4.0), 0.0);
                p.life = ml;
                p.max_life = ml;
                p.alpha = rng.gen_range(0.04..0.18);
                p.size = rng.gen_range(1.0..2.5);
            }
        }
    }
}
