use embedded_graphics::{
    egrectangle, egtext,
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Rectangle, Triangle},
    style::{PrimitiveStyle, TextStyle},
    text_style,
};
use panic_halt as _;
use ssd1306::{prelude::*};

pub fn display_text<T: GraphicsMode<>>(display: T, text: &str, x: i16, y: i16) {
    let text_style = TextStyle::new(Font6x8, BinaryColor::On);
    egtext!(text = text, top_left = (x, y), style = text_style)
        .draw(&mut display)
        .unwrap();
}
