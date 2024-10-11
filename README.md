# Proyecto Raytracing

## Descripción
Este proyecto está enfocado en el trazado de rayos (ray tracing) y la representación de cubos y esferas 3D con soporte para materiales con texturas, luces múltiples y efectos como reflexión, refracción, y transparencia. 

## Características principales
- Renderizado 3D: Soporte para cubos y esferas con texturas y materiales personalizados.
- Luces: Soporte para múltiples fuentes de luz con diferentes intensidades y colores.
- Materiales:
        Reflexión y refracción con control de opacidad.
        Texturas animadas, como el movimiento de agua.
        Materiales emisivos que actúan como fuentes de luz.
- Ciclo día/noche: Implementación de un ciclo dinámico de día y noche que afecta la iluminación de la escena.
- Optimización: Uso de paralelismo para renderizar la escena.


## Instalación
- Para usar o contribuir a este proyecto, necesitas tener Rust instalado. Puedes instalar Rust desde la página oficial: https://www.rust-lang.org/tools/install.

## Requisitos
Rust: Versión estable (o nightly si usas características experimentales).
Cargo: Administrador de dependencias incluido con Rust.

### Pasos de instalación
- Clona el repositorio:
`git clone https://github.com/tu-usuario/nombre-del-proyecto.git`
`cd nombre-del-proyecto`
- Compila y ejecuta el proyecto:
`cargo run`

- Si quieres generar una versión optimizada:
`cargo build --release`
`./target/release/Raytracing` 

## Uso
Una vez compilado, el motor renderizará una escena básica con iluminación y objetos 3D. Puedes modificar la escena cambiando los parámetros en el archivo de configuración o modificando directamente las fuentes de luz, objetos y texturas en el código.

## Parámetros ajustables
Cámara: Ajusta la posición y dirección de la cámara para cambiar la perspectiva de la escena.
Objetos: Puedes añadir más cubos o esferas a la escena.
Luces: Añade o modifica las fuentes de luz en la escena, ajustando sus intensidades y colores.
Texturas: Añade texturas personalizadas a los materiales de los objetos.