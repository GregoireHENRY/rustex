extern crate printpdf;
extern crate freetype;
extern crate image;

mod toolbox;
mod m_pdf;
mod title;
mod layer;

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

    let (mut doc, page1, layer1) = PdfDocument::new("PDF", Mm(width), Mm(height), "Layer 1");
    doc = doc.with_conformance(PdfConformance::Custom(CustomPdfConformance {
    	requires_icc_profile: false,
    	requires_xmp_metadata: false,
        .. Default::default()
    }));
    let mut _layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_external_font(&mut font_reader).unwrap();
    let mut layer = layer::Layer::new(&mut _layer, font, face, width, height);

    title::create(&mut layer);

    doc.save(&mut BufWriter::new(File::create("main.pdf").unwrap())).unwrap();
}
