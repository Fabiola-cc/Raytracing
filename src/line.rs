use crate::framebuffer::Framebuffer;
use nalgebra_glm::Vec3;

pub fn draw_line(fb: &mut Framebuffer, start: Vec3, end: Vec3) {
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
        fb.point(x as f32, y as f32);

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