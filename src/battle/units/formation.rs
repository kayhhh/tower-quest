use bevy::prelude::*;
use rand::Rng;

#[derive(Component, Clone, Default)]
pub enum Formation {
    #[default]
    Box,
    Pyramid,
}

impl Formation {
    pub fn coords(&self, count: usize) -> Vec<(f32, f32)> {
        match self {
            Formation::Pyramid => coords_pyramid(count),
            Formation::Box => coords_box(count),
        }
    }
}

fn coords_pyramid(count: usize) -> Vec<(f32, f32)> {
    let mut coords = Vec::new();

    let mut x = 0.0;
    let mut y = 0.0;

    let mut row = 0;
    let mut row_count = 1;

    while coords.len() < count {
        coords.push((x, y));

        if coords.len() == row_count {
            row += 1;
            row_count += row + 1;
            x = 0.0;
            y = row as f32;
        } else {
            x += 1.0;
            y -= 1.0;
        }
    }

    coords
}

fn coords_box(count: usize) -> Vec<(f32, f32)> {
    let mut coords = Vec::new();

    let mut x = 0.0;
    let mut y = 0.0;

    // Find closest square, greater than or equal to unit_count
    let mut square = 1;
    while square * square < count {
        square += 1;
    }

    for i in 0..count {
        coords.push((x, y));

        if i % square == square - 1 {
            x = 0.0;
            y += 1.0;
        } else {
            x += 1.0;
        }
    }

    coords
}

pub fn rand_formation() -> Formation {
    let mut rng = rand::thread_rng();

    match rng.gen_range(0..2) {
        0 => Formation::Pyramid,
        1 => Formation::Box,
        _ => unreachable!(),
    }
}
