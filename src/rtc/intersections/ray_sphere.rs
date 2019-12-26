use crate::*;

impl LocalRayIntersection for Sphere {
    fn local_ray_intersection(&self, ray: Ray) -> Vec<Intersection> {
        let sphere_to_ray = ray.origin - point(0., 0., 0.);
        let dir = ray.direction;
        let a = dir.dot(dir);
        let b = 2. * dir.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.;

        let discriminant = b.powf(2.) - 4. * a * c;

        if discriminant < 0. {
            return Intersection::none();
        }

        let t1 = (-b - discriminant.sqrt()) / (2. * a);
        let t2 = (-b + discriminant.sqrt()) / (2. * a);

        vec![
            Intersection::ray_sphere(&self, t1),
            Intersection::ray_sphere(&self, t2),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersecting_rays_with_unit_spheres() {
        let s = Sphere::unit();

        // normal hit
        let r1 = Ray::new(point(0., 0., -5.), vec(0., 0., 1.));
        let i1 = s.ray_intersection(r1);
        assert_eq!(vec![4., 6.], i1.iter().map(|x| x.t).collect::<Vec<_>>());

        // tangent hit still produces two collision points
        let r2 = Ray::new(point(0., 1., -5.), vec(0., 0., 1.));
        let i2 = s.ray_intersection(r2);
        assert_eq!(vec![5., 5.], i2.iter().map(|x| x.t).collect::<Vec<_>>());

        // missing entirely
        let r3 = Ray::new(point(0., 2., -5.), vec(0., 0., 1.));
        let i3 = s.ray_intersection(r3);
        assert_eq!(0, i3.len());

        // a ray can intersect from behind its origin point
        let r4 = Ray::new(point(0., 0., 5.), vec(0., 0., 1.));
        let i4 = s.ray_intersection(r4);
        assert_eq!(vec![-6., -4.], i4.iter().map(|x| x.t).collect::<Vec<_>>());
    }

    #[test]
    fn intersecting_rays_with_transformed_spheres() {
        let r1 = Ray::new(point(0., 0., -5.), vec(0., 0., 1.));
        let s1 = Sphere::pos_r(point(0., 0., 0.), 2.);
        let i1 = s1.ray_intersection(r1);
        assert_eq!(vec![3., 7.], i1.iter().map(|x| x.t).collect::<Vec<_>>());

        let r2 = Ray::new(point(0., 0., -5.), vec(0., 0., 1.));
        let s2 = Sphere::pos_r(point(5., 0., 0.), 1.);
        let i2 = s2.ray_intersection(r2);
        assert_eq!(0, i2.len());
    }
}
