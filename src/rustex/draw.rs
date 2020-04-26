use crate::rustex;
use rustex::pdf;

use printpdf::*;
use std::fs::File;
use image::GenericImageView;

pub fn text(doc: &mut rustex::Doc, text: &str)
{
    pdf::change_font(&doc.layer.main, &doc.font, doc.layer.size);
    if doc.layer.align[0] == "mid" { doc.layer.x = pdf::calc_lower_left_for_centered_text(&text.to_string(), doc); }
    else                           { doc.layer.x = doc.width/2.0; }
    pdf::offset(&doc.layer.main, doc.layer.x, 0.0);
    doc.layer.main.write_text(text, &doc.font);
    pdf::offset(&doc.layer.main, -doc.layer.x, 1.0+doc.layer.between);
    doc.layer.main.add_line_break();
}

pub fn image(doc: &mut rustex::Doc, name: &str)
{
    let mut imgfile = File::open(format!("{}{}", "rsc/img/", name)).unwrap();
    let img = image::open(format!("{}{}", "rsc/img/", name)).unwrap();
    let dim = img.dimensions();
    let decoder = image::jpeg::JpegDecoder::new(&mut imgfile).unwrap();
    let image = Image::try_from(decoder).unwrap();
    image.add_to_layer(doc.layer.main.clone(), Some(Mm(doc.layer.x -(dim.0 as f64) * pdf::px2mm(300) * 0.5 * doc.layer.scale)), Some(Mm(doc.layer.y)), None, Some(doc.layer.scale), Some(doc.layer.scale), None);
}
