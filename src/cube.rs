use crate::materials::Material;
use nalgebra_glm::{Vec3, min2, max2};
use crate::ray_intersect::{RayIntersect, Intersect};

pub struct Cube {
    pub min: Vec3, // Una esquina del cubo
    pub max: Vec3, // La esquina opuesta del cubo
    pub material: Material,
}

impl Cube {
    // Obtén las coordenadas UV para el cubo
    fn get_uv(&self, point: &Vec3) -> (f32, f32) {
        // Determina qué cara del cubo se está intersectando
        let mut u = 0.0;
        let mut v = 0.0;

        if point.x == self.min.x { // Cara izquierda
            u = (point.z - self.min.z) / (self.max.z - self.min.z);
            v = (point.y - self.min.y) / (self.max.y - self.min.y);
        } else if point.x == self.max.x { // Cara derecha
            u = (point.z - self.min.z) / (self.max.z - self.min.z);
            v = (point.y - self.min.y) / (self.max.y - self.min.y);
        } else if point.y == self.min.y { // Cara inferior
            u = (point.x - self.min.x) / (self.max.x - self.min.x);
            v = (point.z - self.min.z) / (self.max.z - self.min.z);
        } else if point.y == self.max.y { // Cara superior
            u = (point.x - self.min.x) / (self.max.x - self.min.x);
            v = (point.z - self.min.z) / (self.max.z - self.min.z);
        } else if point.z == self.min.z { // Cara trasera
            u = (point.x - self.min.x) / (self.max.x - self.min.x);
            v = (point.y - self.min.y) / (self.max.y - self.min.y);
        } else if point.z == self.max.z { // Cara delantera
            u = (point.x - self.min.x) / (self.max.x - self.min.x);
            v = (point.y - self.min.y) / (self.max.y - self.min.y);
        }

        (u, v)
    }
}


impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        // Inverso de la dirección del rayo
        let inv_dir = Vec3::new(
            if ray_direction.x != 0.0 { 1.0 / ray_direction.x } else { f32::INFINITY },
            if ray_direction.y != 0.0 { 1.0 / ray_direction.y } else { f32::INFINITY },
            if ray_direction.z != 0.0 { 1.0 / ray_direction.z } else { f32::INFINITY }
        );

        // Calcular t_min y t_max para los ejes x, y, z
        let t_min = (self.min - ray_origin).component_mul(&inv_dir);
        let t_max = (self.max - ray_origin).component_mul(&inv_dir);

        // Obtener los valores mínimos y máximos por eje
        let t1 = min2(&t_min, &t_max);
        let t2 = max2(&t_min, &t_max);

        // Encontrar el t_near y t_far
        let t_near = t1.x.max(t1.y).max(t1.z);
        let t_far = t2.x.min(t2.y).min(t2.z);

        // Si el rayo no intersecta el cubo
        if t_near < 0.0 || t_near > t_far {
            return Intersect::empty();
        }

        // Calcular el punto de intersección y normal
        let intersection_point = ray_origin + ray_direction * t_near;

        // Para encontrar la normal de la cara intersectada
        let normal = if (intersection_point.x - self.min.x).abs() < 1e-4 {
            Vec3::new(-1.0, 0.0, 0.0)
        } else if (intersection_point.x - self.max.x).abs() < 1e-4 {
            Vec3::new(1.0, 0.0, 0.0)
        } else if (intersection_point.y - self.min.y).abs() < 1e-4 {
            Vec3::new(0.0, -1.0, 0.0)
        } else if (intersection_point.y - self.max.y).abs() < 1e-4 {
            Vec3::new(0.0, 1.0, 0.0)
        } else if (intersection_point.z - self.min.z).abs() < 1e-4 {
            Vec3::new(0.0, 0.0, -1.0)
        } else {
            Vec3::new(0.0, 0.0, 1.0)
        };

        let normal = normal.normalize();
        // Calcular las coordenadas UV en la cara donde se dio la intersección
        let (u, v) = self.get_uv(&intersection_point);

        // Retornar la intersección con toda la información
        Intersect::new(intersection_point, normal, t_near, self.material, u, v)
    }
}
