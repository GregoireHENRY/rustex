use crate::toolbox;
use crate::layer;

use printpdf::*;
use freetype::face;
use std::fs::File;
use image::GenericImageView;
use Vec;

pub const BLUE:  Color = Color::Rgb(Rgb{r: 13.0 / 256.0, g: 71.0 / 256.0, b: 161.0 / 256.0, icc_profile: None});
pub const BLACK: Color = Color::Rgb(Rgb{r:  0.0 / 256.0, g:  0.0 / 256.0, b:   0.0 / 256.0, icc_profile: None});
pub const PT2MM: f64 = 0.352778;

pub fn text(layer: &mut layer::Layer, text: &str)
{
    change_font(&layer.layer, &layer.font, layer.size);
    if layer.align[0] == "mid" { layer.x = calc_lower_left_for_centered_text(&text.to_string(), layer.size, layer.width, &layer.face); }
    else                       { layer.x = layer.width/2.0; }
    offset(&mut layer.layer, layer.x, 0.0);
    layer.layer.write_text(text, &layer.font);
    offset(&mut layer.layer, -layer.x, 1.0+layer.between);
    layer.layer.add_line_break();
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

pub fn offset(layer: &PdfLayerReference, dx: f64, dy: f64)
{
    layer.set_text_cursor(Mm(dx), Mm(-dy));
}

pub fn readblock(layer: &mut layer::Layer, block: std::option::Option<&str>)
{
    let block = block.unwrap();
    let mut lines = block.split("\n").peekable();
    if vec!["\\text", "\\image"].contains(lines.peek().unwrap()) {
        layer.head = lines.next().unwrap().to_string();
    }
    let mut arg0: Vec<&str> = Vec::new();
    let mut cont: Vec<&str> = Vec::new();
    let mut argf: Vec<&str> = Vec::new();
    let mut status: u64 = 0;
    for line in lines { 
        if toolbox::strsz(line) == 0 { continue; }
        if status == 0 && toolbox::charind(line, 0) != '\\' {
            status = 1;
        }
        else if status == 1 && toolbox::charind(line, 0) == '\\' {
            status = 2;
        }
        match status {
            0 => arg0.push(line),
            1 => cont.push(line),
            2 => argf.push(line),
            _ => ()
        }
    }
    match layer.head.as_str() {
        "\\text"  => textblock(layer, arg0, cont, argf),
        "\\image" => imageblock(layer, arg0, cont, argf),
        _ => ()
    }
}

pub fn textblock(layer: &mut layer::Layer, arg0: Vec<&str>, cont: Vec<&str>, argf: Vec<&str>)
{
    for s in arg0 {
        let mut ss = s.split(' ');
        match ss.next().unwrap() {
            "\\fill" => {
                let color = match ss.next().unwrap() {
                    "blue" => BLUE,
                    _      => BLACK
                };
                layer.layer.set_fill_color(color);
            },
            "\\offset" => {
                let sx  = ss.next().unwrap();
                let sy  = ss.next().unwrap();
                let lsx = toolbox::strsz(sx);
                let lsy = toolbox::strsz(sy);
                layer.x   = match toolbox::charind(sx, lsx-1) {
                    'W' => sx[0..lsx-1].parse::<f64>().unwrap() * layer.width,
                    _   => sx.parse().unwrap()
                };
                layer.y = match toolbox::charind(sy, lsy-1) {
                    'H' => sy[0..lsy-1].parse::<f64>().unwrap() * layer.height,
                    _   => sy.parse().unwrap()
                };
                offset(&layer.layer, 0.0, layer.y);
            },
            "\\offsety" => {
                let sy  = ss.next().unwrap();
                let lsy = toolbox::strsz(sy);
                layer.y = match toolbox::charind(sy, lsy-1) {
                    'H' => sy[0..lsy-1].parse::<f64>().unwrap() * layer.height,
                    _   => sy.parse().unwrap()
                };
                offset(&layer.layer, 0.0, layer.y);
            },
            "\\size"    => {
                layer.size = ss.next().unwrap().parse().expect("Font size expect a number");
            },
            "\\align"   => {
                layer.align = toolbox::vecstring(ss.next().unwrap().split("-").collect());
            },
            "\\between" => {
                layer.between = ss.next().unwrap().parse().expect("Between line value expect a number");
            }
            _ => ()
        }
    }
    if layer.align[1] == "mid" { offset(&layer.layer, 0.0, -offset_for_vertical_center(cont.len(), layer.size, &layer.face)*1.08); }
    for s in cont {
        text(layer, s);
    }
    offset(&mut layer.layer, 0.0, 1.0+layer.between);
    for s in argf {
        println!("{}", s);
    }
}

pub fn imageblock(layer: &mut layer::Layer, arg0: Vec<&str>, cont: Vec<&str>, argf: Vec<&str>)
{
    for s in arg0 {
        let mut ss = s.split(' ');
        match ss.next().unwrap() {
            "\\offset"  => {
                let sx  = ss.next().unwrap();
                let sy  = ss.next().unwrap();
                let lsx = toolbox::strsz(sx);
                let lsy = toolbox::strsz(sy);
                layer.x   = match toolbox::charind(sx, lsx-1) {
                    'W' => sx[0..lsx-1].parse::<f64>().unwrap() * layer.width,
                    _   => sx.parse().unwrap()
                };
                layer.y = match toolbox::charind(sy, lsy-1) {
                    'H' => sy[0..lsy-1].parse::<f64>().unwrap() * layer.height,
                    _   => sy.parse().unwrap()
                };
            },
            "\\scale"    => {
                layer.scale = ss.next().unwrap().parse().expect("Image scale expect a number");
            }
            _           => ()
        }
    }
    for s in cont {
        image(layer, s);
    }
    for s in argf {
        println!("{}", s);
    }
}

pub fn image(layer: &mut layer::Layer, name: &str)
{
    let mut imgfile = File::open(format!("{}{}", "rsc/img/", name)).unwrap();
    let img = image::open(format!("{}{}", "rsc/img/", name)).unwrap();
    let dim = img.dimensions();
    let decoder = image::jpeg::JpegDecoder::new(&mut imgfile).unwrap();
    let image = Image::try_from(decoder).unwrap();
    image.add_to_layer(layer.layer.clone(), Some(Mm(layer.x -(dim.0 as f64) * px2mm(300) * 0.5 * layer.scale)), Some(Mm(layer.y)), None, Some(layer.scale), Some(layer.scale), None);
}
