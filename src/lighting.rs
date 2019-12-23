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
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl PhongMaterial {
    pub fn new(
        color: Color,
        ambient: f32,
        diffuse: f32,
        specular: f32,
        shininess: f32,
    ) -> PhongMaterial {
        PhongMaterial {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}

impl Default for PhongMaterial {
    fn default() -> PhongMaterial {
        PhongMaterial::new(Color::new(1., 1., 1.), 0.1, 0.9, 0.9, 200.)
    }
}

pub struct LightHit {
    pub point: Tuple,
    pub surface_normal: Tuple,
    pub to_eye: Tuple,
    pub material: PhongMaterial,
    // whether the light ray hit the inside surface of the object.
    // in this case surface_normal is reversed to provide a useful value
    pub inside: bool,
}

pub fn light_ray(ray: Ray, object: Sphere) -> Option<LightHit> {
    let intersects = object.ray_intersection(ray)?;
    let hit = Intersection::hit(&intersects)?;
    let point = ray.position(hit.t);
    let to_eye = -ray.direction;

    let mut surface_normal = object.normal_at(point);
    let mut inside = false;
    if surface_normal.dot(to_eye) < 0. {
        // surface_normal is pointing away from eye, so
        // we're hitting the inside of the surface
        surface_normal = -surface_normal;
        inside = true;
    }

    let material = object.material;
    Some(LightHit {
        point,
        surface_normal,
        material,
        to_eye,
        inside,
    })
}

// TODO: traits for material, light, etc?
pub fn lighting(
    material: PhongMaterial,
    light: PointLight,
    surface_position: Tuple,
    eye: Tuple,
    surface_normal: Tuple,
) -> Color {
    assert!(surface_position.is_point());
    // `eye` is a vector from `surface_position` to the eye position
    assert!(eye.is_vec());
    assert!(surface_normal.is_vec());

    let effective_color = material.color * light.intensity;
    let light_direction = (light.position - surface_position).normalize();
    let ambient = effective_color * material.ambient;
    let cos_light_angle = light_direction.dot(surface_normal);
    let diffuse = match cos_light_angle {
        x if x < 0. => Color::black(), // light is behind surface normal
        x => effective_color * material.diffuse * x, // light is in front, modified by angle
    };
    let specular = match cos_light_angle {
        x if x < 0. => Color::black(),
        _ => {
            let cos_reflection_angle = reflect(-light_direction, surface_normal).dot(eye);
            match cos_reflection_angle {
                x if x < 0. => Color::black(),
                x => {
                    let factor = x.powf(material.shininess);
                    light.intensity * material.specular * factor
                }
            }
        }
    };

    ambient + diffuse + specular
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_lighting() {
        // lighting with the eye between the light and the surface
        let material = PhongMaterial::default();
        let surface_position = Tuple::point(0., 0., 0.);
        let eye = Tuple::vec(0., 0., -1.);
        let normal = Tuple::vec(0., 0., -1.);
        let light = PointLight::new(Color::white(), Tuple::point(0., 0., -10.));
        let result = lighting(material, light, surface_position, eye, normal);
        // result is ambient + diffuse + specular
        assert_color_eq!(Color::new(1.9, 1.9, 1.9), result, epsilon = 0.0001);
    }

    #[test]
    fn viewing_at_45_deg() {
        let material = PhongMaterial::default();
        let surface_position = Tuple::point(0., 0., 0.);
        let s22 = 2_f32.sqrt() / 2.;
        let eye = Tuple::vec(0., s22, s22);
        let normal = Tuple::vec(0., 0., -1.);
        let light = PointLight::new(Color::white(), Tuple::point(0., 0., -10.));
        let result = lighting(material, light, surface_position, eye, normal);
        // the surface is still fully lit, but we no longer see the specular highlight
        assert_color_eq!(Color::new(1., 1., 1.), result, epsilon = 0.0001);
    }

    #[test]
    fn light_at_45_deg() {
        let material = PhongMaterial::default();
        let surface_position = Tuple::point(0., 0., 0.);
        let eye = Tuple::vec(0., 0., -1.);
        let normal = Tuple::vec(0., 0., -1.);
        let light = PointLight::new(Color::white(), Tuple::point(0., 10., -10.));
        let result = lighting(material, light, surface_position, eye, normal);
        // the surface is only partially lit, and we don't see a specular highlight
        assert_color_eq!(Color::new(0.7364, 0.7364, 0.7364), result, epsilon = 0.0001);
    }

    #[test]
    fn light_and_eye_at_45_deg() {
        let material = PhongMaterial::default();
        let surface_position = Tuple::point(0., 0., 0.);
        let s22 = 2_f32.sqrt() / 2.;
        let eye = Tuple::vec(0., -s22, -s22);
        let normal = Tuple::vec(0., 0., -1.);
        let light = PointLight::new(Color::white(), Tuple::point(0., 10., -10.));
        let result = lighting(material, light, surface_position, eye, normal);
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
        let surface_position = Tuple::point(0., 0., 0.);
        let eye = Tuple::vec(0., 0., -1.);
        let normal = Tuple::vec(0., 0., -1.);
        let light = PointLight::new(Color::white(), Tuple::point(0., 0., 10.));
        let result = lighting(material, light, surface_position, eye, normal);
        // only the ambient light is present
        assert_color_eq!(Color::new(0.1, 0.1, 0.1), result, epsilon = 0.0001);
    }

    #[test]
    fn light_ray_from_outside_sphere() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vec(0., 0., 1.));
        let shape = Sphere::unit();
        let intersection = light_ray(r, shape).unwrap();

        assert_eq!(Tuple::vec(0., 0., -1.), intersection.surface_normal);
        assert_eq!(false, intersection.inside);
    }

    #[test]
    fn light_ray_from_inside_sphere() {
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vec(0., 0., 1.));
        let shape = Sphere::unit();
        let intersection = light_ray(r, shape).unwrap();

        // since we're hitting the +ve z side of the sphere, the outside normal is (0,0,+1)
        // but it's inverted since we're hitting the inside
        assert_eq!(Tuple::vec(0., 0., -1.), intersection.surface_normal);
        assert_eq!(true, intersection.inside);
    }
}
