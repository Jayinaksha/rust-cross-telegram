use rand::Rng;

pub struct MatrixColumn {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
}

pub struct MatrixRain {
    pub cols: Vec<MatrixColumn>,
}

impl MatrixRain {

    pub fn new(count: usize) -> Self {

        let mut rng = rand::thread_rng();

        let mut cols = Vec::new();

        for _ in 0..count {

            cols.push(MatrixColumn {
                x: rng.gen_range(0.0..1920.0),
                y: rng.gen_range(0.0..1080.0),
                speed: rng.gen_range(40.0..120.0),
            });

        }

        Self { cols }

    }

    pub fn update(&mut self, dt: f32) {

        for col in &mut self.cols {

            col.y += col.speed * dt;

            if col.y > 1080.0 {
                col.y = 0.0;
            }

        }

    }

}