use crate::m_pdf;
use crate::layer;

pub fn create(layer: &mut layer::Layer)
{
    let s = include_str!("../rsc/slide/title").to_string();
    let mut ss = s.split("\n\n");

    layer.layer.begin_text_section();

    m_pdf::readblock(layer, ss.next());
    m_pdf::readblock(layer, ss.next());
    m_pdf::readblock(layer, ss.next());
    m_pdf::readblock(layer, ss.next());
    m_pdf::readblock(layer, ss.next());
    m_pdf::readblock(layer, ss.next());
    m_pdf::readblock(layer, ss.next());
    m_pdf::readblock(layer, ss.next());
    m_pdf::readblock(layer, ss.next());
    m_pdf::readblock(layer, ss.next());
    layer.debug = true;
    m_pdf::readblock(layer, ss.next());

    layer.layer.end_text_section();
}
