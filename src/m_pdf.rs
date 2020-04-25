//use crate::toolbox;

use printpdf::*;
use freetype::face;
use std::fs::File;
use image::GenericImageView;
use Vec;

pub const BLUE:  Color = Color::Rgb(Rgb{r: 13.0 / 256.0, g: 71.0 / 256.0, b: 161.0 / 256.0, icc_profile: None});
pub const BLACK: Color = Color::Rgb(Rgb{r:  0.0 / 256.0, g:  0.0 / 256.0, b:   0.0 / 256.0, icc_profile: None});
pub const PT2MM: f64 = 0.352778;

pub fn supertext(layer: &PdfLayerReference, text: &str, size: i64, font: &IndirectFontRef, face: &face::Face, x0: f64, align: &str, dyline: f64)
{
    let mut x;
    let mut dy = 0.0;
    let split: Vec<&str> = text.lines().collect();
    let aligns: Vec<&str> = align.split("-").collect();
    let sz = split.len();
    change_font(layer, font, size);
    if aligns[1] == "mid" { dy = offset_for_vertical_center(sz, size, &face); }
    layer.set_text_cursor(Mm(0.0), Mm(dy));
    for sp in split {
        if aligns[0] == "mid" { x = calc_lower_left_for_centered_text(&sp.to_string(), size, x0*2.0, &face); }
        else                  { x = x0; }
        layer.set_text_cursor(Mm(x), Mm(0.0));
	layer.write_text(sp, &font);
        layer.set_text_cursor(Mm(-x), Mm(0.0));
        layer.set_text_cursor(Mm(0.0), Mm(-1.0-dyline));
        layer.add_line_break();
    }
    layer.set_text_cursor(Mm(0.0), Mm(1.0+dyline));
}

fn vertical_scale(face: &face::Face)
-> i64
{
    let vert_scale = {
        if let Ok(_ch) = face.load_char(0x0020, face::LoadFlag::NO_SCALE) { face.glyph().metrics().vertAdvance }
        else                                                              { 1000 }
    };
    vert_scale
}

fn calc_lower_left_for_centered_text(text: &String, font_scale: i64, parent_width: f64, font_face: &face::Face)
-> f64
{
    let s_w = calc_text_width_pt(text, font_scale, font_face);
    (parent_width - s_w*PT2MM*0.94) / 2.0
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

fn offset_for_vertical_center(lines: usize, scale: i64, face: &face::Face)
-> f64
{
    (vertical_scale(&face) / scale) as f64 * PT2MM * (-0.2 + (lines as f64-1.0)/2.95)
}

pub fn change_font(layer: &PdfLayerReference, font: &IndirectFontRef, scale: i64)
{
    layer.set_font(&font, scale);
    layer.set_line_height(scale);
}

/*
pub fn add_line_breaks(layer: &PdfLayerReference, n: u64)
{
    for _ in 1..=n { layer.add_line_break(); }
}
*/

/*
pub fn add_font_breaks(layer: &PdfLayerReference, n: u64, font: &IndirectFontRef, scale: i64)
{
    change_font(layer, &font, scale);
    add_line_breaks(layer, n);
}
*/

fn px2mm(dpi: u64)
-> f64
{
    25.4/dpi as f64
}

pub fn image(layer: &PdfLayerReference, name: &str, x: f64, y: f64, scale: f64)
{
    let mut imgfile = File::open(name).unwrap();
    let img = image::open(name).unwrap();
    let dim = img.dimensions();
    let decoder = image::jpeg::JpegDecoder::new(&mut imgfile).unwrap();
    let image = Image::try_from(decoder).unwrap();
    image.add_to_layer(layer.clone(), Some(Mm(x -(dim.0 as f64) * px2mm(300) * 0.5 * scale)), Some(Mm(y)), None, Some(scale), Some(scale), None);

}

pub fn offsety(layer: &PdfLayerReference, dy: f64)
{
    layer.set_text_cursor(Mm(0.0), Mm(-dy));
}
