#![no_std]
#![no_main]

use panic_halt as _;

use core::fmt::Write;
use cortex_m_rt::{entry, exception, ExceptionFrame};
use cortex_m_semihosting::hprintln;
use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    style::{TextStyle},
};
use embedded_hal::digital::v2::OutputPin;
use heapless::String;
use ssd1306::{prelude::*, Builder, I2CDIBuilder};
use stm32f1xx_hal::{
    adc, delay,
    i2c::{BlockingI2c, DutyCycle, Mode},
    pac,
    prelude::*,
    stm32,
};

mod libraries;

#[entry]
fn main() -> ! {
    let text_style = TextStyle::new(Font6x8, BinaryColor::On);

    // Device Peripherals
    let dp = pac::Peripherals::take().unwrap();

    // Core Peripherals
    let cp = stm32::CorePeripherals::take().unwrap();

    // Get the FLASH memory structure
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    // Configure PA2 as Relay Control
    let mut pa2 = gpioa.pa2.into_push_pull_output(&mut gpioa.crl);

    // Initialise the ADC1 at PA1
    let mut adc1 = adc::Adc::adc1(dp.ADC1, &mut rcc.apb2, clocks);

    let mut delay = delay::Delay::new(cp.SYST, clocks);

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

    hprintln!("Waiting on the sensor...").unwrap();
    delay.delay_ms(3000_u16);

    // let mut pa4 = gpioa.pa4.into_open_drain_output(&mut gpioa.crl);
    let mut pa3 = gpioa.pa3.into_analog(&mut gpioa.crl);

    // let mut reading = dht11::Reading::read(&mut delay, &mut pa4).unwrap();
    // match dht11::Reading::read(&mut delay, &mut pa4) {
    //     Ok(dht11::Reading {
    //         temperature,
    //         relative_humidity,
    //     }) => hprintln!("{}°, {}% RH", temperature, relative_humidity).unwrap(),
    //     Err(e) => hprintln!("Error {:?}", e).unwrap(),.unwrap()
    // }

    let mut moisture_string: String<heapless::consts::U32> = String::new();
    let mut relay_string: String<heapless::consts::U32> = String::new();

    loop {
        moisture_string.clear();
        relay_string.clear();
        let data: u16 = adc1.read(&mut pa3).unwrap();
        if libraries::moisture_sensor::get_reading(data) {
            hprintln!("Low Moisture Turning on Pump, Analog {}", data).unwrap();
            pa2.set_high().unwrap();
            write!(relay_string, "Pump On").unwrap();
        } else {
            hprintln!("High Moisture Turning off Pump, Analog {}", data).unwrap();
            pa2.set_low().unwrap();
            write!(relay_string, "Pump Off").unwrap();
        }

        write!(moisture_string, "Moisture Analog {}", data).unwrap();

        disp.clear();
        Text::new(moisture_string.as_str(), Point::new(5, 5))
            .into_styled(text_style)
            .draw(&mut disp)
            .unwrap();
        Text::new(relay_string.as_str(), Point::new(5, 15))
            .into_styled(text_style)
            .draw(&mut disp)
            .unwrap();
        Text::new("Temperature 38°C", Point::new(5, 25))
            .into_styled(text_style)
            .draw(&mut disp)
            .unwrap();
        Text::new("Humidity 50%", Point::new(5, 35))
            .into_styled(text_style)
            .draw(&mut disp)
            .unwrap();

        disp.flush().unwrap();

        delay.delay_ms(1000u16);
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
