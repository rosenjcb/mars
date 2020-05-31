use raylib::{color::Color, math::Vector3};
use rand::Rng;
use crate::physics::{Scale, Physics};

pub struct Cube {
    color: Color,
}

pub struct Entity {
    pub cube: Option<Cube>,
    pub physics: Option<Physics>,
    pub focus: bool
}

impl Entity {

    pub fn new_cube(pos: Vector3, color: Color, size: f32, scale: Scale) -> Entity {
       Entity {
           cube: Some(Cube { color: color}),
           physics: None,
           focus: false
       } 
    }

    pub fn random_cube() -> Entity {
        let mut rng = rand::thread_rng();

        Entity  {
            cube: Some(Cube { color: Color::GREEN}),
            physics: Some(Physics { pos: Vector3::new(rng.gen_range(0.0, 100.0), rng.gen_range(0.0, 100.0), rng.gen_range(0.0, 100.0)), color: random_color(rng.gen_range(0, 3)), size: rng.gen_range(1.0, 10.0), scale: random_scale() }),
            focus: false
        }
    }
}

fn random_color(num: i32) -> Color {
    match num {
        0 => Color::RED,
        1 => Color::BLUE,
        2 => Color::GREEN,
        _ => Color::WHITE 
    }
}

fn random_scale() -> Scale {
    let mut rng = rand::thread_rng();

    match rng.gen::<bool>() {
        true => Scale::Grow,
        false => Scale::Shrink,
    }
}