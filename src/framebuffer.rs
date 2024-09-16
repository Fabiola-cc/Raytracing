use std::fmt;
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