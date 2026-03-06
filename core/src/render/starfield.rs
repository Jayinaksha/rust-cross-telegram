use glam::Vec3;
use rand::Rng;

pub struct Star {
    pub pos: Vec3,
    pub speed: f32,
}

pub struct Starfield {
    pub stars: Vec<Star>,
}

impl Starfield {

    pub fn new(count: usize) -> Self {

        let mut rng = rand::thread_rng();

        let mut stars = Vec::new();

        for _ in 0..count {

            stars.push(Star {
                pos: Vec3::new(
                    rng.gen_range(-50.0..50.0),
                    rng.gen_range(-50.0..50.0),
                    rng.gen_range(1.0..100.0),
                ),
                speed: rng.gen_range(10.0..40.0),
            });

        }

        Self { stars }

    }

    pub fn update(&mut self, dt: f32) {

        for star in &mut self.stars {

            star.pos.z -= star.speed * dt;

            if star.pos.z < 1.0 {
                star.pos.z = 100.0;
            }

        }

    }

}