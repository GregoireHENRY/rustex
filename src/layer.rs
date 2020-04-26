use crate::toolbox;

use printpdf::*;
use freetype::face;
use Vec;
//use std::fs::File;
//use image::GenericImageView;

pub struct Layer<'a>
{
    pub layer: &'a mut PdfLayerReference,
    pub font: IndirectFontRef,
    pub face: face::Face,
    pub width: f64,
    pub height: f64,
    pub head: String,
    pub x: f64,
    pub y: f64,
    pub size: i64,
    pub align: Vec<String>,
    pub between: f64,
    pub scale: f64,
    pub debug: bool,
}

impl<'a> Layer<'a>
{
    pub fn new(layer: &'a mut PdfLayerReference,
               font: IndirectFontRef,
               face: face::Face,
               width: f64,
               height: f64)
    -> Layer<'a>
    { 
        Layer{layer: layer,
              font: font,
              face: face,
              width: width,
              height: height,
              head: "text".to_string(),
              x: 0.0,
              y: 0.0,
              size: 12,
              align: toolbox::vecstring(vec!["left", "top"]),
              between: 0.0,
              scale: 1.0,
              debug: false}
    }
}
