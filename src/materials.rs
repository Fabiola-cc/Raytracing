use crate::color::Color;
use crate::textures::Texture;
use std::sync::Arc;

// Estructura que contiene las texturas
pub struct TextureManager {
    textures: Vec<Arc<Texture>>, // Contenedor de todas las texturas
}

impl TextureManager {
    pub fn new() -> Self {
        TextureManager {
            textures: Vec::new(),
        }
    }

    // Añadir una textura al contenedor y devolver el índice
    pub fn load_texture(&mut self, path: &str) -> usize {
        let texture = Arc::new(Texture::new(path));
        self.textures.push(texture);
        self.textures.len() - 1 // Devuelve el índice de la textura
    }

    // Obtener una referencia a la textura según el índice
    pub fn get_texture(&self, index: usize) -> &Arc<Texture> {
        &self.textures[index]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    pub albedo: [f32; 2],
    pub reflectivity: f32,
    pub transparency: f32,
    pub refraction_index: f32,
    pub texture_index: Option<usize>, // Índice de la textura en TextureManager
    pub emissive_color: Option<Color>, // Agregar color emisivo
    pub emissive_intensity: f32,
}

impl Material {
    // Constructor para materiales sin textura
    pub fn new(
        diffuse: Color, 
        specular: f32, 
        albedo: [f32; 2],
        reflectivity: f32,
        transparency: f32,
        refraction_index: f32,
    ) -> Self {
        Material {
            diffuse, 
            specular, 
            albedo, 
            reflectivity,
            transparency, 
            refraction_index,
            texture_index: None, 
            emissive_color: None,
            emissive_intensity: 0.0,
        }
    }

    // Constructor para materiales con textura
    pub fn new_with_texture(
        texture_index: usize, // Usar el índice de la textura
        specular: f32,
        albedo: [f32; 2],
        refraction_index: f32,
    ) -> Self {
        Material {
            diffuse: Color::new(0, 0, 0), // Color por defecto, será sobrescrito por la textura
            specular,
            albedo,
            reflectivity: 0.0,
            transparency: 0.0,
            refraction_index,
            texture_index: Some(texture_index), // Asignar el índice de la textura
            emissive_color: None,
            emissive_intensity: 0.0,
        }
    }

    // Constructor con color emisivo
    pub fn new_with_emission(
        diffuse: Color,
        specular: f32,
        albedo: [f32; 2],
        reflectivity: f32,
        transparency: f32,
        refraction_index: f32,
        emissive_color: Option<Color>, 
        emissive_intensity: f32,
    ) -> Self {
        Material {
            diffuse,
            specular,
            albedo,
            reflectivity,
            transparency,
            refraction_index,
            texture_index: None, 
            emissive_color,
            emissive_intensity,
        }
    }

    // Método para comprobar si es emisivo
    pub fn is_emissive(&self) -> bool {
        self.emissive_intensity > 0.0
    }

    // Método para obtener la luz emisiva
    pub fn get_emission(&self) -> Color {
        if let Some(color) = self.emissive_color {
            color * self.emissive_intensity
        } else {
            Color::black() // Si no tiene color emisivo, no emite luz
        }
    }

    // Obtener el color difuso del material según las coordenadas de textura (u, v)
    pub fn get_diffuse_color(&self, u: f32, v: f32, texture_manager: &TextureManager) -> Color {
        if let Some(texture_index) = self.texture_index {
            let texture = texture_manager.get_texture(texture_index);
            let x = (u * (texture.width as f32 - 1.0)) as usize;
            let y = ((1.0 - v) * (texture.height as f32 - 1.0)) as usize;
            texture.get_color(x, y)
        } else {
            self.diffuse
        }
    }

    pub fn black() -> Self {
        Material {
            diffuse: Color::new(0,0,0),
            specular: 0.0,
            albedo: [0.0, 0.0],
            reflectivity: 0.0,
            transparency: 0.0, 
            refraction_index: 0.0,
            texture_index: None, // No hay textura asociada
            emissive_color: None,
            emissive_intensity: 0.0,
        }
    }
}
