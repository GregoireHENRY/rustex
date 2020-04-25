extern crate printpdf;
extern crate freetype;
extern crate image;

mod toolbox;
mod m_pdf;
mod title;

use printpdf::*;
use freetype::Library;
use std::fs::File;
use std::io::BufWriter;

fn main()
{
    dotenv::dotenv().ok();
    let width  = toolbox::env_f64("WIDTH");
    let height = toolbox::env_f64("HEIGHT");
    let lib = Library::init().unwrap();
    let face = lib.new_face("rsc/font/Arial.ttf", 0).unwrap();
    let mut font_reader = std::io::Cursor::new(include_bytes!("../rsc/font/Arial.ttf").as_ref());

    let (doc, page1, layer1) = PdfDocument::new("PDF", Mm(width), Mm(height), "Layer 1");
    let layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_external_font(&mut font_reader).unwrap();

    title::create(&layer, &font, width, height, &face);

    doc.save(&mut BufWriter::new(File::create("main.pdf").unwrap())).unwrap();
}
