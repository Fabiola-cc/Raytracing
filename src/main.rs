use rayon::prelude::*; // Importa Rayon para paralelismo
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{Vec3, normalize};
use std::f32::INFINITY;
use std::f32::consts::PI;
use crate::framebuffer::Framebuffer;
use crate::ray_intersect::{RayIntersect, Intersect};
use crate::color::Color;
use crate::sphere::Sphere;
use crate::materials::Material;
use crate::camera::Camera;
use crate::light::Light;

mod framebuffer;
mod ray_intersect;
mod color;
mod sphere;
mod materials;
mod camera;
mod light;

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

fn cast_shadow(
    intersect: &Intersect,
    light: &Light,
    objects: &[Sphere],
) -> f32 {
    let light_dir = (light.position - intersect.point).normalize();

    // Ajusta el origen del rayo de sombra para evitar la autointersección
    let offset = intersect.normal * 1e-4; // Pequeño valor para evitar estar dentro de la esfera
    let shadow_ray_origin = intersect.point + offset;

    let mut shadow_intensity = 0.0;

    for object in objects {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);
        if shadow_intersect.is_intersecting {
            shadow_intensity = 0.7;
            break;
        }
    }

    shadow_intensity
}

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Sphere], light: &Light) -> Color {
    let mut intersect = Intersect::empty( );
    let mut zbuffer = INFINITY; // what is the closest element this ray has hit?
    
    for object in objects {
       let tmp = object.ray_intersect(ray_origin, ray_direction);
        if tmp.is_intersecting &&
            tmp.distance < zbuffer { // is this distance less than the previous?
            zbuffer = intersect.distance; // this is the closest
            intersect = tmp;
            }
    }
    if !intersect.is_intersecting {
        // return default sky box color
        return Color :: new(4, 12, 36);
    }
        
    let light_dir = (light.position - intersect.point).normalize();
    let view_dir = (ray_origin - intersect.point).normalize();
    let reflect_dir = reflect(&-light_dir, &intersect.normal);
    let shadow_intensity = cast_shadow(&intersect, light, objects);
    let light_intensity = light.intensity * (1.0 - shadow_intensity);

    let diffuse_intensity = intersect.normal.dot(&light_dir).max(0.0).min(1.0);
    let diffuse = intersect.material.diffuse * intersect.material.albedo[0] * diffuse_intensity * light_intensity;

    let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersect.material.specular);
    let specular = light.color * intersect.material.albedo[1] * specular_intensity * light_intensity;

    diffuse + specular
}

fn render(framebuffer: &mut Framebuffer, objects: &[Sphere], camera: &Camera, light: &Light) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = PI / 3.0;
    let perspective_scale = (fov / 2.0).tan();

    // Creamos una referencia mutable a framebuffer para usar en el closure de renderizado paralelo
    let pixels: Vec<(usize, usize, Color)> = (0..framebuffer.height)
        .into_par_iter() // Iteramos en paralelo sobre las filas
        .flat_map(|y| {
            (0..framebuffer.width)
                .into_par_iter() // Iteramos en paralelo sobre las columnas
                .map(move |x| {
                    // Map the pixel coordinate to screen space [-1, 1]
                    let screen_x = (2.0 * x as f32) / width - 1.0;
                    let screen_y = -(2.0 * y as f32) / height + 1.0;

                    // Adjust for aspect ratio
                    let screen_x = screen_x * aspect_ratio * perspective_scale;
                    let screen_y = screen_y * perspective_scale;

                    // Calculate the direction of the ray for this pixel
                    let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));

                    let rotated_direction = camera.basis_change(&ray_direction);
                    let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, light);

                    (x, y, pixel_color) // Devolvemos las coordenadas y el color del píxel
                })
                .collect::<Vec<_>>() // Colectamos los píxeles en un vector
        })
        .collect();

    // Asignamos los colores calculados al framebuffer
    for (x, y, color) in pixels {
        framebuffer.set_current_color(color);
        framebuffer.point(x as f32, y as f32);
    }
}

fn main() {
    let rubber = Material {
        diffuse: Color::new(140, 43, 24),
        specular: 1.0,
        albedo: [0.9, 0.1],
    };
    let ivory = Material {
        diffuse: Color::new(147, 151, 153),
        specular: 50.0,
        albedo: [0.6, 0.3],
    };

    let objects = [
        Sphere {
            center: Vec3::new(0.0, 0.0, 2.5),
            radius: 0.1,
            material: ivory,
        },
        Sphere {
            center: Vec3::new(0.0, 0.0, 0.0),
            radius: 1.0,
            material: rubber
        },
    ];

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let mut light = Light::new(
        Vec3::new(0.0, 0.0, 5.0),
        Color::new(255, 255, 255),
        1.0,
    );

    let mut framebuffer = Framebuffer::new(800, 600);

    let mut window = Window::new(
        "Raytracing",
        framebuffer.width,
        framebuffer.height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let rotation_speed = PI/10.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        //CAMERA ORBIT CONTROLS
        if window.is_key_down(Key :: Left) {
            camera.orbit(rotation_speed, 0.0);
        }   
        if window.is_key_down(Key :: Right) {
            camera.orbit(-rotation_speed, 0.0);
        }   
        if window.is_key_down(Key :: Up) {
            camera.orbit(0.0, -rotation_speed);
        }
        if window.is_key_down(Key :: Down) {
            camera.orbit(0.0, rotation_speed);
        }
        
        framebuffer.clear();
        framebuffer.set_background_color(Color::new(25, 20, 2));

        render(&mut framebuffer, &objects, &mut camera, &mut light);

        // Actualiza la ventana con el buffer
        window.update_with_buffer(&framebuffer.to_u32_buffer(), framebuffer.width, framebuffer.height).unwrap();
    }
}