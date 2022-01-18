use crate::traits::Quantization;
use std::collections::HashMap;

pub struct MedianCut {
    space: Vec<Point>,
    buckets: usize,
    shades_count: usize,
    palette: Vec<[u8; 3]>,
}

pub struct Point {
    index: usize, // запоминаем место, откуда мы его брали изначально
    xyz: [u8; 3],
}

impl MedianCut {
    pub fn new(space: Vec<[u8; 3]>, shades_count: usize) -> Self {
        let space: Vec<Point> = space
            .into_iter()
            .enumerate()
            .map(|(i, xyz)| Point { index: i, xyz })
            .collect();

        Self {
            buckets: 1,
            palette: Vec::new(),
            shades_count,
            space,
        }
    }
}

impl Quantization for MedianCut {
    fn build(&mut self) {
        let points_count = self.space.len();
        let mut buckets_count: usize = 1;

        while buckets_count < self.shades_count {
            let bucket_size = points_count / buckets_count;

            for b in 0..buckets_count {
                let bucket = &mut self.space[(b * bucket_size)..((b + 1) * bucket_size)];

                let mut x: (u8, u8) = (255, 0);
                let mut y: (u8, u8) = (255, 0);
                let mut z: (u8, u8) = (255, 0);

                bucket.iter().for_each(|p| {
                    if p.xyz[0] < x.0 {
                        x.0 = p.xyz[0];
                    }
                    if p.xyz[0] > x.1 {
                        x.1 = p.xyz[0];
                    }

                    if p.xyz[1] < y.0 {
                        y.0 = p.xyz[1];
                    }
                    if p.xyz[1] > y.1 {
                        y.1 = p.xyz[1];
                    }

                    if p.xyz[2] < z.0 {
                        z.0 = p.xyz[2];
                    }
                    if p.xyz[2] > z.1 {
                        z.1 = p.xyz[2];
                    }
                });

                let mut highest_space: usize = 0;
                let x_range = x.1 - x.0;
                let y_range = y.1 - y.0;
                let z_range = z.1 - z.0;

                if y_range >= z_range && y_range >= x_range {
                    highest_space = 1;
                } else if z_range >= y_range && z_range >= x_range {
                    highest_space = 2;
                }

                bucket.sort_by(|p1, p2| p1.xyz[highest_space].cmp(&p2.xyz[highest_space]));
            }

            buckets_count *= 2;
        }

        self.buckets = buckets_count;

        for b in 0..buckets_count {
            let bucket_size = points_count / buckets_count;
            let bucket = &mut self.space[(b * bucket_size)..((b + 1) * bucket_size)];

            let mut hash_colors = HashMap::<[u8; 3], usize>::new();

            bucket.iter().for_each(|point| {
                hash_colors
                    .entry(point.xyz)
                    .and_modify(|n| *n += 1)
                    .or_insert(1);
            });

            let mut max_n: usize = 0;
            let mut max_xyz: [u8; 3] = [0, 0, 0];

            hash_colors.into_iter().for_each(|(xyz, n)| {
                if n > max_n {
                    max_xyz = xyz;
                    max_n = n;
                }
            });

            self.palette.push(max_xyz);

            bucket.iter_mut().for_each(|point| point.xyz = max_xyz);
        }

        self.space.sort_by(|p1, p2| p1.index.cmp(&p2.index));
    }

    fn get_palette(&mut self) -> Vec<String> {
        self.palette
            .iter()
            .map(|[r, g, b]| format!("#{:02X?}{:02X?}{:02X?}", r, g, b))
            .collect()
    }

    fn to_buffer(&mut self) -> Vec<u8> {
        self.space.iter().flat_map(|point| point.xyz).collect()
    }
}
