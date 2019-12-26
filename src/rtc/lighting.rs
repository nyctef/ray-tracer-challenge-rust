use crate::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointLight {
    intensity: Color,
    position: Tuple,
}

impl PointLight {
    pub fn new(intensity: Color, position: Tuple) -> PointLight {
        PointLight {
            intensity,
            position,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhongMaterial {
    pub pattern: Pattern,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl PhongMaterial {
    pub fn solid(
        color: Color,
        ambient: f32,
        diffuse: f32,
        specular: f32,
        shininess: f32,
    ) -> PhongMaterial {
        PhongMaterial {
            pattern: solid(color),
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}

impl Default for PhongMaterial {
    fn default() -> PhongMaterial {
        PhongMaterial::solid(white(), 0.1, 0.9, 0.9, 200.)
    }
}

#[derive(Debug, Clone)]
struct LightHit {
    pub world_point: Tuple,
    pub object_point: Tuple,
    // a point slightly above the surface in world space, used to cast shadow rays
    pub over_point: Tuple,
    pub surface_normal: Tuple,
    pub to_eye: Tuple,
    pub material: PhongMaterial,
    // whether the light ray hit the inside surface of the object.
    // in this case surface_normal is reversed to provide a useful value
    pub inside: bool,
}

fn prepare_computations(hit: &Intersection, ray: Ray) -> Option<LightHit> {
    let world_point = ray.position(hit.t);
    let object_point = hit.obj.world_to_object() * world_point;

    let to_eye = -ray.direction;

    let mut surface_normal = hit.obj.normal_at(world_point);

    let mut inside = false;
    if surface_normal.dot(to_eye) < 0. {
        // surface_normal is pointing away from eye, so
        // we're hitting the inside of the surface
        surface_normal = -surface_normal;
        inside = true;
    }

    // TODO: this epsilon seems a bit big, but smaller values cause lots of artifacts
    let over_point = world_point + (surface_normal.normalize() * 0.0001);

    let material = hit.obj.material();

    Some(LightHit {
        world_point,
        object_point,
        over_point,
        surface_normal,
        material,
        to_eye,
        inside,
    })
}

fn light_ray(world: &World, ray: Ray) -> Option<LightHit> {
    let intersects = world.ray_intersection(ray);
    let hit = Intersection::hit(&intersects)?;
    prepare_computations(hit, ray)
}

fn lighting(
    material: PhongMaterial,
    light: PointLight,
    world_point: Tuple,
    object_point: Tuple,
    eye: Tuple,
    surface_normal: Tuple,
    is_shadow: bool,
) -> Color {
    assert!(world_point.is_point());
    // `eye` is a vector from `surface_position` to the eye position
    assert!(eye.is_vec());
    assert!(surface_normal.is_vec());

    let color = material.pattern.sample_pattern_at(object_point);
    let effective_color = color * light.intensity;
    let light_direction = (light.position - world_point).normalize();
    let ambient = effective_color * material.ambient;

    if is_shadow {
        return ambient;
    }

    let cos_light_angle = light_direction.dot(surface_normal);
    let diffuse = match cos_light_angle {
        x if x < 0. => black(),                      // light is behind surface normal
        x => effective_color * material.diffuse * x, // light is in front, modified by angle
    };
    let specular = match cos_light_angle {
        x if x < 0. => black(),
        _ => {
            let cos_reflection_angle = reflect(-light_direction, surface_normal).dot(eye);
            match cos_reflection_angle {
                x if x < 0. => black(),
                x => {
                    let factor = x.powf(material.shininess);
                    light.intensity * material.specular * factor
                }
            }
        }
    };

    ambient + diffuse + specular
}

fn shade_hit(world: &World, hit: LightHit) -> Color {
    let mut result = black();

    for light in &world.lights {
        // TODO: is_shadowed should probably take a light instead of a world
        let is_shadowed = is_shadowed(world, hit.over_point);

        result += lighting(
            hit.material,
            *light,
            hit.world_point,
            hit.object_point,
            hit.to_eye,
            hit.surface_normal,
            is_shadowed,
        );
    }

    result
}

pub fn color_at(world: &World, ray: Ray) -> Color {
    light_ray(world, ray)
        .map(|h| shade_hit(world, h))
        .unwrap_or(black())
}

fn is_shadowed(world: &World, point: Tuple) -> bool {
    assert!(point.is_point());
    // TODO: support more than one light
    let light = world.lights[0];
    let point_to_light = light.position - point;
    let distance_to_light = point_to_light.magnitude();
    let direction = point_to_light.normalize();

    let intersections = world.ray_intersection(Ray::new(point, direction));
    let hit = Intersection::hit(&intersections);

    // println!(
    //     "Casting shadow ray from {:?} to light at {:?} and hit {:?}",
    //     point, light.position, hit
    // );

    // if the ray hits, check that anything it hit is further than distance_to_light
    hit.map(|i| i.t < distance_to_light).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_lighting() {
        // lighting with the eye between the light and the surface
        let material = PhongMaterial::default();
        let surface_position = point(0., 0., 0.);
        let eye = vec(0., 0., -1.);
        let normal = vec(0., 0., -1.);
        let light = PointLight::new(white(), point(0., 0., -10.));
        let result = lighting(
            material,
            light,
            surface_position,
            surface_position,
            eye,
            normal,
            false,
        );
        // result is ambient + diffuse + specular
        assert_color_eq!(Color::new(1.9, 1.9, 1.9), result, epsilon = 0.0001);
    }

    #[test]
    fn viewing_at_45_deg() {
        let material = PhongMaterial::default();
        let surface_position = point(0., 0., 0.);
        let s22 = 2_f32.sqrt() / 2.;
        let eye = vec(0., s22, s22);
        let normal = vec(0., 0., -1.);
        let light = PointLight::new(white(), point(0., 0., -10.));
        let result = lighting(
            material,
            light,
            surface_position,
            surface_position,
            eye,
            normal,
            false,
        );
        // the surface is still fully lit, but we no longer see the specular highlight
        assert_color_eq!(Color::new(1., 1., 1.), result, epsilon = 0.0001);
    }

    #[test]
    fn light_at_45_deg() {
        let material = PhongMaterial::default();
        let surface_position = point(0., 0., 0.);
        let eye = vec(0., 0., -1.);
        let normal = vec(0., 0., -1.);
        let light = PointLight::new(white(), point(0., 10., -10.));
        let result = lighting(
            material,
            light,
            surface_position,
            surface_position,
            eye,
            normal,
            false,
        );
        // the surface is only partially lit, and we don't see a specular highlight
        assert_color_eq!(Color::new(0.7364, 0.7364, 0.7364), result, epsilon = 0.0001);
    }

    #[test]
    fn light_and_eye_at_45_deg() {
        let material = PhongMaterial::default();
        let surface_position = point(0., 0., 0.);
        let s22 = 2_f32.sqrt() / 2.;
        let eye = vec(0., -s22, -s22);
        let normal = vec(0., 0., -1.);
        let light = PointLight::new(white(), point(0., 10., -10.));
        let result = lighting(
            material,
            light,
            surface_position,
            surface_position,
            eye,
            normal,
            false,
        );
        // the surface is partially lit again
        // since the eye is now in the path of the light's reflection, we get the specular highlight back
        let expected = 0.7364 + 0.9;
        assert_color_eq!(
            Color::new(expected, expected, expected),
            result,
            epsilon = 0.0001
        );
    }

    #[test]
    fn light_behind_surface() {
        // the light should not illuminate anything when behind the surface normal
        let material = PhongMaterial::default();
        let surface_position = point(0., 0., 0.);
        let eye = vec(0., 0., -1.);
        let normal = vec(0., 0., -1.);
        let light = PointLight::new(white(), point(0., 0., 10.));
        let result = lighting(
            material,
            light,
            surface_position,
            surface_position,
            eye,
            normal,
            false,
        );
        // only the ambient light is present
        assert_color_eq!(Color::new(0.1, 0.1, 0.1), result, epsilon = 0.0001);
    }

    #[test]
    fn light_ray_from_outside_sphere() {
        let r = Ray::new(point(0., 0., -5.), vec(0., 0., 1.));
        let shape = &Sphere::unit();
        let hit = Intersection::ray_sphere(shape, 4.);
        let intersection = prepare_computations(&hit, r).unwrap();

        assert_eq!(vec(0., 0., -1.), intersection.surface_normal);
        assert_eq!(false, intersection.inside);
    }

    #[test]
    fn light_ray_from_inside_sphere() {
        let r = Ray::new(point(0., 0., 0.), vec(0., 0., 1.));
        let shape = &Sphere::unit();
        let hit = Intersection::ray_sphere(shape, 1.);
        let intersection = prepare_computations(&hit, r).unwrap();

        // since we're hitting the +ve z side of the sphere, the outside normal is (0,0,+1)
        // but it's inverted since we're hitting the inside
        assert_eq!(vec(0., 0., -1.), intersection.surface_normal);
        assert_eq!(true, intersection.inside);
    }

    #[test]
    fn shade_hit_from_outside_sphere() {
        let w = &World::default();
        let r = Ray::new(point(0., 0., -5.), vec(0., 0., 1.));
        let hit = light_ray(w, r).unwrap();
        let color = shade_hit(w, hit);

        assert_color_eq!(
            Color::new(0.38066, 0.47583, 0.2855),
            color,
            epsilon = 0.00001
        );
    }

    #[test]
    fn shade_hit_from_inside_sphere() {
        let mut w = World::default();
        w.lights[0] = PointLight::new(white(), point(0., 0.25, 0.));
        let r = Ray::new(point(0., 0., 0.), vec(0., 0., 1.));
        let hit = light_ray(&w, r).unwrap();
        let color = shade_hit(&w, hit);

        assert_color_eq!(
            Color::new(0.90498, 0.90498, 0.90498),
            color,
            epsilon = 0.00001
        );
    }

    #[test]
    #[ignore]
    fn shade_hit_with_multiple_lights() {
        unimplemented!();
    }

    #[test]
    fn shade_hit_with_an_intersection_in_shadow() {
        let s1 = Sphere::unit();
        let s2 = Sphere::pos_r(point(0., 0., 10.), 1.);
        let l = PointLight::new(white(), point(0., 0., -10.));
        let w = World::new(vec![Box::new(s1), Box::new(s2)], vec![l]);

        let hit = light_ray(&w, Ray::new(point(0., 0., 5.), vec(0., 0., 1.))).unwrap();

        let c = shade_hit(&w, hit);
        assert_eq!(Color::new(0.1, 0.1, 0.1), c);
    }

    #[test]
    fn lighting_with_surface_in_shadow() {
        let material = PhongMaterial::default();
        let surface_position = point(0., 0., 0.);
        let eye = vec(0., 0., -1.);
        let normal = vec(0., 0., -1.);
        let light = PointLight::new(white(), point(0., 0., -10.));
        let is_shadow = true;
        let result = lighting(
            material,
            light,
            surface_position,
            surface_position,
            eye,
            normal,
            is_shadow,
        );
        // result is just ambient
        assert_color_eq!(Color::new(0.1, 0.1, 0.1), result, epsilon = 0.0001);
    }

    #[test]
    fn should_offset_the_point_for_shadow_rays() {
        // we can't send shadow rays directly from the ray intersection point,
        // since there's a chance they'll immediately intersect with the object
        // being shaded. Instead we bump up the shadow ray source a tiny amount
        // using the surface normal.
        // here we send a ray upwards which hits the bottom of a sphere at z=0
        let r = Ray::new(point(0., 0., -5.), vec(0., 0., 1.));
        let s = Sphere::pos_r(point(0., 0., 1.), 1.);
        let w = World::new(vec![Box::new(s)], vec![]);
        let hit = light_ray(&w, r).unwrap();

        // see TODO about large epsilon in prepare_computations
        assert!(hit.over_point.z > -0.0002);
        assert!(hit.world_point.z > hit.over_point.z);
    }

    mod is_shadowed {
        use super::super::*;

        #[test]
        fn no_shadow_when_point_and_obstacle_are_orthogonal() {
            // light is in top-left-behind quadrant, and object is on top the y axis
            let w = World::default();
            let p = point(0., 10., 0.);
            assert_eq!(false, is_shadowed(&w, p));
        }

        #[test]
        fn shadow_when_obstacle_between_point_and_light() {
            // light is in top-left-behind quadrant, and object is in bottom-right-forward quad
            // the center sphere is between them
            let w = World::default();
            let p = point(10., -10., 10.);
            assert_eq!(true, is_shadowed(&w, p));
        }

        #[test]
        fn no_shadow_when_obstacle_is_behind_light() {
            // light is in top-left-behind quadrant, and point is further out in the same direction
            let w = World::default();
            let p = point(-20., 20., -20.);
            assert_eq!(false, is_shadowed(&w, p));
        }

        #[test]
        fn no_shadow_when_point_is_between_light_and_obstacle() {
            // light is in top-left-behind quadrant, and point is between it and the origin
            let w = World::default();
            let p = point(-5., 5., -5.);
            assert_eq!(false, is_shadowed(&w, p));
        }
    }
}
