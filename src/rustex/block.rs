use crate::rustex;
use rustex::{pdf, draw};
use crate::toolbox;

use Vec;

pub fn read(doc: &mut rustex::Doc, block: &str)
{
    let mut lines = block.split("\n").peekable();
    if vec!["\\text", "\\image"].contains(lines.peek().unwrap()) {
        doc.layer.head = lines.next().unwrap().to_string();
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
    match doc.layer.head.as_str() {
        "\\text"  => textblock(doc, arg0, cont, argf),
        "\\image" => imageblock(doc, arg0, cont, argf),
        _ => ()
    }
}

pub fn textblock(doc: &mut rustex::Doc, arg0: Vec<&str>, cont: Vec<&str>, argf: Vec<&str>)
{
    for s in arg0 {
        let mut ss = s.split(' ');
        match ss.next().unwrap() {
            "\\fill" => {
                let color = match ss.next().unwrap() {
                    "blue" => pdf::BLUE,
                    _      => pdf::BLACK
                };
                doc.layer.main.set_fill_color(color);
            },
            "\\offset" => {
                let sx  = ss.next().unwrap();
                let sy  = ss.next().unwrap();
                let lsx = toolbox::strsz(sx);
                let lsy = toolbox::strsz(sy);
                doc.layer.x   = match toolbox::charind(sx, lsx-1) {
                    'W' => sx[0..lsx-1].parse::<f64>().unwrap() * doc.width,
                    _   => sx.parse().unwrap()
                };
                doc.layer.y = match toolbox::charind(sy, lsy-1) {
                    'H' => sy[0..lsy-1].parse::<f64>().unwrap() * doc.height,
                    _   => sy.parse().unwrap()
                };
                pdf::offset(&doc.layer.main, 0.0, doc.layer.y);
            },
            "\\offsety" => {
                let sy  = ss.next().unwrap();
                let lsy = toolbox::strsz(sy);
                doc.layer.y = match toolbox::charind(sy, lsy-1) {
                    'H' => sy[0..lsy-1].parse::<f64>().unwrap() * doc.height,
                    _   => sy.parse().unwrap()
                };
                pdf::offset(&doc.layer.main, 0.0, doc.layer.y);
            },
            "\\size"    => {
                doc.layer.size = ss.next().unwrap().parse().expect("Font size expect a number");
            },
            "\\align"   => {
                doc.layer.align = toolbox::vecstring(ss.next().unwrap().split("-").collect());
            },
            "\\between" => {
                doc.layer.between = ss.next().unwrap().parse().expect("Between line value expect a number");
            }
            _ => ()
        }
    }
    if doc.layer.align[1] == "mid" { pdf::offset(&doc.layer.main, 0.0, -pdf::offset_for_vertical_center(cont.len(), doc.layer.size, &doc.face)*1.08); }
    for s in cont {
        draw::text(doc, s);
    }
    pdf::offset(&doc.layer.main, 0.0, 1.0+doc.layer.between);
    for s in argf {
        println!("{}", s);
    }
}

pub fn imageblock(doc: &mut rustex::Doc, arg0: Vec<&str>, cont: Vec<&str>, argf: Vec<&str>)
{
    for s in arg0 {
        let mut ss = s.split(' ');
        match ss.next().unwrap() {
            "\\offset"  => {
                let sx  = ss.next().unwrap();
                let sy  = ss.next().unwrap();
                let lsx = toolbox::strsz(sx);
                let lsy = toolbox::strsz(sy);
                doc.layer.x   = match toolbox::charind(sx, lsx-1) {
                    'W' => sx[0..lsx-1].parse::<f64>().unwrap() * doc.width,
                    _   => sx.parse().unwrap()
                };
                doc.layer.y = match toolbox::charind(sy, lsy-1) {
                    'H' => sy[0..lsy-1].parse::<f64>().unwrap() * doc.height,
                    _   => sy.parse().unwrap()
                };
            },
            "\\scale"    => {
                doc.layer.scale = ss.next().unwrap().parse().expect("Image scale expect a number");
            }
            _           => ()
        }
    }
    for s in cont {
        draw::image(doc, s);
    }
    for s in argf {
        println!("{}", s);
    }
}
