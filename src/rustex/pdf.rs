use crate::rustex;

use printpdf::*;
use freetype::face;

pub const BLUE:  Color = Color::Rgb(Rgb{r: 13.0 / 256.0, g: 71.0 / 256.0, b: 161.0 / 256.0, icc_profile: None});
pub const BLACK: Color = Color::Rgb(Rgb{r:  0.0 / 256.0, g:  0.0 / 256.0, b:   0.0 / 256.0, icc_profile: None});
pub const PT2MM: f64 = 0.352778;

fn vertical_scale(face: &face::Face)
-> i64
{
    let vert_scale = {
        if let Ok(_ch) = face.load_char(0x0020, face::LoadFlag::NO_SCALE) { face.glyph().metrics().vertAdvance }
        else                                                              { 1000 }
    };
    vert_scale
}

pub fn calc_lower_left_for_centered_text(text: &String, doc: &mut rustex::Doc)
-> f64
{
    let s_w = calc_text_width_pt(text, doc.layer.size, &doc.face);
    (doc.width - s_w*PT2MM*doc.magicx) / 2.0
}

fn calc_text_width_pt(text: &String, font_scale: i64, font_face: &face::Face)
-> f64
{
    let vert_scale = vertical_scale(&font_face);
    let sum_width = text.chars().fold(0, |acc, ch| 
        if let Ok(_ch) = font_face.load_char(ch as usize, face::LoadFlag::NO_SCALE) {
            let glyph_w = font_face.glyph().metrics().horiAdvance;
            acc + glyph_w
        } else { acc }
    );
    sum_width as f64 / (vert_scale as f64 / font_scale as f64)
}

pub fn offset_for_vertical_center(lines: usize, scale: i64, face: &face::Face)
-> f64
{
    (vertical_scale(&face) / scale) as f64 * PT2MM * (-0.2 + (lines as f64-1.0)/2.95)
}

pub fn change_font(layer: &PdfLayerReference, font: &IndirectFontRef, scale: i64)
{
    layer.set_font(&font, scale);
    layer.set_line_height(scale);
}

/*pub fn add_line_breaks(layer: &PdfLayerReference, n: u64)
{
    for _ in 1..=n { layer.add_line_break(); }
}
pub fn add_font_breaks(layer: &PdfLayerReference, n: u64, font: &IndirectFontRef, scale: i64)
{
    change_font(layer, &font, scale);
    add_line_breaks(layer, n);
}*/

pub fn px2mm(dpi: u64)
-> f64
{
    25.4/dpi as f64
}

pub fn offset(layer: &PdfLayerReference, dx: f64, dy: f64)
{
    layer.set_text_cursor(Mm(dx), Mm(-dy));
}
