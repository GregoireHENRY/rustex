pub mod slide;
mod pdf;
mod block;
mod draw;

use crate::toolbox;

use printpdf::*;
use printpdf::indices::*;
use freetype::{Library, face};
use std::fs::File;
use std::io::BufWriter;
use std::collections::HashMap;
use Vec;

pub struct Layer
{
    pub main: PdfLayerReference,
    pub head: String,
    pub x: f64,
    pub y: f64,
    pub size: i64,
    pub align: Vec<String>,
    pub between: f64,
    pub scale: f64,
    pub debug: bool,
}

impl Layer
{
    pub fn new(layer: PdfLayerReference)
    -> Layer
    { 
        Layer{main: layer,
              head: "text".to_string(),
              x: 0.0,
              y: 0.0,
              size: 12,
              align: toolbox::vecstring(vec!["left", "top"]),
              between: 0.0,
              scale: 1.0,
              debug: false,
        }
    }
}

pub struct Doc
{
    pub name: String,
    pub main: PdfDocumentReference,
    pub page: PdfPageIndex,
    pub layer: Layer,
    pub width: f64,
    pub height: f64,
    pub fontname: String,
    pub font: IndirectFontRef,
    pub face: face::Face,
    pub magicx: f64,
}

impl Doc
{
    pub fn save(self)
    {
        self.main.save(&mut BufWriter::new(File::create(self.name.as_str()).unwrap())).unwrap();
    }
}

pub fn new()
-> Doc
{
    dotenv::dotenv().ok();
    let width  = toolbox::env_f64("WIDTH");
    let height = toolbox::env_f64("HEIGHT");
    let fontname = toolbox::env_str("FONT");
    let lib = Library::init().unwrap();
    let name = "main.pdf";
    let (mut doc, page1, layer1) = PdfDocument::new(name, Mm(width), Mm(height), "Layer 1");
    doc = doc.with_conformance(PdfConformance::Custom(CustomPdfConformance {
    	requires_icc_profile: false,
    	requires_xmp_metadata: false,
        .. Default::default()
    }));
    let _layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_external_font(File::open(format!("{}{}", "rsc/font/", fontname)).expect("Cannot find given font")).unwrap();
    let face = lib.new_face(format!("{}{}", "rsc/font/", fontname), 0).unwrap();
    let layer = Layer::new(_layer);

    let magicx: HashMap<&str, f64> = [("Arial.ttf", 0.94), ("RobotoMono-Regular.ttf", 1.3)].iter().cloned().collect();
    let magicx = magicx[fontname.as_str()];

    Doc{name: name.to_string(),
        main: doc,
        page: page1,
        layer: layer,
        width: width,
        height: height,
        fontname: fontname.to_string(),
        font: font,
        face: face,
        magicx: magicx,
    }
}
