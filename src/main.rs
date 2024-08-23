mod framebuffer;
mod bmp;
mod ray_intersect;
mod color;

use std::time::Duration;
use minifb::{Key, Window, WindowOptions};
use crate::framebuffer::Framebuffer;
use nalgebra_glm::{Vec3, normalize};
use crate::ray_intersect::{Sphere, Material, RayIntersect, Intersect};
use crate::color::Color;

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Sphere]) -> Color {
    let mut intersect = Intersect:: empty( );
    let mut zbuffer = f32 :: INFINITY; // what is the closest element this ray has hit?
    
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

fn render(framebuffer: &mut Framebuffer, objects: &[Sphere]) {
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
            let pixel_color = cast_ray(&Vec3::new(0.0, 0.0, 0.0), &ray_direction, objects);

            framebuffer.set_current_color(pixel_color);
            framebuffer.point(x as f32, y as f32);
        }
    }
}


fn main() {
    let white = Material {
        diffuse: Color::new(255, 255, 255),
    };
    let brown = Material {
        diffuse: Color::new(112, 65, 39),
    };
    let black = Material {
        diffuse: Color::new(0,0,0),
    };

    let objects = [
        //EYES
        Sphere {
            center: Vec3::new(-0.38, 0.40, -3.0),
            radius: 0.05,
            material: white,
        },
        Sphere {
            center: Vec3::new(0.22, 0.40, -3.0),
            radius: 0.05,
            material: white,
        },
        Sphere {
            center: Vec3::new(-0.30, 0.35, -3.0),
            radius: 0.15,
            material: black,
        },
        Sphere {
            center: Vec3::new(0.30, 0.35, -3.0),
            radius: 0.15,
            material: black,
        },
        //NOSE
        Sphere {
            center: Vec3::new(0.0, 0.05, -3.0),
            radius: 0.17,
            material: black,
        },
        Sphere {
            center: Vec3::new(0.0, -0.35, -3.0),
            radius: 0.60,
            material: white,
        },
        //HEAD
        Sphere {
            center: Vec3::new(0.0, 0.0, -4.0),
            radius: 1.5,
            material: brown,
        },
        //EARS
        Sphere {
            center: Vec3::new(-0.80, 0.80, -3.0),
            radius: 0.22,
            material: white,
        },
        Sphere {
            center: Vec3::new(0.80, 0.80, -3.0),
            radius: 0.22,
            material: white,
        },
        Sphere {
            center: Vec3::new(-0.80, 0.80, -3.0),
            radius: 0.40,
            material: brown,
        },
        Sphere {
            center: Vec3::new(0.80, 0.80, -3.0),
            radius: 0.40,
            material: brown,
        },
        
    ];

    let mut framebuffer = Framebuffer::new(800, 600);
    render(&mut framebuffer, &objects);

    let mut window = Window::new(
        "Bear",
        framebuffer.width,
        framebuffer.height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Convierte el buffer de `Vec<u8>` a `Vec<u32>`
        let mut buffer_u32 = vec![0u32; framebuffer.width * framebuffer.height];
        for (i, chunk) in framebuffer.data.chunks_exact(3).enumerate() {
            let r = chunk[0] as u32;
            let g = chunk[1] as u32;
            let b = chunk[2] as u32;
            buffer_u32[i] = (255 << 24) | (r << 16) | (g << 8) | b;
        }

        // Actualiza la ventana con el buffer
    window.update_with_buffer(&buffer_u32, framebuffer.width, framebuffer.height).unwrap();
    }
}