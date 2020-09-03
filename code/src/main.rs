#![no_std]
#![no_main]

use embedded_graphics::{
    egtext,
    egrectangle,
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Rectangle, Triangle},
    style::{PrimitiveStyle, TextStyle},
    text_style,
};

use cortex_m_rt::{entry, exception, ExceptionFrame};
use panic_halt as _;
use ssd1306::{prelude::*, Builder, I2CDIBuilder};
use stm32f1xx_hal::{
    i2c::{BlockingI2c, DutyCycle, Mode},
    prelude::*,
    stm32,
};

mod libraries;

#[entry]
fn main() -> ! {
    let text_style = TextStyle::new(Font6x8, BinaryColor::On);
    let thick_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 3);
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000.hz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );

    let interface = I2CDIBuilder::new().init(i2c);
    let mut disp: GraphicsMode<_> = Builder::new().connect(interface).into();

    disp.init().unwrap();

    egtext!(text = "ECE5042", top_left = (5, 5), style = text_style)
        .draw(&mut disp)
        .unwrap();

    egtext!(
        text = "Microcontroller",
        top_left = (5, 15),
        style = text_style
    )
    .draw(&mut disp)
    .unwrap();

    egtext!(text = "Project", top_left = (5, 25), style = text_style)
        .draw(&mut disp)
        .unwrap();

    libraries::oled::display_text(disp, "test", 5, 45);

    egrectangle!(
        top_left = (64, 5),
        bottom_right = (75, 15),
        style = thick_stroke
    )
    .draw(&mut disp).unwrap();

    disp.flush().unwrap();

    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
