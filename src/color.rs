use std::fmt;
use std::ops::{Add, Mul};

// Definimos la estructura Color
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

// Implementamos métodos para la estructura Color
impl Color {
    // Método para crear una nueva instancia de Color con clamping
    pub fn new(red: i32, green: i32, blue: i32) -> Color {
        Color {
            red: red.clamp(0, 255) as u8,
            green: green.clamp(0, 255) as u8,
            blue: blue.clamp(0, 255) as u8,
        }
    }

    pub fn black() -> Color {
        Color {
            red: 0 as u8,
            green: 0 as u8,
            blue: 0 as u8,
        }
    }

    // Método para crear una instancia de Color a partir de un valor hexadecimal
    pub fn from_hex(hex: u32) -> Color {
        Color {
            red: ((hex >> 16) & 0xFF) as u8,
            green: ((hex >> 8) & 0xFF) as u8,
            blue: (hex & 0xFF) as u8,
        }
    }

    // Método para convertir Color a hexadecimal
    pub fn to_hex(&self) -> u32 {
        ((self.red as u32) << 16) | ((self.green as u32) << 8) | (self.blue as u32)
    }
}

// Implementamos el trait Add para permitir la suma de colores
impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            red: self.red.saturating_add(other.red),
            green: self.green.saturating_add(other.green),
            blue: self.blue.saturating_add(other.blue),
        }
    }
}

// Implementamos el trait Mul para permitir la multiplicación de colores por una constante
impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, scalar: f32) -> Color {
        Color {
            red: (self.red as f32 * scalar).clamp(0.0, 255.0) as u8,
            green: (self.green as f32 * scalar).clamp(0.0, 255.0) as u8,
            blue: (self.blue as f32 * scalar).clamp(0.0, 255.0) as u8,
        }
    }
}

// Implementamos el trait Display para permitir la impresión de la estructura Color
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Color: R={}, G={}, B={}", self.red, self.green, self.blue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let color = Color::new(300, -20, 256);
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 0);
        assert_eq!(color.blue, 255);
    }

    #[test]
    fn test_from_hex() {
        let color = Color::from_hex(0xFF00FF);
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 0);
        assert_eq!(color.blue, 255);
    }

    #[test]
    fn test_to_hex() {
        let color = Color::new(255, 0, 255);
        assert_eq!(color.to_hex(), 0xFF00FF);
    }

    #[test]
    fn test_add() {
        let color1 = Color::new(100, 150, 200);
        let color2 = Color::new(155, 105, 60);
        let result = color1 + color2;
        assert_eq!(result.red, 255);
        assert_eq!(result.green, 255);
        assert_eq!(result.blue, 255);
    }

    #[test]
    fn test_mul() {
        let color = Color::new(100, 150, 200);
        let result = color * 0.5;
        assert_eq!(result.red, 50);
        assert_eq!(result.green, 75);
        assert_eq!(result.blue, 100);
    }

    #[test]
    fn test_display() {
        let color = Color::new(255, 0, 255);
        assert_eq!(format!("{}", color), "Color: R=255, G=0, B=255");
    }
}
