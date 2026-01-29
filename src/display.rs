use eframe::egui::{self, Color32, Pos2, Rect, Ui};

/// Pixel size for digit rendering
const PIXEL_SIZE: f32 = 8.0;
const PIXEL_GAP: f32 = 2.0;

/// Smaller pixel size for overlay mode
const OVERLAY_PIXEL_SIZE: f32 = 5.0;
const OVERLAY_PIXEL_GAP: f32 = 1.0;

/// 5x7 pixel patterns for digits 0-9
/// Each digit is represented as a 7-row array of 5-bit patterns
const DIGIT_PATTERNS: [[u8; 7]; 10] = [
    // 0
    [
        0b01110,
        0b10001,
        0b10011,
        0b10101,
        0b11001,
        0b10001,
        0b01110,
    ],
    // 1
    [
        0b00100,
        0b01100,
        0b00100,
        0b00100,
        0b00100,
        0b00100,
        0b01110,
    ],
    // 2
    [
        0b01110,
        0b10001,
        0b00001,
        0b00110,
        0b01000,
        0b10000,
        0b11111,
    ],
    // 3
    [
        0b01110,
        0b10001,
        0b00001,
        0b00110,
        0b00001,
        0b10001,
        0b01110,
    ],
    // 4
    [
        0b00010,
        0b00110,
        0b01010,
        0b10010,
        0b11111,
        0b00010,
        0b00010,
    ],
    // 5
    [
        0b11111,
        0b10000,
        0b11110,
        0b00001,
        0b00001,
        0b10001,
        0b01110,
    ],
    // 6
    [
        0b00110,
        0b01000,
        0b10000,
        0b11110,
        0b10001,
        0b10001,
        0b01110,
    ],
    // 7
    [
        0b11111,
        0b00001,
        0b00010,
        0b00100,
        0b01000,
        0b01000,
        0b01000,
    ],
    // 8
    [
        0b01110,
        0b10001,
        0b10001,
        0b01110,
        0b10001,
        0b10001,
        0b01110,
    ],
    // 9
    [
        0b01110,
        0b10001,
        0b10001,
        0b01111,
        0b00001,
        0b00010,
        0b01100,
    ],
];

/// Draw a single pixel block with custom size
fn draw_pixel_sized(ui: &mut Ui, pos: Pos2, color: Color32, pixel_size: f32) {
    let rect = Rect::from_min_size(pos, egui::vec2(pixel_size, pixel_size));
    ui.painter().rect_filled(rect, 1.0, color);
}

/// Draw a digit at the specified position with custom size
fn draw_digit_sized(ui: &mut Ui, digit: u8, top_left: Pos2, color: Color32, pixel_size: f32, pixel_gap: f32) {
    if digit > 9 {
        return;
    }

    let pattern = DIGIT_PATTERNS[digit as usize];
    let step = pixel_size + pixel_gap;

    for (row, &bits) in pattern.iter().enumerate() {
        for col in 0..5 {
            // Check if bit is set (from left to right)
            if (bits >> (4 - col)) & 1 == 1 {
                let x = top_left.x + col as f32 * step;
                let y = top_left.y + row as f32 * step;
                draw_pixel_sized(ui, Pos2::new(x, y), color, pixel_size);
            }
        }
    }
}

/// Draw the colon separator with custom size
fn draw_colon_sized(ui: &mut Ui, top_left: Pos2, color: Color32, blink: bool, pixel_size: f32, pixel_gap: f32) {
    if !blink {
        return;
    }
    
    let step = pixel_size + pixel_gap;
    // Upper dot (row 2)
    draw_pixel_sized(ui, Pos2::new(top_left.x, top_left.y + 2.0 * step), color, pixel_size);
    // Lower dot (row 4)
    draw_pixel_sized(ui, Pos2::new(top_left.x, top_left.y + 4.0 * step), color, pixel_size);
}

/// Calculate the width of a digit with custom size
fn digit_width_sized(pixel_size: f32, pixel_gap: f32) -> f32 {
    5.0 * (pixel_size + pixel_gap) - pixel_gap
}

/// Calculate the height of a digit with custom size
fn digit_height_sized(pixel_size: f32, pixel_gap: f32) -> f32 {
    7.0 * (pixel_size + pixel_gap) - pixel_gap
}

/// Draw the full time display (MM:SS)
pub fn draw_time(ui: &mut Ui, minutes: u32, seconds: u32, center: Pos2, color: Color32, show_colon: bool, overlay_mode: bool) {
    let (pixel_size, pixel_gap) = if overlay_mode {
        (OVERLAY_PIXEL_SIZE, OVERLAY_PIXEL_GAP)
    } else {
        (PIXEL_SIZE, PIXEL_GAP)
    };
    
    let step = pixel_size + pixel_gap;
    let d_width = digit_width_sized(pixel_size, pixel_gap);
    let d_height = digit_height_sized(pixel_size, pixel_gap);
    let colon_width = pixel_size + step; // Single pixel + gap
    let spacing = step * 1.5;
    
    // Total width: 4 digits + 1 colon + spacing
    let total_width = 4.0 * d_width + colon_width + 4.0 * spacing;
    let start_x = center.x - total_width / 2.0;
    let start_y = center.y - d_height / 2.0;

    let m1 = (minutes / 10) as u8;
    let m2 = (minutes % 10) as u8;
    let s1 = (seconds / 10) as u8;
    let s2 = (seconds % 10) as u8;

    let mut x = start_x;

    // Minutes tens
    draw_digit_sized(ui, m1, Pos2::new(x, start_y), color, pixel_size, pixel_gap);
    x += d_width + spacing;

    // Minutes ones
    draw_digit_sized(ui, m2, Pos2::new(x, start_y), color, pixel_size, pixel_gap);
    x += d_width + spacing;

    // Colon
    draw_colon_sized(ui, Pos2::new(x, start_y), color, show_colon, pixel_size, pixel_gap);
    x += colon_width + spacing;

    // Seconds tens
    draw_digit_sized(ui, s1, Pos2::new(x, start_y), color, pixel_size, pixel_gap);
    x += d_width + spacing;

    // Seconds ones
    draw_digit_sized(ui, s2, Pos2::new(x, start_y), color, pixel_size, pixel_gap);
}
