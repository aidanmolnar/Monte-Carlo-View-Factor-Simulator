use glam::Vec3;
use monte_carlo_view_factors::{surfaces::Surface, Scene};

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

    scene.debug_rays_from_surface(0, 100);
}
