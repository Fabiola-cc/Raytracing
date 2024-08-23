use crate::materials::Material;
use nalgebra_glm::{Vec3, dot};
use crate::ray_intersect::{RayIntersect, Intersect};

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