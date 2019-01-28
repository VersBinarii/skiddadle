use image::{GenericImageView, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, FontCollection, Scale};

struct Point {
  pub x: u32,
  pub y: u32,
}

#[repr(C)]
pub struct Caption {
  text: String,
  position: Point,
  scale: Scale,
}

impl Caption {
  pub fn new(text: String, x: u32, y: u32, s: u32) -> Self {
    Self {
      text: text,
      position: Point { x: x + 2, y: y + 2 },
      scale: Scale {
        x: (s as f32) * 1.7,
        y: s as f32,
      },
    }
  }
}

pub fn render_text(caption: &Caption, infile: &[u8], outfile: &str) {
  let white: Rgba<u8> = Rgba([255u8, 255u8, 255u8, 255u8]);
  let black: Rgba<u8> = Rgba([0u8, 0u8, 0u8, 255u8]);
  let mut image = match image::load_from_memory(&infile) {
    Ok(i) => i,
    Err(_e) => {
      panic!();
    }
  };
  let font = Vec::from(include_bytes!("../impact.ttf") as &[u8]);
  let font = FontCollection::from_bytes(font)
    .unwrap()
    .into_font()
    .unwrap();

  let img_width = image.width();

  let wrapped_text = wrap_text(&caption.text, img_width, &font, &caption.scale);

  let outline_tickness = 2;

  let mut vert_offset = caption.position.y;
  for text in wrapped_text {
    draw_text_mut(
      &mut image,
      black,
      caption.position.x - outline_tickness,
      vert_offset,
      caption.scale,
      &font,
      &text,
    );
    draw_text_mut(
      &mut image,
      black,
      caption.position.x + outline_tickness,
      vert_offset,
      caption.scale,
      &font,
      &text,
    );
    draw_text_mut(
      &mut image,
      black,
      caption.position.x,
      vert_offset - outline_tickness,
      caption.scale,
      &font,
      &text,
    );
    draw_text_mut(
      &mut image,
      black,
      caption.position.x,
      vert_offset + outline_tickness,
      caption.scale,
      &font,
      &text,
    );
    draw_text_mut(
      &mut image,
      white,
      caption.position.x,
      vert_offset,
      caption.scale,
      &font,
      &text,
    );
    vert_offset += caption.scale.y as u32;
  }

  let _ = image.save(outfile);
}

fn get_text_width(text: &str, font: &Font, scale: &Scale) -> u32 {
  5 + font
    .glyphs_for(text.chars())
    .map(|g| g.scaled(*scale).h_metrics().advance_width)
    .sum::<f32>() as u32
}

fn wrap_text(text: &str, width: u32, font: &Font, scale: &Scale) -> Vec<String> {
  let mut wrapped_text = Vec::new();

  let mut line = String::new();
  let mut line_remaining = width;
  for word in text.split(' ') {
    let word_width = get_text_width(word, font, scale);
    if word_width < line_remaining {
      line.push_str(&format!("{} ", word));
      line_remaining = line_remaining.checked_sub(word_width + 5).unwrap_or(0);
      continue;
    } else {
      wrapped_text.push(line);
      line = String::new();
      line.push_str(&format!("{} ", word));
      line_remaining = width.checked_sub(word_width + 5).unwrap_or(0);
    }
  }
  line.pop(); // remove the trailing space
  wrapped_text.push(line);
  wrapped_text
}
