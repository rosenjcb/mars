use raylib::{color::Color, math::Vector3, camera::Camera3D};
use crate::entity::Entity;

pub struct Physics {
    pub pos: Vector3,
    pub color: Color,
    pub size: f32,
    pub scale: Scale,
}

pub enum Scale {
   Grow,
   Shrink 
}

pub fn apply_physics(entity: &mut Entity, camera: &Camera3D) {

    is_looking(entity, camera); 
    // println!("{:?}", entity.focus);
    if entity.focus { return; }
    match &mut entity.physics {
        Some(p) => scale_cube(p),
        None => ()
    };
}

fn is_looking(entity: &mut Entity, camera: &Camera3D) {

    match &mut entity.physics {
        Some(physics) => { 
            let obj_vec: Vector3 = Vector3::from(camera.position - physics.pos).normalized();

            let target_vec = Vector3::from(camera.position - camera.target).normalized();
            let cos = target_vec.dot(obj_vec);

            let tolerance: f32 = 10.0;
            //&& cube.pos.distance_to(camera.position) < 0.0

            let dist = physics.pos.distance_to(camera.position).abs();
            if cos.acos() * 180.0 / 3.14 <= tolerance && dist < 30.0 {
                entity.focus = true;
            } else {
                entity.focus = false;
            } 
            // entity.focus = true;
        }
        None => () 
    }
}

fn scale_cube(physics: &mut Physics) {
    physics.scale = match physics.scale {
        Scale::Grow => if physics.size >= 10.0 { Scale::Shrink} else { Scale::Grow },
        Scale::Shrink => if physics.size <= 1.0 { Scale::Grow } else { Scale::Shrink }
    };

    physics.size = match physics.scale {
        Scale::Grow => physics.size + 0.005,
        Scale::Shrink => physics.size - 0.005
    }
}