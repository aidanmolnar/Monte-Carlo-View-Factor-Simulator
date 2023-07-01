#[cfg(test)]

// Helper function for computing percent error
fn percent_error((expected, measured): (f32, f32)) -> f32 {
    100. * (measured - expected) / expected
}

#[test]
fn sphere_in_cylinder() {
    assert!(percent_error(super::sphere_in_cylinder(1.0)) < 1.)
}

#[test]
fn disk_to_sphere() {
    assert!(percent_error(super::disk_to_sphere(1.0, 1.0)) < 1.)
}

#[test]
fn small_cylinder_to_sphere() {
    assert!(percent_error(super::small_cylinder_to_sphere(5.0)) < 1.)
}

#[test]
fn small_sphere_to_sphere() {
    assert!(percent_error(super::small_sphere_to_sphere(2.0)) < 1.)
}

#[test]
fn cylinder_to_cylinder() {
    assert!(percent_error(super::cylinder_to_cylinder(2.0)) < 1.)
}

#[test]
fn strip_to_cylinder() {
    assert!(percent_error(super::strip_to_cylinder(1.0, 1.0)) < 1.)
}

#[test]
fn rod_to_coaxial_disk() {
    assert!(percent_error(super::rod_to_coaxial_disk(1.0)) < 1.)
}

#[test]
fn equal_rectangular_plates() {
    assert!(percent_error(super::equal_rectangular_plates(1.0, 2.0)) < 1.)
}

#[test]
fn unequal_disks() {
    assert!(percent_error(super::unequal_disks(1.0, 2.0)) < 1.)
}

#[test]
fn sphere_to_coaxial_cone() {
    assert!(percent_error(super::sphere_to_coaxial_cone(1.0, 1.0, 40_f32.to_radians())) < 1.)
}

#[test]
fn coaxial_cone_to_sphere() {
    assert!(percent_error(super::coaxial_cone_to_sphere(2.0, 1.0, 40_f32.to_radians())) < 1.)
}
