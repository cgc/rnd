extern crate piston_window;
extern crate image as im;

use piston_window::*;

struct Rectangle<T> {
    x: T,
    y: T,
    width: T,
    height: T,
}

// http://www.cs.princeton.edu/courses/archive/spr01/cs126/assignments/mandel.html

fn mandel(x: f64, y: f64) -> u8 {
  let mut r = x;
  let mut s = y;
  for i in 0..255 {
    // Compute next values using current.
    let newr = r*r - s*s + x;
    let news = 2.0*r*s + y;
    // Swap in next values.
    r = newr;
    s = news;
    // Halting condition
    if r*r + s*s > 4.0 {
      return i;
    }
  }

  return 255;
}

fn draw_mandel(canvas: &mut im::ImageBuffer<im::Rgba<u8>, Vec<u8>>, window_rect: &Rectangle<u32>, mandel_rect: &Rectangle<f64>) {
  /*
  let white = im::Rgba([255, 255, 255, 255]);
  let black = im::Rgba([0, 0, 0, 255]);
  */

  /*
  Generated in matlab with:
  >> hi = round(hsv(64) * 255);
  >> fprintf(strjoin(arrayfun(@(x) cellstr(sprintf('im::Rgba([%s, 255]),\\n', strjoin(arrayfun(@(x) cellstr(num2str(x)), hi(x, :)), ', '))), 1:size(hi, 1)), ''))
  */
  let colors: [im::Rgba<u8>; 64] = [
  im::Rgba([255, 0, 0, 255]),
  im::Rgba([255, 24, 0, 255]),
  im::Rgba([255, 48, 0, 255]),
  im::Rgba([255, 72, 0, 255]),
  im::Rgba([255, 96, 0, 255]),
  im::Rgba([255, 120, 0, 255]),
  im::Rgba([255, 143, 0, 255]),
  im::Rgba([255, 167, 0, 255]),
  im::Rgba([255, 191, 0, 255]),
  im::Rgba([255, 215, 0, 255]),
  im::Rgba([255, 239, 0, 255]),
  im::Rgba([247, 255, 0, 255]),
  im::Rgba([223, 255, 0, 255]),
  im::Rgba([199, 255, 0, 255]),
  im::Rgba([175, 255, 0, 255]),
  im::Rgba([151, 255, 0, 255]),
  im::Rgba([128, 255, 0, 255]),
  im::Rgba([104, 255, 0, 255]),
  im::Rgba([80, 255, 0, 255]),
  im::Rgba([56, 255, 0, 255]),
  im::Rgba([32, 255, 0, 255]),
  im::Rgba([8, 255, 0, 255]),
  im::Rgba([0, 255, 16, 255]),
  im::Rgba([0, 255, 40, 255]),
  im::Rgba([0, 255, 64, 255]),
  im::Rgba([0, 255, 88, 255]),
  im::Rgba([0, 255, 112, 255]),
  im::Rgba([0, 255, 135, 255]),
  im::Rgba([0, 255, 159, 255]),
  im::Rgba([0, 255, 183, 255]),
  im::Rgba([0, 255, 207, 255]),
  im::Rgba([0, 255, 231, 255]),
  im::Rgba([0, 255, 255, 255]),
  im::Rgba([0, 231, 255, 255]),
  im::Rgba([0, 207, 255, 255]),
  im::Rgba([0, 183, 255, 255]),
  im::Rgba([0, 159, 255, 255]),
  im::Rgba([0, 135, 255, 255]),
  im::Rgba([0, 112, 255, 255]),
  im::Rgba([0, 88, 255, 255]),
  im::Rgba([0, 64, 255, 255]),
  im::Rgba([0, 40, 255, 255]),
  im::Rgba([0, 16, 255, 255]),
  im::Rgba([8, 0, 255, 255]),
  im::Rgba([32, 0, 255, 255]),
  im::Rgba([56, 0, 255, 255]),
  im::Rgba([80, 0, 255, 255]),
  im::Rgba([104, 0, 255, 255]),
  im::Rgba([128, 0, 255, 255]),
  im::Rgba([151, 0, 255, 255]),
  im::Rgba([175, 0, 255, 255]),
  im::Rgba([199, 0, 255, 255]),
  im::Rgba([223, 0, 255, 255]),
  im::Rgba([247, 0, 255, 255]),
  im::Rgba([255, 0, 239, 255]),
  im::Rgba([255, 0, 215, 255]),
  im::Rgba([255, 0, 191, 255]),
  im::Rgba([255, 0, 167, 255]),
  im::Rgba([255, 0, 143, 255]),
  im::Rgba([255, 0, 120, 255]),
  im::Rgba([255, 0, 96, 255]),
  im::Rgba([255, 0, 72, 255]),
  im::Rgba([255, 0, 48, 255]),
  im::Rgba([255, 0, 24, 255]),
  ];

  for x in 0..window_rect.width {
    let fracx: f64 = (x as f32 / window_rect.width as f32).into();
    let mx: f64 = mandel_rect.x + fracx * mandel_rect.width;
    for y in 0..window_rect.height {
      let fracy: f64 = (y as f32 / window_rect.height as f32).into();
      let my: f64 = mandel_rect.y + fracy * mandel_rect.height;
      //canvas.put_pixel(x, y, if mandel(mx, my) < 255 { black } else { white });
      let itcount = mandel(mx, my);
      /* grayscale!
      let color = (-(itcount as i16) + 255) as u8;
      canvas.put_pixel(x, y, im::Rgba([color, color, color, 255]));
      */
      // using hsv colors
      let color = colors[(itcount / 4) as usize];
      canvas.put_pixel(x, y, color);
    }
  }

}

fn main() {
    let opengl = OpenGL::V3_2;
    let (width, height) = (600, 600);
    let mut window: PistonWindow =
        WindowSettings::new("piston: paint", (width, height))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut canvas = im::ImageBuffer::new(width, height);
    let mut texture: G2dTexture = Texture::from_image(
            &mut window.factory,
            &canvas,
            &TextureSettings::new()
        ).unwrap();

    let mut last_pos: Option<[f64; 2]> = None;

    let window_rect = Rectangle {
        x: 0,
        y: 0,
        width: width,
        height: height,
    };

    let mut mandel_rect = Rectangle::<f64> {
        x: -1.5,
        y: -1.0,
        width: 2.0,
        height: 2.0,
    };

    draw_mandel(&mut canvas, &window_rect, &mandel_rect);

    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            texture.update(&mut window.encoder, &canvas).unwrap();
            window.draw_2d(&e, |c, g| {
                clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });
        }

        if let Some(button) = e.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
              if let Some(last_pos) = last_pos {
                let (x, y) = (last_pos[0], last_pos[1]);

                // Convert current click location to a point in the space we're rendering
                let newcenterx = x / window_rect.width as f64 * mandel_rect.width + mandel_rect.x;
                let newcentery = y / window_rect.height as f64 * mandel_rect.height + mandel_rect.y;

                // Zoom in by reducing window size. Let current x/y be the center.
                // TODO record these to make it easier to zoom out?
                mandel_rect.width /= 3.0;
                mandel_rect.height /= 3.0;
                mandel_rect.x = newcenterx - mandel_rect.width / 2.0;
                mandel_rect.y = newcentery - mandel_rect.height / 2.0;

                // TODO make this happen on background thread?
                draw_mandel(&mut canvas, &window_rect, &mandel_rect);
              }
            }
        };
        if let Some(pos) = e.mouse_cursor_args() {
          // Capture mouse when moving so we can use most recent position to zoom
          last_pos = Some(pos);
        }
    }
}
