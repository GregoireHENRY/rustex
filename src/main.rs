extern crate printpdf;
extern crate freetype;
extern crate image;

mod rustex;
mod toolbox;

use rustex::slide;

fn main()
{
    let mut doc = rustex::new();

    slide::create(&mut doc, "title");

    doc.save();
}
