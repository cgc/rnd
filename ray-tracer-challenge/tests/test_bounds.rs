use std::f64::{INFINITY, consts::PI};

use ray_tracer_challenge::*;

#[test]
fn test_sphere() {
    let dir = vector(1., 0., 0.);

    for (o, exp, expbb) in [
        (point(-2., 0., 0.), 2, 2),
        (point(-2., -0.5, 0.), 2, 2),
        (point(-2., 0., 0.5), 2, 2),
        (point(-2., 0.9, 0.9), 0, 2), // Misses at corner.
    ] {
        let r = ray(&o, &dir);
        let s = sphere();
        let bb = s.as_local_shape().local_bounding_box();
        assert_eq!(s.local_intersect(&r).len(), exp);
        assert_eq!(bb.local_intersect_ts(&r).len(), expbb)
    }
}

#[test]
fn test_cone() {
    let dir = vector(1., 0., 0.);

    let mut ice_cream = cone();
    ice_cream.set_minimum(&0.);
    ice_cream.set_maximum(&10.);

    let mut spike = cone();
    spike.set_minimum(&-10.);
    spike.set_maximum(&0.);

    let mut hourglass = cone();
    hourglass.set_minimum(&-10.);
    hourglass.set_maximum(&10.);

    for (shape, o, exp, expbb) in [

        (&ice_cream, point(-20., 9., 8.), 2, 2),
        (&ice_cream, point(-20., 8., 9.), 0, 2),
        (&ice_cream, point(-20., EPSILON, 0.), 2, 2),
        (&ice_cream, point(-20., -EPSILON, 0.), 0, 0),
        (&ice_cream, point(-20., -9., -8.), 0, 0),
        (&ice_cream, point(-20., -8., -9.), 0, 0),

        (&spike, point(-20., 9., 8.), 0, 0),
        (&spike, point(-20., 8., 9.), 0, 0),
        (&spike, point(-20., EPSILON, 0.), 0, 0),
        (&spike, point(-20., -EPSILON, 0.), 2, 2),
        (&spike, point(-20., -8., -9.), 0, 2),
        (&spike, point(-20., -9., -8.), 2, 2),

        // Combines the above two
        (&hourglass, point(-20., 9., 8.), 2, 2),
        (&hourglass, point(-20., 8., 9.), 0, 2),
        (&hourglass, point(-20., EPSILON, 0.), 2, 2),
        (&hourglass, point(-20., -EPSILON, 0.), 2, 2),
        (&hourglass, point(-20., -8., -9.), 0, 2),
        (&hourglass, point(-20., -9., -8.), 2, 2),
    ] {
        let r = ray(&o, &dir);
        let bb = shape.as_local_shape().local_bounding_box();
        assert_eq!(shape.local_intersect(&r).len(), exp);
        assert_eq!(bb.local_intersect_ts(&r).len(), expbb)
    }
}

#[test]
fn test_triangle() {
    let dir = vector(1., 0., 0.);
    let s = triangle(
        &point(-1., 0., 1.),
        &point(0., 1., -1.),
        &point(1., -1., 0.),
    );
    let bb = s.as_local_shape().local_bounding_box();

    assert!(bb == BoundingBox::new(
        point(-1., -1., -1.),
        point(1., 1., 1.),
    ));

    for (o, exp, expbb) in [
        (point(-2., 0., 0.), 1, 2),
        (point(-2., 0.9, 0.9), 0, 2), // Misses at corner.
    ] {
        let r = ray(&o, &dir);
        assert_eq!(s.local_intersect(&r).len(), exp);
        assert_eq!(bb.local_intersect_ts(&r).len(), expbb)
    }
}

#[test]
fn test_group() {
    let mut g = group();

    let mut s1 = cube();
    s1.set_transform(&translation(-10., -3., -5.));
    add_child(&mut g, &mut s1);

    let mut s2 = cube();
    s2.set_transform(&translation(10., 2., -1.));
    add_child(&mut g, &mut s2);

    let bb = g.as_local_shape().local_bounding_box();
    assert_eq!(bb, BoundingBox::new(
        point(-11., -4., -6.),
        point(11., 3., 0.),
    ));
}

#[test]
fn test_bounding_box_corners() {
    let bb = BoundingBox::new(
        point(-1., -2., -3.),
        point(1., 2., 3.),
    );

    assert_eq!(bb.corners(), [
        point(-1., -2., -3.),
        point(1., -2., -3.),
        point(-1., 2., -3.),
        point(1., 2., -3.),
        point(-1., -2., 3.),
        point(1., -2., 3.),
        point(-1., 2., 3.),
        point(1., 2., 3.),
    ]);
}

#[test]
fn test_bounding_box_build() {
    let mut bb = BoundingBox::new_empty();
    assert_eq!(bb.surface_area(), 0.);

    // First one sets bounds.
    bb.include_bb(&BoundingBox::new(
        point(1., 2., 3.),
        point(4., 5., 6.),
    ));
    assert_eq!(bb.min, point(1., 2., 3.));
    assert_eq!(bb.max, point(4., 5., 6.));
    assert_eq!(bb.surface_area(), 54.);

    // Second one can update bounds.
    bb.include_bb(&BoundingBox::new(
        point(1., 2., 0.),
        point(7., 5., 6.),
    ));
    assert_eq!(bb.min, point(1., 2., 0.));
    assert_eq!(bb.max, point(7., 5., 6.));
    assert_eq!(bb.surface_area(), 2. * (36. + 18. + 18.));

    // Empty one keeps finite bounds.
    bb.include_bb(&BoundingBox::new_empty());
    assert_eq!(bb.min, point(1., 2., 0.));
    assert_eq!(bb.max, point(7., 5., 6.));
    assert_eq!(bb.surface_area(), 2. * (36. + 18. + 18.));

    // Testing infinite shape.
    bb.include_bb(&plane().as_local_shape().local_bounding_box());
    assert_eq!(bb.min, point(-INFINITY_FOR_BOUNDS, 0., -INFINITY_FOR_BOUNDS));
    assert_eq!(bb.max, point(INFINITY_FOR_BOUNDS, 5., INFINITY_FOR_BOUNDS));
    assert_eq!(bb.surface_area(), INFINITY);
}

#[test]
fn test_bounding_box_build_transformed_plane() {
    let mut bb = BoundingBox::new_empty();
    assert_eq!(bb.surface_area(), 0.);

    let mut s = plane();
    s.set_transform(&rotation_x(PI/4.).rotate_y(PI/4.));
    bb.include_transformed_shape(&s);
    let half_sqrt_2 = 2_f64.sqrt() / 2.;
    let max = point(INFINITY_FOR_BOUNDS * (half_sqrt_2 + 0.5), INFINITY_FOR_BOUNDS * half_sqrt_2, INFINITY_FOR_BOUNDS * (half_sqrt_2 + 0.5));
    assert_eq!(bb.min, point(-max.x, -max.y, -max.z));
    assert_eq!(bb.max, max);
    assert_eq!(bb.surface_area(), INFINITY);
}

#[test]
fn test_bvh_build() {
    let mut shapes = vec![];
    for _ in 0..3 {
        let mut s = sphere();
        s.set_transform(&translation(-10., 0., 0.));
        shapes.push(s);
        let mut s = sphere();
        s.set_transform(&translation(10., 0., 0.));
        shapes.push(s);
    }
    let bb = BoundingBox::from_transformed_shapes(&shapes.iter().map(|s| s).collect::<Vec<&Shape>>());
    let b = BVHNode::build(bb, &shapes);
    assert!(b.child_nodes().is_some());
    let (left, right) = b.child_nodes().unwrap();
    assert!(left.shapes().unwrap().len() == 3);
    assert!(right.shapes().unwrap().len() == 3);
}

#[test]
fn test_bvh_intersect() {
    let mut shapes = vec![];
    for idx in 0..3 {
        let y = (idx as f64 - 1.) / 2.;
        let mut s = sphere();
        s.set_transform(&translation(-10., y, 0.));
        shapes.push(s);
        let mut s = sphere();
        s.set_transform(&translation(10., y, 0.));
        shapes.push(s);
    }
    let bb = BoundingBox::from_transformed_shapes(&shapes.iter().map(|s| s).collect::<Vec<&Shape>>());
    let b = BVHNode::build(bb, &shapes);

    let xs = intersections(b.intersect(&ray(&point(-10., 0., -2.), &vector(0., 0., 1.))));
    assert_eq!(xs.count, 6);
    let offset_circle_diff = 0.75_f64.sqrt();
    assert_eq!(
        xs.data.iter().map(|i| i.t).collect::<Vec<f64>>(),
        vec![1., 2.-offset_circle_diff, 2.-offset_circle_diff, 2.+offset_circle_diff, 2.+offset_circle_diff, 3.],
    );
}
