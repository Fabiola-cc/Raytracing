use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use nalgebra_glm::{Vec3, normalize};
use std::f32::INFINITY;

mod framebuffer;
use crate::framebuffer::Framebuffer;
mod ray_intersect;
use crate::ray_intersect::{RayIntersect, Intersect};
mod color;
use crate::color::Color;
mod sphere;
use crate::sphere::Sphere;
mod materials;
use crate::materials::Material;
mod camera;
use crate::camera::Camera;

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Sphere]) -> Color {
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
        
    let diffuse = intersect.material.diffuse;    
    diffuse
}

fn render(framebuffer: &mut Framebuffer, objects: &[Sphere], camera: &Camera) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            // Map the pixel coordinate to screen space [-1, 1]
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            // Adjust for aspect ratio
            let screen_x = screen_x * aspect_ratio;

            // Calculate the direction of the ray for this pixel
            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));

            // Cast the ray and get the pixel color
            let origin = &Vec3::new(0.0, 0.0, 5.0);

            let rotated_direction = camera.basis_change(&ray_direction);

            let pixel_color = cast_ray(origin, &rotated_direction, objects);

            framebuffer.set_current_color(pixel_color);
            framebuffer.point(x as f32, y as f32);
        }
    }
}


fn main() {
    let ivory = Material {
        diffuse: Color::new(255, 255, 255),
    };
    let rubber = Material {
        diffuse: Color::new(140, 43, 24),
    };

    let objects = [
        Sphere {
            center: Vec3::new(-0.2, 0.0, 2.0),
            radius: 0.2,
            material: ivory,
        },
        Sphere {
            center: Vec3::new(0.0, 0.0, 0.0),
            radius: 1.0,
            material: rubber
        },
    ];

    let mut camera = Camera::new(
        Vec3::new(0.0, -3.0, 5.5),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.clear();
        
        render(&mut framebuffer, &objects, &mut camera);

        // Actualiza la ventana con el buffer
        window.update_with_buffer(&framebuffer.to_u32_buffer(), framebuffer.width, framebuffer.height).unwrap();
    }
}