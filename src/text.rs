use fontdue::Font;

// ============================================================
// FONT
// ============================================================

const FONT_DATA: &[u8] = include_bytes!("../assets/fonts/Roboto-VariableFont_wdth,wght.ttf");

pub fn load_font() -> Font {
    Font::from_bytes(FONT_DATA, fontdue::FontSettings::default()).expect("Failed to load font")
}
// ============================================================
// TEXT BITMAP
// ============================================================

pub fn create_text_bitmap(font: &Font, text: &str, font_size: f32) -> (Vec<u8>, u32, u32) {
    let mut glyphs = Vec::new();

    let mut total_width = 0usize;
    let mut max_height = 0usize;

    for character in text.chars() {
        let (metrics, bitmap) = font.rasterize(character, font_size);

        let character_width = if character == ' ' {
            (font_size * 0.30) as usize
        } else {
            metrics.width
        };

        total_width += character_width;
        max_height = max_height.max(metrics.height);

        glyphs.push((character, metrics, bitmap));
    }

    if total_width == 0 {
        total_width = 1;
    }

    if max_height == 0 {
        max_height = 1;
    }

    let mut rgba_data = vec![0u8; total_width * max_height * 4];

    let mut x_offset = 0usize;

    for (character, metrics, bitmap) in glyphs {
        if character == ' ' {
            x_offset += (font_size * 0.30) as usize;
            continue;
        }

        for y in 0..metrics.height {
            for x in 0..metrics.width {
                let source_index = y * metrics.width + x;

                let destination_index = (y * total_width + x_offset + x) * 4;

                let alpha = bitmap[source_index];

                rgba_data[destination_index] = 255;
                rgba_data[destination_index + 1] = 255;
                rgba_data[destination_index + 2] = 255;
                rgba_data[destination_index + 3] = alpha;
            }
        }

        x_offset += metrics.width;
    }

    (rgba_data, total_width as u32, max_height as u32)
}
