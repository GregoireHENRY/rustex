use crate::m_pdf;

use printpdf::*;
use freetype::face;

pub fn create(layer: &PdfLayerReference, font: &IndirectFontRef, width: f64, height: f64, face: &face::Face)
{
    let s = include_str!("../rsc/slide/title").to_string();
    let mut ss = s.split("\n\n");

    layer.begin_text_section();

    layer.set_fill_color(m_pdf::BLUE);
    m_pdf::offsety(layer, -height*0.8);
    m_pdf::supertext(&layer, ss.next().unwrap(), 36, &font, &face, width/2.0, "mid-mid", 0.0);

    m_pdf::offsety(layer, -8.0);
    m_pdf::supertext(&layer, ss.next().unwrap(), 18, &font, &face, width/2.0, "mid-mid", 0.0);

    layer.set_fill_color(m_pdf::BLACK);
    m_pdf::offsety(layer, 22.0);
    m_pdf::supertext(&layer, ss.next().unwrap(), 20, &font, &face, width/2.0, "mid-mid", 3.0);

    m_pdf::offsety(layer, -2.0);
    m_pdf::supertext(&layer, ss.next().unwrap(), 16, &font, &face, width/2.0, "mid-mid", 0.0);

    m_pdf::offsety(layer, 2.0);
    m_pdf::supertext(&layer, ss.next().unwrap(), 16, &font, &face, width/2.0, "mid-mid", 0.0);

    m_pdf::image(&layer, "rsc/img/ipsa.jpg",   width*0.25, height*0.20, 0.30);
    m_pdf::image(&layer, "rsc/img/obspm.jpg",  width*0.38, height*0.20, 0.60);
    m_pdf::image(&layer, "rsc/img/birdy.jpg",  width*0.50, height*0.20, 0.20);
    m_pdf::image(&layer, "rsc/img/cceres.jpg", width*0.59, height*0.20, 1.15);
    m_pdf::image(&layer, "rsc/img/esep.jpg",   width*0.71, height*0.20, 1.00);

    m_pdf::offsety(layer, 40.0);
    m_pdf::supertext(&layer, ss.next().unwrap(), 18, &font, &face, width/2.0, "mid-mid", 0.0);
    
    layer.end_text_section();
}
