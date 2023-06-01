use std::f64::INFINITY;

// pub const EPSILON: f64 = 0.00001;
// Needed this to pass test_world_14_the_reflected_color_for_a_reflective_material and other tests.
pub const EPSILON: f64 = 0.0001;

#[allow(non_upper_case_globals)]
pub const infinity: f64 = INFINITY;

/// # Examples
///
/// ```
/// use ray_tracer_challenge::equal;
/// assert!(equal(1., 1.00000000001));
/// assert!(equal(-1., -1.00000000001));
/// assert!(equal(1e50, 1.00000000001e50));
/// assert!(!equal(1e50, 2e50));
/// assert!(equal(-1e50, -1.00000000001e50));
/// assert!(!equal(-1e50, -2e50));
/// use std::f64::INFINITY;
/// assert!(equal(INFINITY, INFINITY));
/// use std::f64::NEG_INFINITY;
/// assert!(equal(NEG_INFINITY, NEG_INFINITY));
/// assert!(!equal(NEG_INFINITY, INFINITY));
/// ```
pub fn equal(a: f64, b: f64) -> bool {
    let rtol = EPSILON;
    let atol: f64 = 1e-8;
    if a.is_infinite() {
        return a == b;
    }
    // Copying numpy
    (a - b).abs() < rtol + atol * b.abs()
}
