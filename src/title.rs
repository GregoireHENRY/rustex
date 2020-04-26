use crate::m_pdf;
use crate::layer;

pub fn create(layer: &mut layer::Layer)
{
    let s = include_str!("../rsc/slide/title").to_string();
    let ss = s.split("\n\n");

    layer.layer.begin_text_section();

    for sss in ss {
        m_pdf::readblock(layer, sss);
    }

    layer.layer.end_text_section();
}
