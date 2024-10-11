use crate::light::Light;
use crate::color::Color;
use nalgebra_glm::Vec3;

pub struct Scene {
    pub time_of_day: f32, // Valor entre 0.0 (medianoche) y 1.0 (medianoche siguiente)
    pub light: Light,
    pub cycle_duration: f32, 
}

impl Scene {
    pub fn new(cycle_duration: f32) -> Self {
        Self {
            time_of_day: 0.0, // Empezar en medianoche
            light: Light {
                position: Vec3::new(0.0, 10.0, 10.0), // Posición inicial de la luz
                color: Color::new(255, 255, 255),     // Color inicial (luz blanca)
                intensity: 1.0,                       // Intensidad inicial
            },
            cycle_duration,
        }
    }

    pub fn update_time(&mut self, delta_time: f32) {
        self.time_of_day = (self.time_of_day + delta_time) % 1.0; // Ciclo continuo
        self.update_light();
    }

    fn update_light(&mut self) {
        let angle = self.time_of_day * std::f32::consts::PI * 2.0; // Ángulo para el ciclo de rotación
        let light_intensity = (angle.sin() + 1.0) * 0.5; // Intensidad entre 0.0 y 1.0

        // Cambiar la posición de la luz, simulando la rotación del sol
        self.light.position = Vec3::new(
            angle.cos() * 10.0, // 10 unidades de distancia en el eje X
            angle.sin() * 10.0, // 10 unidades de distancia en el eje Y
            10.0, // Mantener una altura constante
        );

        // Cambiar el color de la luz, simulando diferentes tonos durante el día
        if self.time_of_day < 0.25 || self.time_of_day > 0.75 {
            // Noche: Luz más fría y tenue
            self.light.color = Color::new(50, 50, 100); // Color azul oscuro
        } else if self.time_of_day < 0.5 {
            // Mañana: Luz cálida
            self.light.color = Color::new(255, 200, 150); // Luz cálida de amanecer
        } else {
            // Tarde: Luz más intensa
            self.light.color = Color::new(255, 255, 255); // Luz blanca de mediodía
        }

        self.light.intensity = light_intensity; // Ajustar intensidad según el ángulo
    }
}
