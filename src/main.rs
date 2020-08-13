extern crate cgmath;
extern crate mint;
extern crate three;

// use cgmath::prelude::*;
use three::Object;
use cgmath::{Angle, Decomposed, One, Quaternion, Rad, Rotation3, Transform, Vector3};

// use mint::*;
pub mod packing;
mod sweeping_iterator;

fn main() {
    let max_points: u32 = 5000;

    let mut win = three::Window::new("Fibonacci sphere packing");
    let cam = win.factory.perspective_camera(75.0, 1.0 .. 50.0);
    cam.set_position([0.0, 0.0, 5.0]);

    let mut controls = three::controls::Orbit::builder(&cam)
        .position([0.0, 3.0, -1.0])
        .target([0.0, 0.0, -1.0])
        .up([0.0, 0.0, -1.0])
        .build();

    let light = win.factory.point_light(0xffffff, 1.0);
    light.set_position([10.0, 10.0, 10.0]);
    win.scene.add(&light);
    
    let light2 = win.factory.point_light(0xffffff, 1.0);
    light2.set_position([-10.0, -10.0, -10.0]);
    win.scene.add(&light2);
        
    // let mut sweep = sweeping_iterator::SweepingIterator::new(max_points as i32);

    let mut num_points: u32 = 5;
    let mut num_points_idx: usize = 0;
    let num_points_idx_max: usize = 5000;
    let power_idx: f32 = 1.8;

    let mut idx_direction: i8 = 0;

    let num_points_arr: Vec<u32> = (0..num_points_idx_max as u32).collect::<Vec<u32>>().to_vec();
    // println!("{:#?}", num_points_arr);
    let mut spheres: Vec<three::Mesh> = vec!{};

    let group: three::Group = win.factory.group();
    let rot = Quaternion::from_axis_angle(Vector3::unit_y(), Rad::turn_div_4());
    let mut changed = true;
    group.set_orientation(rot);
    group.set_scale(2.0);
    
    win.scene.add(&group);
    let geometry = three::Geometry::uv_sphere(0.05, 8, 8);
    let upload_geometry = win.factory.upload_geometry(geometry);


    while win.update() && !win.input.hit(three::KEY_ESCAPE) {
        // println!("{}", num_points);
        controls.update(&win.input);
        let keys_hit = win.input.keys_hit();
        if keys_hit.len() > 0 { 
            let key_hit: three::Key = keys_hit[0];
            println!("{:#?}", key_hit);
            if key_hit == three::Key::A {
                idx_direction = -1;
            } else if key_hit == three::Key::D {
                idx_direction = 1;
            } else {
                idx_direction = 0;
            }
            
        }
        

        let points: Vec<mint::Point3<f32>> = packing::points_on_sphere_fib(num_points, 1.0);
        // println!("{}", num_points);

        if changed {
            // println!("changed, resetting group: n: {}, n_idx: {}", num_points, num_points_idx);
            // let sync = win.scene.sync_guard();
            if spheres.len() > 0 {
                spheres.iter().for_each(|sphere| group.remove(sphere))
            }

            let mut i: u32 = 0;

            spheres = points.iter().map(|&point| {
                i += 1;
                let mut material = three::material::Lambert { color: 0x80ff80, flat: false };
                if i % 20 == 0 { material = three::material::Lambert { color: 0xff0080, flat: false }; }
                let mesh = win.factory.create_instanced_mesh(&upload_geometry, material.clone());
        
                // let mesh = win.factory.mesh(geometry, material.clone());
                // println!("{:#?}", mesh);
                mesh.set_position(point);
                return mesh;
            }).collect::<Vec<three::Mesh>>();
        
            spheres.iter().for_each(|sphere| group.add(sphere));
        }

        if idx_direction != 0 {
            num_points_idx = (num_points_idx as i32 + idx_direction as i32).min(num_points_idx_max as i32).max(0) as usize 
        }
        win.render(&cam);
    }
}
