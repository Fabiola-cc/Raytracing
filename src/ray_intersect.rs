use nalgebra_glm::{dot, Vec3};
use crate::color::Color;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl RayIntersect for Sphere {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        // Vector desde el origen del rayo hasta el centro de la esfera
        let oc = ray_origin - self.center;
        
        // Coeficientes para la ecuación cuadrática
        let a = dot(ray_direction, ray_direction);
        let b = 2.0 * dot(&oc, ray_direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        
        // Discriminante de la ecuación cuadrática
        let discriminant = b * b - 4.0 * a * c;
        
        // Si el discriminante es negativo, no hay intersección
        if discriminant < 0.0 {
            return Intersect::empty();
        }
        
        // Calcular la raíz más cercana
        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        
        // Tomar la intersección más cercana y positiva
        let t = if t1 > 0.0 { t1 } else { t2 };
        
        if t > 0.0 {
            // Calcular el punto de intersección y la normal
            let intersection_point = ray_origin + ray_direction * t;
            let normal = (intersection_point - self.center).normalize();
            
            Intersect::new(intersection_point, normal, t, self.material)
        } else {
            Intersect::empty()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Intersect {
    pub distance: f32,
    pub is_intersecting: bool,
    pub material: Material,
}

impl Intersect {
    pub fn new(point: Vec3, normal: Vec3, distance: f32, material: Material) -> Self {
        Intersect {
            distance,
            is_intersecting: true,
            material,
        }
    }

    pub fn empty() -> Self {
        Intersect {
            distance: 0.0,
            is_intersecting: false,
            material: Material {
                diffuse: Color::new(0, 0, 0),
            },
        }
    }
}

pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect;
}
