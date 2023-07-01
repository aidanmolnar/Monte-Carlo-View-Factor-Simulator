use glam::Vec3;
use monte_carlo_view_factors::{surfaces::Surface, Scene};

use std::time::Instant;

fn main() {
    let mut scene = Scene::default();

    // Add +x face
    scene.add_surface(
        Surface::new_disk(Vec3::X * 0.5, 1., -Vec3::X)
            .to_gray_body(0.2)
            .set_specular(),
    );

    // Add -x face
    scene.add_surface(
        Surface::new_disk(-Vec3::X * 0.5, 3., Vec3::X)
            .to_gray_body(0.2)
            .set_specular(),
    );

    let num_rays = 1_000_000;

    let start = Instant::now();
    dbg!(scene.view_factors_for_surface(0, num_rays));
    let duration = start.elapsed();

    println!("Time elapsed computing view factors: {:?}", duration);

    let start = Instant::now();
    dbg!(scene.view_factors_for_surface_parallel(0, num_rays));
    let duration = start.elapsed();

    println!(
        "Time elapsed computing view factors parallel: {:?}",
        duration
    );
}
