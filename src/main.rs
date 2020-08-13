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
    let max_points: u32 = 500;

    let mut win = three::Window::new("Fibonacci sphere packing");
    let cam = win.factory.perspective_camera(75.0, 1.0 .. 50.0);
    cam.set_position([0.0, 0.0, 5.0]);

    let mut controls = three::controls::Orbit::builder(&cam)
        .position([0.0, 3.0, -1.0])
        .target([0.0, 0.0, -1.0])
        .up([0.0, 0.0, -1.0])
        .build();

    let light = win.factory.point_light(0xffffff, 1.0);
    light.set_position([0.0, -10.0, 10.0]);
    win.scene.add(&light);

    // let mut sweep = sweeping_iterator::SweepingIterator::new(max_points as i32);

    // let num_points = sweep.next().unwrap();
    // let num_points = sweep.next().unwrap();
    let mut num_points: u32 = 5;
    let mut spheres: Vec<three::Mesh> = vec!{};

    let group: three::Group = win.factory.group();
    let rot = Quaternion::from_axis_angle(Vector3::unit_y(), Rad::turn_div_4());
    let mut changed = true;
    group.set_orientation(rot);
    group.set_scale(2.0);
    
    win.scene.add(&group);

    while win.update() && !win.input.hit(three::KEY_ESCAPE) {
        // println!("{}", num_points);
        controls.update(&win.input);

        let points: Vec<mint::Point3<f32>> = packing::points_on_sphere_fib(num_points, 0.7);
        // println!("{}", num_points);

        if changed {
            println!("changed, resetting group");
            // let sync = win.scene.sync_guard();
            if spheres.len() > 0 {
                spheres.iter().for_each(|sphere| group.remove(sphere))
            }

            spheres = points.iter().map(|&point| {
                let geometry = three::Geometry::uv_sphere(0.05, 4, 4);
                let material = three::material::Lambert { color: 0x80ff80, flat: false };
        
                let mesh = win.factory.mesh(geometry, material.clone());
                // println!("{:#?}", mesh);
                mesh.set_position(point);
                return mesh;
            }).collect::<Vec<three::Mesh>>();
        
            spheres.iter().for_each(|sphere| group.add(sphere));
        }
    
        if let Some(diff) = win.input.timed(three::AXIS_LEFT_RIGHT) {
            // println!("{}", diff);
            if diff > 0.0 {
                num_points = (num_points + 1).min(max_points);
                changed = true;
            } else if diff < 0.0 && num_points > 0 {
                num_points = (num_points - 1).max(0);
                changed = true;
            } else {
                changed = false;
            }
        } else {
            // println!("changing = false");
            if changed == true { changed = false};
        }
        win.render(&cam);
    }
}
