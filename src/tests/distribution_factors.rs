use crate::{surfaces::Surface, Scene};

use glam::Vec3;

/// e1 and e2 are emissivities of plates
/// Analytical Source:
/// Work from equations on slide 121
/// https://tfaws.nasa.gov/wp-content/uploads/TFAWS2015-SC-Form-Factors-and-Grey-Bodies.pdf
/// https://www.desmos.com/calculator/obqvvlfwpv
pub fn infinite_plates(e1: f32, e2: f32) -> (f32, f32) {
    let mut scene = Scene::default();

    let length = 10_000.;

    scene.add_surface(
        Surface::new_rectangle(Vec3::ZERO, length, length, Vec3::Z, Vec3::X).to_gray_body(e1),
    );

    scene.add_surface(
        Surface::new_rectangle(Vec3::Z, length, length, Vec3::NEG_Z, Vec3::NEG_X).to_gray_body(e2),
    );

    let v = scene.view_factors_for_surface(0, 10_000)[1];

    let v_an = e2 / (e1 + e2 - e1 * e2);

    (v, v_an)
}

/// Analytical Source:
/// Work from equations on slide 170
/// https://tfaws.nasa.gov/wp-content/uploads/TFAWS2015-SC-Form-Factors-and-Grey-Bodies.pdf
pub fn cube_enclosure(e: f32) -> Vec<f32> {
    let mut scene = Scene::default();

    let i = Vec3::X;
    let j = Vec3::Y;
    let k = i.cross(j).normalize();

    // Add +x face
    scene.add_surface(Surface::new_rectangle(i * 0.5, 1., 1., -i, j).to_gray_body(e));

    // Add -x face
    scene.add_surface(Surface::new_rectangle(-i * 0.5, 1., 1., i, j).to_gray_body(e));

    // Add +y face
    scene.add_surface(Surface::new_rectangle(j * 0.5, 1., 1., -j, -i).to_gray_body(e));

    // Add -y face
    scene.add_surface(Surface::new_rectangle(-j * 0.5, 1., 1., j, i).to_gray_body(e));

    // Add +z face
    scene.add_surface(Surface::new_rectangle(k * 0.5, 1., 1., -k, -i).to_gray_body(e));

    // Add -z face
    scene.add_surface(Surface::new_rectangle(-k * 0.5, 1., 1., k, i).to_gray_body(e));

    //scene.debug_rays_from_surface(0, 10000);
    let v = scene.view_factors_for_surface(2, 10_000);

    dbg!(v)
}

#[cfg(test)]
mod test {
    // Helper function for computing percent error
    fn percent_error((expected, measured): (f32, f32)) -> f32 {
        100. * (measured - expected) / expected
    }

    #[test]
    fn infinite_plates() {
        assert!(percent_error(super::infinite_plates(0.2, 0.7)) < 1.)
    }
}
