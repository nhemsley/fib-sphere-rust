extern crate mint;
// use std::ops::*;


pub fn generate_epsilon(n: u32) -> f32 {
  let epsilon: f32;
  if n >= 600000 {
    epsilon = 214.0;
  } else if n>= 400000 {
    epsilon = 75.0
  } else if  n>= 11000 {
    epsilon = 27.0
  } else if  n>= 890 {
    epsilon = 10.0
  } else if  n>= 177 {
    epsilon = 3.33
  } else if  n>= 24 {
    epsilon = 1.33
  } else {
    epsilon = 0.33
  };

  return epsilon;

}

pub fn points_on_sphere_fib(n: u32) -> Vec<mint::Point3<f32>> {

    let _epsilon = generate_epsilon(n);
    // println!("{}", _epsilon);
    let five: f32 = 5.0;
    let golden_ratio: f32 = (1.0 + five.powf(0.5)) / 2.0;
    // println!("{}", golden_ratio);

    let range: Vec<u32> = (0..n).collect();


    // thetas = 2 * pi * i / goldenRatio
    let thetas: Vec<f32> = range.iter().map(|i: &u32| { 2.0 as f32 * std::f32::consts::PI * (*i as f32) / golden_ratio}).collect::<Vec<f32>>();

    let phis: Vec<f32> = range.iter().map(|i: &u32| {
        //  phi_a = arccos(1 - 2*(i+epsilon)/(n-1+2*epsilon))
        let acos_of: f32 = 1.0 - (2.0 * ((*i as f32) + _epsilon) / (n as f32 - 1.0 + 2.0 * _epsilon));
        return acos_of.acos();
    }).collect::<Vec<f32>>();
    // println!("{:#?}", thetas);
    // println!("{:#?}", phis);

    return thetas.iter().zip(phis.iter()).
        map(|(&theta, &phi)| mint::Point3{x: theta.cos() * phi.sin(), y: theta.sin() * phi.sin(), z: phi.cos()}).collect::<Vec<mint::Point3<f32>>>().to_vec();
    // return [vector(cos(theta) * sin(phi), sin(theta) * sin(phi), cos(phi)) for theta, phi in zip(theta_a, phi_a)];
}