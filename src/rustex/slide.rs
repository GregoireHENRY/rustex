use crate::rustex;
use rustex::block;

pub fn create(doc: &mut rustex::Doc, slide: &str)
{
    let s = std::fs::read_to_string(format!("{}{}", "rsc/slide/", slide)).unwrap();
    let ss = s.split("\n\n");
    doc.layer.main.begin_text_section();
    for sss in ss {
        block::read(doc, sss);
    }
    doc.layer.main.end_text_section();
}
