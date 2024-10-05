use rayon::prelude::*; // Importa Rayon para paralelismo
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{Vec3, normalize};
use std::f32::INFINITY;
use std::f32::consts::PI;
use crate::framebuffer::Framebuffer;
use crate::ray_intersect::{RayIntersect, Intersect};
use crate::color::Color;
use crate::materials::Material;
use crate::camera::Camera;
use crate::light::Light;
use crate::cube::Cube;

mod framebuffer;
mod ray_intersect;
mod color;
mod sphere;
mod materials;
mod camera;
mod light;
mod textures;
mod cube;

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

fn cast_shadow(
    intersect: &Intersect,
    light: &Light,
    objects: &[Cube],
) -> f32 {
    let light_dir = (light.position - intersect.point).normalize();

    // Ajusta el origen del rayo de sombra para evitar la autointersección
    let offset = intersect.normal * 1e-4; // Pequeño valor para evitar estar dentro del cubo
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

fn refract(incident: &Vec3, normal: &Vec3, eta_t: f32) -> Vec3 {
    let cosi = -incident.dot(normal).max(-1.0).min(1.0);
    
    let (n_cosi, eta, n_normal);
    
    if cosi < 0.0 {
        n_cosi = -cosi;
        eta = 1.0 / eta_t;
        n_normal = -normal;
    } else {
        // Ray is leaving the object
        n_cosi = cosi;
        eta = eta_t;
        n_normal = *normal;
    }
    
    let k = 1.0 - eta * eta * (1.0 - n_cosi * n_cosi);
    
    if k < 0.0 {
        // Total internal reflection
        reflect(incident, &n_normal)
    } else {
        eta * incident + (eta * n_cosi - k.sqrt( )) * n_normal
    }
}

fn cast_ray(
    ray_origin: &Vec3, 
    ray_direction: &Vec3, 
    objects: &[Cube], // Cambiado a Cube
    light: &Light,
    depth: u32) -> Color {
    
    if depth > 3 {
        return Color::new(130, 189, 188);
    }

    let mut intersect = Intersect::empty();
    let mut zbuffer = INFINITY; // el objeto más cercano golpeado por el rayo
    
    // Verificamos la intersección del rayo con los cubos
    for object in objects {
        let tmp = object.ray_intersect(ray_origin, ray_direction);
        if tmp.is_intersecting && tmp.distance < zbuffer {
            zbuffer = tmp.distance;
            intersect = tmp;
        }
    }

    if !intersect.is_intersecting {
        return Color::new(130, 189, 188); // color de fondo
    }

    let light_dir = (light.position - intersect.point).normalize();
    let view_dir = (ray_origin - intersect.point).normalize();
    let reflect_dir = reflect(&-light_dir, &intersect.normal);

    let shadow_intensity = cast_shadow(&intersect, light, objects);
    let light_intensity = light.intensity * (1.0 - shadow_intensity);

    let diffuse_intensity = intersect.normal.dot(&light_dir).max(0.0).min(1.0);
    let diffuse_color = intersect.material.get_diffuse_color(intersect.u, intersect.v);
    let diffuse = diffuse_color * intersect.material.albedo[0] * diffuse_intensity * light_intensity;

    let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersect.material.specular);
    let specular = light.color * intersect.material.albedo[1] * specular_intensity * light_intensity;

    let mut reflect_color = Color::black();
    let reflectivity = intersect.material.reflectivity;

    // Corrige el problema de "acné" en cubos utilizando un pequeño desplazamiento `epsilon`
    let epsilon = 1e-4; // Ajuste para evitar el acné
    let reflect_origin = intersect.point + intersect.normal * epsilon; // Origen ajustado para evitar reintersección
    
    if reflectivity > 0.0 {
        let reflect_dir = reflect(&-ray_direction, &intersect.normal).normalize();
        reflect_color = cast_ray(&reflect_origin, &reflect_dir, objects, light, depth + 1);
    }

    let mut refract_color = Color::black();
    let transparency = intersect.material.transparency;

    if transparency > 0.0 {
        let refract_dir = refract(&ray_direction, &intersect.normal, intersect.material.refraction_index);
        let refract_origin = intersect.point + intersect.normal * epsilon; // Origen ajustado
        refract_color = cast_ray(&refract_origin, &refract_dir, objects, light, depth + 1);
    }

    (diffuse + specular) * (1.0 - reflectivity - transparency) + (reflect_color * reflectivity) + (refract_color * transparency)
}

fn render(framebuffer: &mut Framebuffer, objects: &[Cube], camera: &Camera, light: &Light) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = PI / 3.0;
    let perspective_scale = (fov / 2.0).tan();

    let pixels: Vec<(usize, usize, Color)> = (0..framebuffer.height)
        .into_par_iter() // Iteramos en paralelo sobre las filas
        .flat_map(|y| {
            (0..framebuffer.width)
                .into_par_iter() // Iteramos en paralelo sobre las columnas
                .map(move |x| {
                    let screen_x = (2.0 * x as f32) / width - 1.0;
                    let screen_y = -(2.0 * y as f32) / height + 1.0;

                    let screen_x = screen_x * aspect_ratio * perspective_scale;
                    let screen_y = screen_y * perspective_scale;

                    let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));
                    let rotated_direction = camera.basis_change(&ray_direction);
                    let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, light, 0);

                    (x, y, pixel_color)
                })
                .collect::<Vec<_>>()
        })
        .collect();

    for (x, y, color) in pixels {
        framebuffer.set_current_color(color);
        framebuffer.point(x as f32, y as f32);
    }
}

fn main() {
    let cube_material = Material::new(
        Color::new(255, 0, 0),
        50.0,
        [0.6, 0.3],
        0.6,
        0.0,
        0.0,
    );
    let texture_material = Material::new_with_texture(10.0, [0.6, 0.3], 1.0);

    let objects = [
        Cube {
            min: Vec3::new(-1.0, -1.0, -1.0),
            max: Vec3::new(1.0, 1.0, 1.0),
            material: texture_material,
        },
        Cube {
            min: Vec3::new(-1.0, 2.0, -1.0),
            max: Vec3::new(1.0, 4.0, 1.0),
            material: texture_material,
        },
    ];

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let light = Light::new(
        Vec3::new(0.0, 3.0, 5.0),
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

    let rotation_speed = PI/50.0;
    let zoom_speed = 0.1;
    framebuffer.clear();
    framebuffer.set_background_color(Color::new(25, 20, 2));

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
        // camera zoom controls
        if window.is_key_down(Key::Q) {
            camera.zoom(zoom_speed);
        }
        if window.is_key_down(Key::E) {
            camera.zoom(-zoom_speed);
        }
        if camera.is_changed() {
            // Render the scene
            render(&mut framebuffer, &objects, &camera, &light);
        }

        // Actualiza la ventana con el buffer
        window.update_with_buffer(&framebuffer.to_u32_buffer(), framebuffer.width, framebuffer.height)
        .unwrap();
    }
}