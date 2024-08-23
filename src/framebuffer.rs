use std::fmt;
use nalgebra_glm::Vec2;
use crate::color::Color;

#[derive(Debug)]
pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
    current_color: (u8, u8, u8),
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Framebuffer {
        let size = width * height * 3;
        let data = vec![0; size];
        Framebuffer {
            width,
            height,
            data,
            current_color: (0, 0, 0),
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.data.chunks_mut(3).for_each(|pixel| {
            pixel[0] = color.red;
            pixel[1] = color.green;
            pixel[2] = color.blue;
        });
    }
    

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = (color.red, color.green, color.blue);
    }    

    pub fn point(&mut self, x: f32, y: f32) {
        let x = x.round() as isize;
        let y = y.round() as isize;
        if x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
            return;
        }
        let x = x as usize;
        let y = y as usize;
        self.set_pixel(x, y, self.current_color.0, self.current_color.1, self.current_color.2);
    }

    pub fn clear(&mut self) {
        self.data.fill(0);
    }

    fn set_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        if x >= self.width || y >= self.height {
            return;
        }
        //let flipped_y = self.height - 1 - y;  // Invertir el valor de y
        let index = (y * self.width + x) * 3;
        self.data[index] = r;
        self.data[index + 1] = g;
        self.data[index + 2] = b;
    }

    pub fn get_pixel(&self, x: isize, y: isize) -> Option<(u8, u8, u8)> {
        if x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        //let flipped_y = self.height - 1 - y;  // Invertir el valor de y
        let index = (y * self.width + x) * 3;
        Some((self.data[index], self.data[index + 1], self.data[index + 2]))
    }

    fn hex_to_rgb(hex: u32) -> (u8, u8, u8) {
        (
            ((hex >> 16) & 0xFF) as u8,
            ((hex >> 8) & 0xFF) as u8,
            (hex & 0xFF) as u8,
        )
    }

    pub fn to_u32_buffer(&self) -> Vec<u32> {
        let mut buffer = vec![0; self.width * self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                let index = (y * self.width + x) * 3;
                let r = self.data[index] as u32;
                let g = self.data[index + 1] as u32;
                let b = self.data[index + 2] as u32;
                buffer[y * self.width + x] = (r << 16) | (g << 8) | b;
            }
        }
        buffer
    }

    pub fn draw_line(&mut self, start: Vec2, end: Vec2) {
        let x1 = start.x.round() as isize;
        let y1 = start.y.round() as isize;
        let x2 = end.x.round() as isize;
        let y2 = end.y.round() as isize;

        let dx = (x2 - x1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let dy = -(y2 - y1).abs();
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = x1;
        let mut y = y1;

        loop {
            self.point(x as f32, y as f32);

            if x == x2 && y == y2 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }    
}

impl fmt::Display for Framebuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = (y * self.width + x) * 3;
                write!(f, "({}, {}, {}) ", self.data[index], self.data[index + 1], self.data[index + 2])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_framebuffer_creation() {
        let fb = Framebuffer::new(10, 10);
        assert_eq!(fb.width, 10);
        assert_eq!(fb.height, 10);
        assert_eq!(fb.data.len(), 300); // 10 * 10 * 3
    }

    #[test]
    fn test_set_get_pixel() {
        let mut fb = Framebuffer::new(10, 10);
        fb.set_pixel(5, 5, 255, 0, 0);
        let pixel = fb.get_pixel(5, 5);
        assert_eq!(pixel, Some((255, 0, 0)));

        let out_of_bounds_pixel = fb.get_pixel(15, 15);
        assert_eq!(out_of_bounds_pixel, None);
    }

    #[test]
    fn test_color() {
        let mut fb = Framebuffer::new(10, 10);
        fb.set_current_color(0xffee00);
        assert_eq!(fb.current_color, (255, 238, 0));
    }

    #[test]
    fn test_clear() {
        let mut fb = Framebuffer::new(10, 10);
        fb.set_background_color(0xFF0000);
        fb.clear();
        assert_eq!(fb.get_pixel(0, 0), Some((0, 0, 0)));
        assert_eq!(fb.get_pixel(9, 9), Some((0, 0, 0)));
    }

    #[test]
    fn test_set_background_color() {
        let mut fb = Framebuffer::new(10, 10);
        fb.set_background_colo
    }
}