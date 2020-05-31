// mod physics;
// mod entity; 
// mod lib;

use raylib::prelude::*;
use raylib::consts::CameraMode;
use mars::entity::Entity;
use mars::physics;

// use crate::entity::Entity;
// }
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("MARS")
        .build();

    let mut game_state: Vec<Entity> = (0..100)
        .map(|_| Entity::random_cube())
        .collect();
    
    let position = Vector3::new(0.0, 10.0, 0.0);
    let target = Vector3::new(-8.0, -5.0, 24.0);
    let up = Vector3::new(0.0, 5.0, 0.0);
    let fovy = 70.0;

    let mut camera = Camera3D::perspective(position, target, up, fovy);

    rl.set_camera_mode(camera, CameraMode::CAMERA_FIRST_PERSON);

    // let mesh = Mesh::gen_mesh_cube(&thread, 1.0, 1.0, 1.0);
    // Model::gengcc
    // mesh.colors_mut()[0] = Color::GOLD;
    // let mesh = unsafe { ffi::GenMeshCube(1.0, 1.0, 1.0) };

    // let m = unsafe { ffi::LoadModelFromMesh(mesh) };

    // // let model = Model(m);
    // // println!("{:?}", std::any::get_ty::<&m>);
    // if m.bindPose.is_null() {
    //     println!("There's a null mesh here!");
    // Model::bind_pose()
    // } else {
    //     println!("Nothing's null");
    // }

    // Ok(Model(m));
    // println!("{:?}", mesh.vertices()); 
    // let model = rl.load_model_from_mesh(&thread, &mesh);

    // match model {
    //     Ok(m) => println!("congrats"),
    //     Err(e) => println!("{:?}", e)
    // }
    // let mat = model.materials();

    // mat[0] = Material::load_materials(filename)
    while !rl.window_should_close() {

        // let model = rl.load_model_from_mesh(&thread, &mesh).unwrap();
        rl.update_camera(&mut camera);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        d.draw_text(format!("Position: {0}, {1}, {2}", camera.position.x, camera.position.y, camera.position.z).as_str(), 0, 0, 20, Color::WHITE);
        d.draw_text(format!("Target: {0}, {1}, {2}", camera.target.x, camera.target.y, camera.target.z).as_str(), 0, 20, 20, Color::WHITE);
        d.draw_text(format!("Up: {0}, {1}, {2}", camera.up.x, camera.up.y, camera.up.z).as_str(), 0, 40, 20, Color::WHITE);

        let mut world = d.begin_mode_3D(&camera);

        // let realWorld = World::new();
        // world.draw_model(&model, Vector3::new(0.0, 0.0, 0.0), 1.0, Color::GOLD);
        for (i, cube) in game_state.iter_mut().enumerate() {
            physics::apply_physics(cube, &camera);
            match &mut cube.physics {
                Some(physics) => { 
                    if !cube.focus {
                        world.draw_cube(physics.pos, physics.size, physics.size, physics.size, physics.color);
                    } else {
                        world.draw_cube(physics.pos, physics.size, physics.size, physics.size, Color::WHITE);
                    }
                }
                None => ()
            }
        }
    }
}