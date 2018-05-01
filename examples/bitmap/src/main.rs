extern crate stdweb;
extern crate futures;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{
    document,
    window,
    ImageBitmapRenderingContext,
    ImageData,
};

use stdweb::PromiseFuture;

use stdweb::web::html_element::CanvasElement;
use futures::prelude::*;

// Shamelessly stolen from webplatform's TodoMVC example.
macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

fn main() {
    stdweb::initialize();

    let window = window();
    let canvas: CanvasElement = document().query_selector( "#canvas" ).unwrap().unwrap().try_into().unwrap();
    let context: ImageBitmapRenderingContext = canvas.get_context().unwrap();

    canvas.set_width(canvas.offset_width() as u32);
    canvas.set_height(canvas.offset_height() as u32);

    let image_data = ImageData::new_empty(canvas.width(), canvas.height()).unwrap();

    let future = window.create_image_bitmap(image_data).and_then(|image_bitmap| {
      context.transfer_from_image_bitmap(Some(image_bitmap));
    });

    PromiseFuture::spawn(future);
    stdweb::event_loop();
}
