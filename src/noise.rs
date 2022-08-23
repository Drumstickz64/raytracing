use rand::prelude::*;

use crate::math;

pub struct Perlin {
    ranvec: Vec<glam::DVec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let ranvec: Vec<glam::DVec3> = std::iter::repeat_with(|| math::random_range_vec(-1.0, 1.0))
            .take(Self::POINT_COUNT)
            .collect();

        Self {
            ranvec,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn turb(&self, p: glam::DVec3, depth: i32) -> f64 {
        let mut acc = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            acc += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        acc.abs()
    }

    pub fn noise(&self, p: glam::DVec3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[glam::DVec3::ZERO; 2]; 2]; 2];

        #[allow(clippy::needless_range_loop)]
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]]
                }
            }
        }

        Self::perlin_interp(c, u, v, w)
    }

    fn perlin_generate_perm() -> Vec<usize> {
        let mut p: Vec<usize> = (0..Self::POINT_COUNT).collect();
        Self::permute(&mut p, Self::POINT_COUNT);
        p
    }

    fn permute(p: &mut [usize], n: usize) {
        let mut rng = thread_rng();
        for i in (1..n).rev() {
            let target_idx = rng.gen_range(0..i);
            p.swap(i, target_idx);
        }
    }

    fn perlin_interp(c: [[[glam::DVec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        // Hermitian Smoothing
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut acc = 0.0;

        #[allow(clippy::needless_range_loop)]
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = glam::dvec3(u - i as f64, v - j as f64, w - k as f64);
                    acc += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * c[i][j][k].dot(weight_v);
                }
            }
        }

        acc
    }
}
