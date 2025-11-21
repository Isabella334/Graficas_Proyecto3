pub struct CellularNoise {}

impl CellularNoise {
    pub fn new() -> Self {
        CellularNoise {}
    }

    fn hash2d(x: i32, y: i32) -> (f32, f32) {
        let mut h = x as u32;
        h = h.wrapping_mul(374761393).wrapping_add(y as u32);
        h ^= h >> 13;
        h = h.wrapping_mul(1274126177);
        h ^= h >> 16;
        
        let h2 = h.wrapping_mul(668265263);
        
        let fx = (h & 0xFFFF) as f32 / 65535.0;
        let fy = (h2 & 0xFFFF) as f32 / 65535.0;
        
        (fx, fy)
    }

    pub fn cellular2d(&self, x: f32, y: f32) -> f32 {
        let xi = x.floor() as i32;
        let yi = y.floor() as i32;
        
        let mut min_dist = f32::INFINITY;
        
        for dy in -1..=1 {
            for dx in -1..=1 {
                let cell_x = xi + dx;
                let cell_y = yi + dy;
                
                let (rand_x, rand_y) = Self::hash2d(cell_x, cell_y);
                
                let point_x = cell_x as f32 + rand_x;
                let point_y = cell_y as f32 + rand_y;
                
                let dist_x = x - point_x;
                let dist_y = y - point_y;
                let dist = (dist_x * dist_x + dist_y * dist_y).sqrt();
                
                if dist < min_dist {
                    min_dist = dist;
                }
            }
        }
        
        min_dist
    }

    pub fn cellular2d_f2_f1(&self, x: f32, y: f32) -> f32 {
        let xi = x.floor() as i32;
        let yi = y.floor() as i32;
        
        let mut min_dist1 = f32::INFINITY;
        let mut min_dist2 = f32::INFINITY;
        
        for dy in -1..=1 {
            for dx in -1..=1 {
                let cell_x = xi + dx;
                let cell_y = yi + dy;
                
                let (rand_x, rand_y) = Self::hash2d(cell_x, cell_y);
                
                let point_x = cell_x as f32 + rand_x;
                let point_y = cell_y as f32 + rand_y;
                
                let dist_x = x - point_x;
                let dist_y = y - point_y;
                let dist = (dist_x * dist_x + dist_y * dist_y).sqrt();
                
                if dist < min_dist1 {
                    min_dist2 = min_dist1;
                    min_dist1 = dist;
                } else if dist < min_dist2 {
                    min_dist2 = dist;
                }
            }
        }
        
        min_dist2 - min_dist1
    }
}
