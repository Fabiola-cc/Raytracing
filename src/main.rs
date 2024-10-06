use rayon::prelude::*; // Importa Rayon para paralelismo
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{Vec3, normalize};
use std::f32::INFINITY;
use std::f32::consts::PI;
use std::time::Instant;
use crate::framebuffer::Framebuffer;
use crate::ray_intersect::{RayIntersect, Intersect};
use crate::color::Color;
use crate::materials::{Material, TextureManager};
use crate::camera::Camera;
use crate::light::Light;
use crate::cube::Cube;
use crate::scene::Scene;

mod framebuffer;
mod ray_intersect;
mod color;
mod sphere;
mod materials;
mod camera;
mod light;
mod textures;
mod cube;
mod scene;

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
    objects: &[Cube], 
    lights: &[Light], 
    depth: u32,
    texture_manager: &TextureManager
) -> Color {
    
    if depth > 3 {
        return Color::new(130, 189, 188); // Color de fondo si excedemos la profundidad máxima
    }

    let mut intersect = Intersect::empty();
    let mut zbuffer = INFINITY; // El objeto más cercano golpeado por el rayo
    
    // Verificamos la intersección del rayo con los cubos
    for object in objects {
        let tmp = object.ray_intersect(ray_origin, ray_direction);
        if tmp.is_intersecting && tmp.distance < zbuffer {
            zbuffer = tmp.distance;
            intersect = tmp;
        }
    }

    if !intersect.is_intersecting {
        return Color::new(130, 189, 188); // Color de fondo
    }

    let material = intersect.material;

    // Si el material es emisivo, sumamos su emisión
    let mut final_color = if material.is_emissive() {
        material.get_emission() // Obtener la emisión del material
    } else {
        Color::black()
    };

    let view_dir = (ray_origin - intersect.point).normalize();

    // Iteramos sobre todas las luces
    for light in lights {
        let light_dir = (light.position - intersect.point).normalize();
        let reflect_dir = reflect(&-light_dir, &intersect.normal);

        let shadow_intensity = cast_shadow(&intersect, light, objects);
        let light_intensity = light.intensity * (1.0 - shadow_intensity);

        // Componente difusa
        let diffuse_intensity = intersect.normal.dot(&light_dir).max(0.0).min(1.0);
        let diffuse_color = intersect.material.get_diffuse_color(intersect.u, intersect.v, texture_manager);
        let diffuse = diffuse_color * intersect.material.albedo[0] * diffuse_intensity * light_intensity;

        // Componente especular
        let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersect.material.specular);
        let specular = light.color * intersect.material.albedo[1] * specular_intensity * light_intensity;

        // Sumar luz difusa y especular de esta luz al color final
        final_color = final_color + diffuse + specular;
    }

    // Cálculo de reflexión
    let mut reflect_color = Color::black();
    let reflectivity = intersect.material.reflectivity;
    let epsilon = 1e-3; // Pequeño desplazamiento para evitar "acné"

    if reflectivity > 0.0 {
        let reflect_dir = reflect(&-ray_direction, &intersect.normal).normalize();
        let reflect_origin = intersect.point + intersect.normal * epsilon;
        reflect_color = cast_ray(&reflect_origin, &reflect_dir, objects, lights, depth + 1, texture_manager);
    }

    // Cálculo de refracción
    let mut refract_color = Color::black();
    let transparency = intersect.material.transparency;

    if transparency > 0.0 {
        let refract_dir = refract(&ray_direction, &intersect.normal, intersect.material.refraction_index);
        let refract_origin = intersect.point - intersect.normal * epsilon;
        refract_color = cast_ray(&refract_origin, &refract_dir, objects, lights, depth + 1, texture_manager);
    }

    // Combinar resultados: color difuso + especular + reflexión + refracción
    (final_color * (1.0 - reflectivity - transparency)) + (reflect_color * reflectivity) + (refract_color * transparency)
}

fn render(framebuffer: &mut Framebuffer, objects: &[Cube], camera: &Camera, 
    texture_manager: &TextureManager, lights: &[Light], scene: &mut Scene, delta_time: f32) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = PI / 3.0;
    let perspective_scale = (fov / 2.0).tan();
    
    // Combinar la luz de la escena con las luces adicionales
    let mut all_lights = Vec::with_capacity(lights.len() + 1);
    all_lights.push(scene.light);       // Agregar la luz principal
    all_lights.extend_from_slice(lights); // Agregar las luces adicionales
    
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
                    let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, &lights, 0, &texture_manager);

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

fn calculate_delta_time(last_update: Instant) -> f32 {
    let now = Instant::now();
    let duration = now.duration_since(last_update);
    let delta_time = duration.as_secs_f32();
    delta_time
}

fn get_bg_color(scene: &mut Scene) -> Color{
    let mut background_color = Color::black();
    if scene.time_of_day <= 0.25 {
        background_color =  Color::new(135, 206, 235); // de 0 a 6 horas es de noche
    } else if scene.time_of_day > 0.25 && scene.time_of_day < 0.75 {
        background_color =  Color::new(255, 140, 0); // de 0 a 6 horas es de noche
    } else if scene.time_of_day >= 0.75 {
        background_color =  Color::new(25, 25, 112); 
    }
    background_color
}

fn main() {
    let mut texture_manager = TextureManager::new();
    let wood_texture_index = texture_manager.load_texture("assets/wood.png");
    let leaf_texture_index = texture_manager.load_texture("assets/leaves.png");
    let grass_texture_index = texture_manager.load_texture("assets/grass.jpg");
    let light_texture_index = texture_manager.load_texture("assets/light.png");
    let stone_texture_index = texture_manager.load_texture("assets/stone.png");
    let brick_texture_index = texture_manager.load_texture("assets/brick.jpg");

    let soil_material = Material::new_with_texture(
        grass_texture_index,
        50.0,
        [0.6, 0.3],
        0.6,
    );
    let wood_material = Material::new_with_texture(
        wood_texture_index,
        50.0,
        [0.6, 0.3],
        0.6,
    );
    let leaf_material = Material::new_with_texture(
        leaf_texture_index,
        50.0,
        [0.6, 0.3],
        0.6,
    );
    let light_material = Material::new_with_texture(
        light_texture_index,
        50.0,
        [0.6, 0.3],
        0.6,
    );
    let stone_material = Material::new_with_texture(
        stone_texture_index,
        50.0,
        [0.6, 0.3],
        0.6,
    );
    let water_material = Material::new(
        Color::new(115, 136, 255),
        50.0,
        [0.6, 0.3],
        0.6,
        0.7,
        0.3
    );
    let emissive_material = Material::new_with_emission(
        Color::new(255, 150, 50),   // Color emisivo (naranja)
        10.0,                       // Especularidad
        [0.9, 0.1],                 // Albedo
        0.0,                        // Reflectividad
        0.0,                        // Transparencia
        1.0,                        // Índice de refracción
        Some(Color::new(255, 150, 50)), // Color de emisión
        1.0                         // Intensidad de emisión
    );
    let yellow = Material::new(
        Color::new(255, 150, 50),
        10.0,
        [0.9, 0.1],                 // Albedo
        0.0,                        // Reflectividad
        0.0,                        // Transparencia
        1.0, 
    );

    let objects = [
        Cube {
            min: Vec3::new(-1.0, 0.25, 0.5),
            max: Vec3::new(-0.5, 0.75, 1.0),
            material: wood_material,
        },
        Cube {
            min: Vec3::new(-1.0, 0.25, 1.5),
            max: Vec3::new(-0.5, 0.75, 2.0),
            material: emissive_material,
        },
        Cube {
            min: Vec3::new(-0.5, 0.25, 1.5),
            max: Vec3::new(0.0, 0.75, 2.0),
            material: yellow,
        },
    ];

    let mut camera = Camera::new(
        Vec3::new(3.0, 5.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let light = Light::new(
        Vec3::new(-2.0, 3.0, 5.0),
        Color::new(0, 0, 255),
        0.5,
    );

    let light1 = Light {
        position: Vec3::new(10.0, 10.0, 10.0),
        color: Color::new(0, 255, 0),
        intensity: 1.0,
    };

    let light2 = Light {
        position: Vec3::new(-10.0, 5.0, 10.0),
        color: Color::new(255, 0, 0), // Luz roja
        intensity: 0.7,
    };

    let lights = vec![light, light1, light2];

    let mut framebuffer = Framebuffer::new(800, 600);
    
    let mut scene = Scene::new();     // Crear la escena
    let mut last_update = Instant::now(); // Para calcular el delta_time

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
        // day light controls
        if window.is_key_down(Key::T) {
            scene.update_time(0.01);
        }
        if window.is_key_down(Key::L) {
            scene.time_of_day = 0.5;
        }
        if window.is_key_down(Key::N) {
            scene.time_of_day = 0.0;
        }
        if camera.is_changed() {
            // Calcular el delta_time
            let delta_time = calculate_delta_time(last_update);
            last_update = Instant::now();

            render(&mut framebuffer, &objects, &camera, &texture_manager, &lights, &mut scene, delta_time);
        }

        // Actualiza la ventana con el buffer
        window.update_with_buffer(&framebuffer.to_u32_buffer(), framebuffer.width, framebuffer.height)
        .unwrap();
    }
}