#![no_std]
#![no_main]

use panic_halt as _;

use embedded_graphics::{
    egrectangle, egtext,
    fonts::Font6x8,
    pixelcolor::BinaryColor,
    prelude::*,
    style::{PrimitiveStyle, TextStyle},
};

use cortex_m_rt::{entry, exception, ExceptionFrame};
use cortex_m_semihosting::hprintln;
use ssd1306::{prelude::*, Builder, I2CDIBuilder};
use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{
    adc, delay,
    i2c::{BlockingI2c, DutyCycle, Mode},
    prelude::*,
    stm32,
    pac,
};

mod libraries;

#[entry]
fn main() -> ! {
    let text_style = TextStyle::new(Font6x8, BinaryColor::On);
    let thick_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 3);
    let dp = pac::Peripherals::take().unwrap();
    let cp = stm32::CorePeripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let mut pc2 = gpioa.pa2.into_push_pull_output(&mut gpioa.crl);

    let mut adc1 = adc::Adc::adc1(dp.ADC1, &mut rcc.apb2, clocks);

    let mut delay = delay::Delay::new(cp.SYST, clocks);

    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    // let i2c = BlockingI2c::i2c1(
    //     dp.I2C1,
    //     (scl, sda),
    //     &mut afio.mapr,
    //     Mode::Fast {
    //         frequency: 400_000.hz(),
    //         duty_cycle: DutyCycle::Ratio2to1,
    //     },
    //     clocks,
    //     &mut rcc.apb1,
    //     1000,
    //     10,
    //     1000,
    //     1000,
    // );

    // let interface = I2CDIBuilder::new().init(i2c);
    // let mut disp: GraphicsMode<_> = Builder::new().connect(interface).into();

    // disp.init().unwrap();

    // egtext!(text = "ECE5042", top_left = (5, 5), style = text_style)
    //     .draw(&mut disp)
    //     .unwrap();

    // egtext!(
    //     text = "Microcontroller",
    //     top_left = (5, 15),
    //     style = text_style
    // )
    // .draw(&mut disp)
    // .unwrap();

    // egtext!(text = "Project", top_left = (5, 25), style = text_style)
    //     .draw(&mut disp)
    //     .unwrap();

    // // libraries::oled::display_text(disp, "test", 5, 45);

    // egrectangle!(
    //     top_left = (64, 5),
    //     bottom_right = (75, 15),
    //     style = thick_stroke
    // )
    // .draw(&mut disp).unwrap();

    // disp.flush().unwrap();

    hprintln!("Waiting on the sensor...").unwrap();
    delay.delay_ms(3000_u16);

    let mut pa4 = gpioa.pa4.into_open_drain_output(&mut gpioa.crl);
    let mut pa3 = gpioa.pa3.into_analog(&mut gpioa.crl);

    // match dht11::Reading::read(&mut delay, &mut pa4) {
    //     Ok(dht11::Reading {
    //         temperature,
    //         relative_humidity,
    //     }) => hprintln!("{}°, {}% RH", temperature, relative_humidity).unwrap(),
    //     Err(e) => hprintln!("Error {:?}", e).unwrap(),.unwrap()
    // }

    loop {
        let data: u16 = adc1.read(&mut pa3).unwrap();
        if libraries::moisture_sensor::get_reading(data) {
            hprintln!("Low Moisture Turning on Pump, Analog {}", data).unwrap();
            pc2.set_high().unwrap();
        } else {
            hprintln!("High Moisture Turning off Pump, Analog {}", data).unwrap();
            pc2.set_low().unwrap();
        }

        delay.delay_ms(1000u16);
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
