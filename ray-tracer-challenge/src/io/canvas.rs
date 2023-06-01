use crate::{Color,BLACK};
use std::cmp;

#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    data: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn fill(&mut self, c: &Color) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.data[y][x] = *c;
            }
        }
    }
}

pub fn canvas(width: usize, height: usize) -> Canvas {
    Canvas {
        width,
        height,
        data: vec![vec![BLACK; width]; height]
    }
}

pub fn write_pixel(c: &mut Canvas, x: i64, y: i64, color: &Color) {
    if
        0 <= x && (x as usize) < c.width &&
        0 <= y && (y as usize) < c.height {
        c.data[y as usize][x as usize] = *color;
    }
}

pub fn pixel_at(c: &Canvas, x: i64, y: i64) -> Color {
    if
        0 <= x && (x as usize) < c.width &&
        0 <= y && (y as usize) < c.height {
        c.data[y as usize][x as usize]
    } else {
        BLACK
    }
}

pub fn canvas_to_ppm(c: &Canvas) -> String {
    let mut s = "P3\n".to_owned();
    s.push_str(&format!("{} {}\n", c.width, c.height).to_string());
    s.push_str("255\n");
    for y in 0..c.height {
        let mut len = 0;
        for x in 0..c.width {
            let color = c.data[y][x];
            for e in [
                color.red,
                color.green,
                color.blue,
            ] {
                let e = e.clamp(0., 1.);
                let fmt = format!("{}", (255. * e).ceil() as i32);
                if len + fmt.len() + 1 > 70 {
                    len = 0;
                    s.push_str("\n");
                }
                if len != 0 {
                    // Handles first write, as well as line break.
                    s.push_str(" ");
                    len += 1;
                }
                s.push_str(&fmt);
                len += fmt.len();
            }
        }
        s.push_str("\n");
    }

    // Terminating newline
    s.push_str("\n");
    s
}

pub fn lines(s: &String, start: usize, end: usize) -> String {
    let x: Vec<&str> = s.lines().collect();
    x[start..end].join("\n")
}

impl cmp::PartialEq<Color> for Canvas {
    fn eq(&self, other: &Color) -> bool {
        for x in 0..self.width {
            for y in 0..self.height {
                if self.data[y][x] != *other {
                    return false
                }
            }
        }
        true
    }
}
