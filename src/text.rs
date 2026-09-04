use fontdue::Font;

// ============================================================
// FONT
// ============================================================

const FONT_DATA: &[u8] = include_bytes!("../assets/fonts/Roboto-VariableFont_wdth,wght.ttf");

pub fn load_font() -> Font {
    Font::from_bytes(FONT_DATA, fontdue::FontSettings::default()).expect("Failed to load font")
}

// ============================================================
// TEXT ALIGNMENT
// ============================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
}

// ============================================================
// TEXT
// ============================================================

pub struct Text {
    // CONTENT
    pub content: String,

    // FONT
    pub font_size: f32,

    // TRANSFORM
    pub position: [f32; 2],
    pub rotation: f32,
    pub scale: [f32; 2],

    // APPEARANCE
    pub color: [f32; 4],
    pub opacity: f32,
    pub visible: bool,

    // LAYOUT
    pub alignment: TextAlignment,
    pub line_spacing: f32,
    pub letter_spacing: f32,
    pub max_width: Option<f32>,

    // GPU UPDATE TRACKING
    revision: u64,
}

impl Text {
    // ========================================================
    // CONSTRUCTOR
    // ========================================================

    pub fn new(content: &str, font_size: f32) -> Self {
        Self {
            content: content.to_string(),

            font_size: font_size.max(1.0),

            position: [0.0, 0.0],

            rotation: 0.0,

            scale: [1.0, 1.0],

            color: [1.0, 1.0, 1.0, 1.0],

            opacity: 1.0,

            visible: true,

            alignment: TextAlignment::Left,

            line_spacing: 1.0,

            letter_spacing: 0.0,

            max_width: None,

            revision: 0,
        }
    }

    // ========================================================
    // CONTENT
    // ========================================================

    pub fn set_content(&mut self, content: &str) {
        self.content = content.to_string();
        self.revision += 1;
    }

    // ========================================================
    // FONT SIZE
    // ========================================================

    pub fn set_font_size(&mut self, font_size: f32) {
        self.font_size = font_size.max(1.0);
        self.revision += 1;
    }

    // ========================================================
    // POSITION
    // ========================================================

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position = [x, y];
    }

    // ========================================================
    // ROTATION
    // ========================================================

    pub fn set_rotation(&mut self, radians: f32) {
        self.rotation = radians;
    }

    // ========================================================
    // SCALE
    // ========================================================

    pub fn set_scale(&mut self, x: f32, y: f32) {
        self.scale = [x.max(0.01), y.max(0.01)];
    }

    // ========================================================
    // COLOR
    // ========================================================

    pub fn set_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.color = [
            r.clamp(0.0, 1.0),
            g.clamp(0.0, 1.0),
            b.clamp(0.0, 1.0),
            a.clamp(0.0, 1.0),
        ];

        self.opacity = self.color[3];
    }

    // ========================================================
    // OPACITY
    // ========================================================

    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity.clamp(0.0, 1.0);
    }

    // ========================================================
    // VISIBILITY
    // ========================================================

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    // ========================================================
    // ALIGNMENT
    // ========================================================

    pub fn set_alignment(&mut self, alignment: TextAlignment) {
        self.alignment = alignment;
        self.revision += 1;
    }

    // ========================================================
    // LINE SPACING
    // ========================================================

    pub fn set_line_spacing(&mut self, spacing: f32) {
        self.line_spacing = spacing.max(0.0);
        self.revision += 1;
    }

    // ========================================================
    // LETTER SPACING
    // ========================================================

    pub fn set_letter_spacing(&mut self, spacing: f32) {
        self.letter_spacing = spacing;
        self.revision += 1;
    }

    // ========================================================
    // MAX WIDTH / WRAPPING
    // ========================================================

    pub fn set_max_width(&mut self, width: Option<f32>) {
        self.max_width = width.map(|value| value.max(1.0));
        self.revision += 1;
    }

    // ========================================================
    // GPU REVISION
    // ========================================================

    pub fn revision(&self) -> u64 {
        self.revision
    }

    // ========================================================
    // BOUNDS
    // ========================================================

    pub fn bounds(&self) -> [f32; 2] {
        let font = load_font();

        let (_, width, height) = create_text_bitmap_with_options(
            &font,
            &self.content,
            self.font_size,
            self.line_spacing,
            self.letter_spacing,
            self.max_width,
            self.alignment,
        );

        [width as f32, height as f32]
    }
}

// ============================================================
// TEXT LAYOUT
// ============================================================

fn character_width(font: &Font, character: char, font_size: f32) -> usize {
    if character == ' ' {
        (font_size * 0.30).max(1.0) as usize
    } else {
        let (metrics, _) = font.rasterize(character, font_size);
        metrics.width
    }
}

fn word_width(font: &Font, word: &str, font_size: f32, letter_spacing: f32) -> usize {
    let mut width = 0.0;

    for (index, character) in word.chars().enumerate() {
        width += character_width(font, character, font_size) as f32;

        if index > 0 {
            width += letter_spacing;
        }
    }

    width.max(0.0) as usize
}

fn wrap_line(
    font: &Font,
    line: &str,
    font_size: f32,
    letter_spacing: f32,
    max_width: Option<f32>,
) -> Vec<String> {
    let Some(max_width) = max_width else {
        return vec![line.to_string()];
    };

    if line.is_empty() {
        return vec![String::new()];
    }

    let max_width = max_width.max(1.0);

    let words: Vec<&str> = line.split_whitespace().collect();

    if words.is_empty() {
        return vec![String::new()];
    }

    let mut lines = Vec::new();
    let mut current = String::new();

    for word in words {
        let candidate = if current.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current, word)
        };

        let candidate_width = word_width(font, &candidate, font_size, letter_spacing);

        if !current.is_empty() && candidate_width as f32 > max_width {
            lines.push(current);
            current = word.to_string();
        } else {
            current = candidate;
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }

    lines
}

// ============================================================
// TEXT BITMAP
// ============================================================

pub fn create_text_bitmap(font: &Font, text: &str, font_size: f32) -> (Vec<u8>, u32, u32) {
    create_text_bitmap_with_options(font, text, font_size, 1.0, 0.0, None, TextAlignment::Left)
}

// ============================================================
// ADVANCED TEXT BITMAP
// ============================================================

pub fn create_text_bitmap_with_options(
    font: &Font,
    text: &str,
    font_size: f32,
    line_spacing: f32,
    letter_spacing: f32,
    max_width: Option<f32>,
    alignment: TextAlignment,
) -> (Vec<u8>, u32, u32) {
    let mut lines = Vec::new();

    for original_line in text.split('\n') {
        let wrapped_lines = wrap_line(font, original_line, font_size, letter_spacing, max_width);

        lines.extend(wrapped_lines);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    // --------------------------------------------------------
    // MEASURE
    // --------------------------------------------------------

    let mut line_widths = Vec::new();
    let mut max_width_pixels = 1usize;
    let mut max_line_height = 1usize;

    for line in &lines {
        let mut width = 0.0;

        for (index, character) in line.chars().enumerate() {
            width += character_width(font, character, font_size) as f32;

            if index > 0 {
                width += letter_spacing;
            }
        }

        let width = width.max(1.0) as usize;

        let mut line_height = 1usize;

        for character in line.chars() {
            let (metrics, _) = font.rasterize(character, font_size);
            line_height = line_height.max(metrics.height);
        }

        max_width_pixels = max_width_pixels.max(width);
        max_line_height = max_line_height.max(line_height);

        line_widths.push(width);
    }

    let line_step = (max_line_height as f32 * line_spacing.max(0.0)).max(max_line_height as f32);

    let total_height = if lines.len() == 1 {
        max_line_height
    } else {
        (max_line_height as f32 + line_step * (lines.len() - 1) as f32).ceil() as usize
    }
    .max(1);

    let mut rgba_data = vec![0u8; max_width_pixels * total_height * 4];

    // --------------------------------------------------------
    // RASTERIZE
    // --------------------------------------------------------

    for (line_index, line) in lines.iter().enumerate() {
        let line_width = line_widths[line_index];

        let alignment_offset = match alignment {
            TextAlignment::Left => 0usize,

            TextAlignment::Center => max_width_pixels.saturating_sub(line_width) / 2,

            TextAlignment::Right => max_width_pixels.saturating_sub(line_width),
        };

        let y_offset = if line_index == 0 {
            0usize
        } else {
            (line_index as f32 * line_step).round() as usize
        };

        let mut x_offset = alignment_offset;

        let character_count = line.chars().count();

        for (character_index, character) in line.chars().enumerate() {
            let (metrics, bitmap) = font.rasterize(character, font_size);

            let char_width = character_width(font, character, font_size);

            if character != ' ' {
                for y in 0..metrics.height {
                    for x in 0..metrics.width {
                        let destination_y = y_offset + y;

                        if destination_y >= total_height {
                            continue;
                        }

                        let destination_x = x_offset + x;

                        if destination_x >= max_width_pixels {
                            continue;
                        }

                        let source_index = y * metrics.width + x;

                        let destination_index =
                            (destination_y * max_width_pixels + destination_x) * 4;

                        let alpha = bitmap[source_index];

                        rgba_data[destination_index] = 255;
                        rgba_data[destination_index + 1] = 255;
                        rgba_data[destination_index + 2] = 255;
                        rgba_data[destination_index + 3] = alpha;
                    }
                }
            }

            x_offset += char_width;

            if character_index + 1 < character_count {
                x_offset = (x_offset as f32 + letter_spacing).max(0.0) as usize;
            }
        }
    }

    (rgba_data, max_width_pixels as u32, total_height as u32)
}
