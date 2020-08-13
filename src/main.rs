extern crate cgmath;
extern crate mint;
extern crate three;

// use cgmath::prelude::*;
use three::Object;
// use mint::*;
pub mod packing;
mod sweeping_iterator;

fn main() {
    let max_points: u32 = 100;

    let mut win = three::Window::new("Fibonacci sphere packing");
    let cam = win.factory.perspective_camera(75.0, 1.0 .. 50.0);
    cam.set_position([0.0, 0.0, 5.0]);

    let light = win.factory.point_light(0xffffff, 1.0);
    light.set_position([0.0, -10.0, 10.0]);
    win.scene.add(&light);

    // let mut sweep = sweeping_iterator::SweepingIterator::new(max_points as i32);

    // let num_points = sweep.next().unwrap();
    // let num_points = sweep.next().unwrap();
    let mut num_points = 5;
    let mut spheres: Vec<three::Mesh> = vec!{};
    while win.update() && !win.input.hit(three::KEY_ESCAPE) {
        // println!("{}", num_points);
        let points: Vec<mint::Point3<f32>> = packing::points_on_sphere_fib(num_points as u32);

        // let sync = win.scene.sync_guard();
        if spheres.len() > 0 {
            spheres.iter().for_each(|sphere| win.scene.remove(sphere))
        }

        spheres = points.iter().map(|&point| {
            let geometry = three::Geometry::uv_sphere(0.1, 10, 10);
            let material = three::material::Lambert { color: 0x80ff80, flat: false};
    
            let mesh = win.factory.mesh(geometry, material.clone());
            // println!("{:#?}", mesh);
            mesh.set_position(point);
            return mesh;
        }).collect::<Vec<three::Mesh>>();
    
        spheres.iter().for_each(|sphere| win.scene.add(&sphere));
    
        if let Some(diff) = win.input.timed(three::AXIS_LEFT_RIGHT) {
            println!("{}", diff);
            if diff > 0.0 {
                num_points = (num_points + 1).min(max_points);
            } else if diff < 0.0 && num_points > 0 {
                num_points = (num_points - 1).max(0);
            }
        }
        win.render(&cam);
    }
}
