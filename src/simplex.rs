pub struct Simplex {}

impl Simplex {
    pub fn new() -> Self {
        Simplex {}
    }

    fn hash(x: i32, y: i32) -> u8 {
        let mut h = x as u32;
        h = h.wrapping_mul(374761393).wrapping_add(y as u32);
        h ^= h >> 13;
        h = h.wrapping_mul(1274126177);
        ((h ^ (h >> 16)) & 0xFF) as u8
    }

    fn grad(hash: u8, x: f32, y: f32) -> f32 {
        match hash & 0x3 {
            0 =>  x + y,
            1 => -x + y,
            2 =>  x - y,
            _ => -x - y,
        }
    }

    pub fn noise2d(&self, x: f32, y: f32) -> f32 {
        const F2: f32 = 0.366025403;
        const G2: f32 = 0.211324865;

        let s = (x + y) * F2;
        let i = (x + s).floor() as i32;
        let j = (y + s).floor() as i32;

        let t = (i + j) as f32 * G2;
        let x0 = x - (i as f32 - t);
        let y0 = y - (j as f32 - t);

        let (i1, j1) = if x0 > y0 { (1, 0) } else { (0, 1) };

        let x1 = x0 - i1 as f32 + G2;
        let y1 = y0 - j1 as f32 + G2;
        let x2 = x0 - 1.0 + 2.0 * G2;
        let y2 = y0 - 1.0 + 2.0 * G2;

        let gi0 = Self::hash(i,     j    );
        let gi1 = Self::hash(i+i1, j+j1 );
        let gi2 = Self::hash(i+1,  j+1  );

        let mut n0 = 0.0;
        let mut n1 = 0.0;
        let mut n2 = 0.0;

        let t0 = 0.5 - x0*x0 - y0*y0;
        if t0 > 0.0 {
            let t0_2 = t0 * t0;
            n0 = t0_2 * t0_2 * Self::grad(gi0, x0, y0);
        }

        let t1 = 0.5 - x1*x1 - y1*y1;
        if t1 > 0.0 {
            let t1_2 = t1 * t1;
            n1 = t1_2 * t1_2 * Self::grad(gi1, x1, y1);
        }

        let t2 = 0.5 - x2*x2 - y2*y2;
        if t2 > 0.0 {
            let t2_2 = t2 * t2;
            n2 = t2_2 * t2_2 * Self::grad(gi2, x2, y2);
        }

        70.0 * (n0 + n1 + n2)
    }

    pub fn fbm(&self, x: f32, y: f32, octaves: usize, lacunarity: f32, gain: f32) -> f32 {
        let mut total = 0.0;
        let mut frequency = 1.0;
        let mut amplitude = 1.0;
        let mut max_value = 0.0;

        for _ in 0..octaves {
            total += self.noise2d(x * frequency, y * frequency) * amplitude;
            max_value += amplitude;
            amplitude *= gain;
            frequency *= lacunarity;
        }

        total / max_value
    }
}
