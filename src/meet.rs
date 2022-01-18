use std::collections::HashMap;

use crate::traits::Quantization;

pub struct Meet {
    space: Vec<[u8; 3]>,
    shades_count: usize,
    palette: Vec<[u8; 3]>,
    colors: Vec<([u8; 3], usize)>,
}

impl Meet {
    pub fn new(space: Vec<[u8; 3]>, shades_count: usize) -> Self {
        Self {
            space,
            shades_count,
            palette: Vec::new(),
            colors: Vec::new()
        }
    }
}

impl Quantization for Meet {
    fn build(&mut self) {
        let mut hash_colors = HashMap::<[u8; 3], usize>::new();

        self.space.iter().for_each(|&point| {
            hash_colors
                .entry(point)
                .and_modify(|n| *n += 1)
                .or_insert(1);
        });

        let mut colors: Vec<([u8; 3], usize)> = hash_colors.into_iter().collect();
        colors.sort_by(|(_, i1), (_, i2)| i2.cmp(i1));

        self.colors = colors;

        for i in 0..self.shades_count {
            self.palette.push(self.colors[i].0);
        }
    }

    fn get_palette(&mut self) -> Vec<String> {
        self.palette
            .iter()
            .map(|[r, g, b]| format!("#{:02X?}{:02X?}{:02X?}", r, g, b))
            .collect()
    }

    fn to_buffer(&mut self) -> Vec<u8> {
        self.colors.iter().flat_map(|&(pixel, count)| {
            (0..count).flat_map(|_| pixel).collect::<Vec<u8>>()
        }).collect()
    }
}
