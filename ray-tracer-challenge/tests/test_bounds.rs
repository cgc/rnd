use std::{f64::{INFINITY, consts::PI, NEG_INFINITY}};

use ray_tracer_challenge::*;

fn times_from_intersections(xs: &Vec<Intersection>) -> Vec<f64> {
    xs.iter().map(|i| i.t).collect::<Vec<f64>>()
}

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
        times_from_intersections(&xs.data),
        vec![1., 2.-offset_circle_diff, 2.-offset_circle_diff, 2.+offset_circle_diff, 2.+offset_circle_diff, 3.],
    );
}

#[test]
fn test_bvh_traversal_policies() {
    let mut p_all = BVHTraversalPolicy::new_all_hits();
    let mut p_closest = BVHTraversalPolicy::new_closest_hit();
    let mut p_any = BVHTraversalPolicy::new_any_hit();

    let s = cube();
    let bb = s.as_local_shape().local_bounding_box();
    let r = ray(&point(0., 0., -10.), &vector(0., 0., 1.));

    // Testing hits behind.
    let xs = vec![intersection(-1., &s)];
    for _ in 0..2 {
        p_all.add_intersections(&mut xs.clone());
        p_closest.add_intersections(&mut xs.clone());
        p_any.add_intersections(&mut xs.clone());
    }

    assert_eq!(times_from_intersections(&p_all.intersections()), [-1., -1.]);
    assert_eq!(times_from_intersections(&p_closest.intersections()), []);
    assert_eq!(times_from_intersections(&p_any.intersections()), []);

    assert_eq!(p_all.should_traverse(&bb, &r), (true, 9.));
    assert_eq!(p_closest.should_traverse(&bb, &r), (true, 9.));
    assert_eq!(p_any.should_traverse(&bb, &r), (true, 9.));

    // Testing hit after cube.

    let xs = vec![intersection(20., &s)];
    p_all.add_intersections(&mut xs.clone());
    p_closest.add_intersections(&mut xs.clone());
    p_any.add_intersections(&mut xs.clone());

    assert_eq!(times_from_intersections(&p_all.intersections()), [-1., -1., 20.]);
    assert_eq!(times_from_intersections(&p_closest.intersections()), [20.]);
    assert_eq!(times_from_intersections(&p_any.intersections()), [20.]);

    assert_eq!(p_all.should_traverse(&bb, &r), (true, 9.));
    assert_eq!(p_closest.should_traverse(&bb, &r), (true, 9.));
    assert_eq!(p_any.should_traverse(&bb, &r), (false, NEG_INFINITY)); // This blocks future hits, since it has had one.

    // Testing hit inside cube.

    let xs = vec![intersection(10., &s)];
    p_all.add_intersections(&mut xs.clone());
    p_closest.add_intersections(&mut xs.clone());
    p_any.add_intersections(&mut xs.clone());

    assert_eq!(times_from_intersections(&p_all.intersections()), [-1., -1., 20., 10.]);
    assert_eq!(times_from_intersections(&p_closest.intersections()), [10.]);
    assert_eq!(times_from_intersections(&p_any.intersections()), [10.]);

    assert_eq!(p_all.should_traverse(&bb, &r), (true, 9.));
    assert_eq!(p_closest.should_traverse(&bb, &r), (true, 9.));
    assert_eq!(p_any.should_traverse(&bb, &r), (false, NEG_INFINITY));

    // Testing hits in front, but before cube.

    let xs = vec![intersection(1., &s)];
    p_all.add_intersections(&mut xs.clone());
    p_closest.add_intersections(&mut xs.clone());
    p_any.add_intersections(&mut xs.clone());

    assert_eq!(times_from_intersections(&p_all.intersections()), [-1., -1., 20., 10., 1.]);
    assert_eq!(times_from_intersections(&p_closest.intersections()), [1.]);
    assert_eq!(times_from_intersections(&p_any.intersections()), [1.]);

    assert_eq!(p_all.should_traverse(&bb, &r), (true, 9.));
    assert_eq!(p_closest.should_traverse(&bb, &r), (false, 9.)); // Once we have a hit in front of cube, it's blocked.
    assert_eq!(p_any.should_traverse(&bb, &r), (false, NEG_INFINITY));

    // Testing that extra hits from behind don't influence things
    let xs = vec![intersection(-1., &s)];
    p_all.add_intersections(&mut xs.clone());
    p_closest.add_intersections(&mut xs.clone());
    p_any.add_intersections(&mut xs.clone());
    assert_eq!(times_from_intersections(&p_all.intersections()), [-1., -1., 20., 10., 1., -1.]);
    assert_eq!(times_from_intersections(&p_closest.intersections()), [1.]);
    assert_eq!(times_from_intersections(&p_any.intersections()), [1.]);

    // And that extra hits after also don't.
    let xs = vec![intersection(10., &s)];
    p_all.add_intersections(&mut xs.clone());
    p_closest.add_intersections(&mut xs.clone());
    p_any.add_intersections(&mut xs.clone());
    assert_eq!(times_from_intersections(&p_all.intersections()), [-1., -1., 20., 10., 1., -1., 10.]);
    assert_eq!(times_from_intersections(&p_closest.intersections()), [1.]);
    assert_eq!(times_from_intersections(&p_any.intersections()), [10.]);

}

#[test]
fn test_bvh_traversal_policies_closest() {
    let mut s = cube();
    s.set_transform(&translation(0., 0., 5.));
    let bb_front = BoundingBox::from_transformed_shapes(&[&s]);
    s.set_transform(&translation(0., 0., 0.));
    let bb_middle = BoundingBox::from_transformed_shapes(&[&s]);
    s.set_transform(&translation(0., 0., -5.));
    let bb_back = BoundingBox::from_transformed_shapes(&[&s]);
    s.set_transform(&translation(0., 5., 0.));
    let bb_side = BoundingBox::from_transformed_shapes(&[&s]);

    for (bb, exp_trav, hits) in [
        (bb_side, (false, INFINITY), vec![]),
        (bb_back, (false, -6.), vec![]),

        (bb_front, (true, 4.), vec![]),
        (bb_front, (false, 4.), vec![1.]),
        (bb_front, (true, 4.), vec![5.]),
        (bb_front, (true, 4.), vec![10.]),
        // Testing that order of observations doesn't matter
        (bb_front, (false, 4.), vec![1., 10.]),
        (bb_front, (false, 4.), vec![10., 1.]),

        (bb_middle, (true, -1.), vec![]),
        (bb_middle, (true, -1.), vec![0.5]),
        (bb_middle, (true, -1.), vec![1.5]),
    ] {
        let r = ray(&point(0., 0., 0.), &vector(0., 0., 1.));
        let mut p_closest = BVHTraversalPolicy::new_closest_hit();
        p_closest.add_intersections(&mut hits.into_iter().map(|h| intersection(h, &s)).collect::<Vec<Intersection>>());
        assert_eq!(p_closest.should_traverse(&bb, &r), exp_trav);
    }
}

#[test]
fn test_bvh_intersect_policies() {
    let cnear = cube();
    let mut cfar = cube();
    // Slightly up to make this bounding box larger.
    cfar.set_transform(&translation(0., 0.1, 5.));
    let mut clong = cube();
    clong.set_transform(&scaling(1., 1., 5.).translate(0., -2., 0.));

    let b = BVHNode {
        bb: BoundingBox::from_transformed_shapes(&[&cnear, &cfar, &clong]),
        ntype: BVHNodeType::Internal(
            Box::new(BVHNode {
                bb: BoundingBox::from_transformed_shapes(&[&cfar, &clong]),
                ntype: BVHNodeType::Leaf(vec![cfar, clong]),
            }),
            Box::new(BVHNode {
                bb: BoundingBox::from_transformed_shapes(&[&cnear]),
                ntype: BVHNodeType::Leaf(vec![cnear]),
            }),
        ),
    };

    for (r, all_hits, closest_hit, any_hit) in [
        (
            ray(&point(0., 0., -10.), &vector(0., 0., 1.)),
            vec![14., 16., 9., 11.], vec![9.], vec![16.]),
        // Facing the other direction
        (
            ray(&point(0., 0., -10.), &vector(0., 0., -1.)),
            vec![-16., -14., -11., -9.], vec![], vec![]),
        // Complete miss
        (
            ray(&point(0., 10., -10.), &vector(0., 0., 1.)),
            vec![], vec![], vec![]),
        // From inside
        (
            ray(&point(0., 0., 0.), &vector(0., 0., 1.)),
            vec![4., 6., -1., 1.], vec![1.], vec![6.]),
        // From inside, looking down
        (
            ray(&point(0., 0., 0.), &vector(0., -1., 0.)),
            vec![1., 3., -1., 1.], vec![1.], vec![3.]),
    ] {
        let xs = b.intersect(&r);
        assert_eq!(times_from_intersections(&xs), all_hits);
        let xs = b.intersect_closest(&r);
        assert_eq!(times_from_intersections(&xs), closest_hit);
        let xs = b.intersect_any(&r);
        assert_eq!(times_from_intersections(&xs), any_hit);
    }

}
