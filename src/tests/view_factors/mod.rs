mod functional_tests;

use crate::{surfaces::Surface, Scene};

use glam::f32::Vec3;

/// These functions calculate the view factor analytically and using simulation with one million rays

/// Analytical Source: http://webserver.dmt.upm.es/~isidoro/tc3/Radiation%20View%20factors.pdf
/// h is ratio of cylinder radius to cylinder half-height
/// Sphere radius is irrellevent as long as it is less than cylinder radius
pub fn sphere_in_cylinder(h: f32) -> (f32, f32) {
    let mut scene = Scene::default();

    scene.add_surface(Surface::new_sphere(Vec3::ZERO, 0.1));

    scene.add_surface(Surface::new_cylinder(Vec3::ZERO, 1.0, 2. * h, Vec3::Z));

    let v = scene.view_factors_for_surface(0, 1_000_000)[1];
    let v_an = h / (1.0f32 + h * h).sqrt();

    (v_an, v)
}

/// Analytical Source: http://webserver.dmt.upm.es/~isidoro/tc3/Radiation%20View%20factors.pdf
/// h is ratio of distance to disk radius
/// r is ratio of sphere radius to disk radius
pub fn disk_to_sphere(h: f32, r: f32) -> (f32, f32) {
    assert!(h >= 1.0);
    assert!(r <= h);

    let mut scene = Scene::default();

    scene.add_surface(Surface::new_disk(h * Vec3::X, 1.0, -Vec3::X));

    scene.add_surface(Surface::new_sphere(Vec3::ZERO, r));

    let v = scene.view_factors_for_surface(0, 1_000_000)[1];
    let v_an = 2. * r * r * (1. - 1. / (1. + 1. / (h * h)).sqrt());

    (v_an, v)
}

/// Analytical Source: http://webserver.dmt.upm.es/~isidoro/tc3/Radiation%20View%20factors.pdf
// h is cylinder center to sphere center divided by sphere radius
pub fn small_cylinder_to_sphere(h: f32) -> (f32, f32) {
    assert!(h >= 1.);

    let mut scene = Scene::default();

    scene.add_surface(Surface::new_cylinder(
        Vec3::X * h,
        0.00001,
        0.00002,
        Vec3::X,
    ));

    scene.add_surface(Surface::new_sphere(Vec3::ZERO, 1.0));

    let v = scene.view_factors_for_surface(0, 1_000_000)[1];

    let s = (1. - 1. / (h * h)).sqrt();
    let pi = std::f32::consts::PI;
    let v_an = 0.5 - s / (pi * h) - s.asin() / pi;

    (v_an, v)
}

/// Analytical Source: http://webserver.dmt.upm.es/~isidoro/tc3/Radiation%20View%20factors.pdf
/// h is spheres center to center distance divided by large sphere radius
pub fn small_sphere_to_sphere(h: f32) -> (f32, f32) {
    assert!(h >= 1.);

    let mut scene = Scene::default();

    scene.add_surface(Surface::new_sphere(Vec3::X * h, 0.00001));

    scene.add_surface(Surface::new_sphere(Vec3::ZERO, 1.0));

    let v = scene.view_factors_for_surface(0, 1_000_000)[1];
    let v_an = 0.5 * (1. - (1. - 1. / (h * h)).sqrt());

    (v_an, v)
}

/// Analytical Source: http://webserver.dmt.upm.es/~isidoro/tc3/Radiation%20View%20factors.pdf
/// h is center to center distance divided by cylinder radius
pub fn cylinder_to_cylinder(h: f32) -> (f32, f32) {
    assert!(h >= 2.);

    let mut scene = Scene::default();

    scene.add_surface(Surface::new_cylinder(Vec3::ZERO, 1.0, 10_000.0, Vec3::Y));

    scene.add_surface(Surface::new_cylinder(Vec3::X * h, 1.0, 10_000.0, Vec3::Y));

    let v = scene.view_factors_for_surface(0, 1_000_000)[1];

    let pi = std::f32::consts::PI;
    let v_an = ((h * h - 4.).sqrt() - h + 2. * (2. / h).asin()) / (2. * pi);

    (v_an, v)
}

/// h is distance between strip and cylinder center divided by radius
/// v is height of strip divided by cylinder diameter
pub fn strip_to_cylinder(h: f32, v: f32) -> (f32, f32) {
    assert!(h >= 1.);

    let mut scene = Scene::default();

    scene.add_surface(Surface::new_rectangle(
        Vec3::X * h,
        2. * v,
        10_000.0,
        -Vec3::X,
        Vec3::Z,
    ));

    scene.add_surface(Surface::new_cylinder(Vec3::ZERO, 1.0, 10_000.0, Vec3::Y));

    let f = scene.view_factors_for_surface(0, 1_000_000)[1];
    let f_an = (v / h).atan() / v;

    (f_an, f)
}

/// Analytical Source: http://webserver.dmt.upm.es/~isidoro/tc3/Radiation%20View%20factors.pdf
/// h is height of rod divided by disk radius
pub fn rod_to_coaxial_disk(h: f32) -> (f32, f32) {
    let mut scene = Scene::default();

    scene.add_surface(Surface::new_cylinder(0.5 * h * Vec3::Z, 0.0001, h, Vec3::Z));

    scene.add_surface(Surface::new_disk(Vec3::ZERO, 1.0, Vec3::Z));

    let v = scene.view_factors_for_surface(0, 1_000_000)[1];

    let pi = std::f32::consts::PI;
    let v_an = 0.25 - 0.5 / pi * ((h * h - 1.) / (h * h + 1.)).asin();

    (v_an, v)
}

/// Analytical Source: http://webserver.dmt.upm.es/~isidoro/tc3/Radiation%20View%20factors.pdf
/// x is width of plate divided by gap
/// y is height of plate divided by gap
pub fn equal_rectangular_plates(x: f32, y: f32) -> (f32, f32) {
    let mut scene = Scene::default();

    scene.add_surface(Surface::new_rectangle(Vec3::X, x, y, -Vec3::X, Vec3::Z));

    scene.add_surface(Surface::new_rectangle(Vec3::ZERO, x, y, Vec3::X, Vec3::Z));

    let v = scene.view_factors_for_surface(0, 1_000_000)[1];

    let x1 = (1. + x * x).sqrt();
    let y1 = (1. + y * y).sqrt();

    let a = (x1 * x1 * y1 * y1 / (x1 * x1 + y1 * y1 - 1.)).ln();
    let b = 2. * x * (y1 * (x / y1).atan() - x.atan());
    let c = 2. * y * (x1 * (y / x1).atan() - y.atan());

    let v_an = (1. / (std::f32::consts::PI * x * y)) * (a + b + c);

    (v_an, v)
}

/// Analytical Source: http://webserver.dmt.upm.es/~isidoro/tc3/Radiation%20View%20factors.pdf
/// r1 is radius of disk 1 divided by gap
/// r2 is radius of disk 2 divided by gap
pub fn unequal_disks(r1: f32, r2: f32) -> (f32, f32) {
    let mut scene = Scene::default();

    scene.add_surface(Surface::new_disk(Vec3::X, r1, Vec3::NEG_X));

    scene.add_surface(Surface::new_disk(Vec3::ZERO, r2, Vec3::X));

    let v = scene.view_factors_for_surface(0, 1_000_000)[1];

    let x = 1. + 1. / (r1 * r1) + r2 * r2 / (r1 * r1);
    let y = (x * x - 4. * r2 * r2 / (r1 * r1)).sqrt();
    let v_an = (x - y) / 2.;

    (v_an, v)
}

/// Analytical Source: http://www.thermalradiation.net/sectionc/C-139.html
/// r is cone radius divided by sphere radius
/// s is distance from sphere surface to cone tip
/// w is cone half angle (in radians)
pub fn sphere_to_coaxial_cone(r: f32, s: f32, w: f32) -> (f32, f32) {
    assert!(w >= (1. / (s + 1.)).asin());

    let mut scene = Scene::default();

    let h = r / w.tan();

    scene.add_surface(Surface::new_sphere(Vec3::X, 1.));

    scene.add_surface(Surface::new_cone((h + s) * Vec3::NEG_X, r, h, Vec3::X));

    let v = scene.view_factors_for_surface(0, 1_000_000)[1];

    let x = r / (1. + s + r / w.tan());
    let denom = (1. + x * x).sqrt();
    let v_an = 0.5 * (1. - 1. / denom);

    (v_an, v)
}

/// Analytical Source: http://www.thermalradiation.net/sectionc/C-139.html
/// r is cone radius divided by sphere radius
/// s is distance from sphere surface to cone tip
/// w is cone half angle (in radians)
pub fn coaxial_cone_to_sphere(r: f32, s: f32, w: f32) -> (f32, f32) {
    assert!(w >= (1. / (s + 1.)).asin());

    let mut scene = Scene::default();

    let h = r / w.tan();

    scene.add_surface(Surface::new_cone((h + s) * Vec3::NEG_X, r, h, Vec3::X));

    scene.add_surface(Surface::new_sphere(Vec3::X, 1.));

    let v = scene.view_factors_for_surface(0, 1_000_000)[1];

    let x = r / (1. + s + r / w.tan());
    let denom = (1. + x * x).sqrt();

    let a_sphere = 4. * std::f32::consts::PI;
    let a_cone = std::f32::consts::PI * r * (h * h + r * r).sqrt();
    let v_an_sphere = 0.5 * (1. - 1. / denom);
    let v_an = a_sphere / a_cone * v_an_sphere;

    (v_an, v)
}
