/*
    SMART AGRICULTURE SYSTEM
    -------------------------------------------------------------------------------------------
    This Embedded project was done as a part of ECE5042 course at VIT University Chennai
    to explore and learn the usage of Rust Lang on Embedded Platforms

    Authors:
    ROHITH BALAJI [20MES0038]
    AJITH KUMAR R [20MES0052]
    SHRUTI GHADGE [20MES0050]

    Hardware:
    STM32F103 [Blue Pill] based on ARM Cortex M3
    ST-LINKv2 Clone
    SSD1306 OLED Display
    DHT-11
    Capacitive Soli Moisture Sensor

    Software:
    OpenOCD - Open On Chip Debugger
    GDB - GNU General Debugger
    telnet

*/

#![no_std]
#![no_main]

// #[cfg(debug_assertions)] will disable the line below it
// during release build as we need semihosting only in debug
#[cfg(debug_assertions)]
use cortex_m_semihosting::hprintln;

// Dependencies
use core::fmt::Write;
use cortex_m_rt::{entry, exception, ExceptionFrame};
use dht_hal_drv::{dht_read, DhtType};
use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyle,
};
use embedded_hal::digital::v2::OutputPin;
use heapless::String;
use panic_halt as _;
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

    /*  Start Hardware Initialization
        -------------------------------------------------------------------------------------------
        This Part is for manually configuring the hardware pins and clock registers.
        Unlike the official solutions like CubeMX by ST Microelectronics,
        Rust does not Initialize the IO pins and Clock Registers.
        So we have done it manually
    */

    // Device Peripherals
    let dp = pac::Peripherals::take().unwrap();

    // Core Peripherals
    let cp = stm32::CorePeripherals::take().unwrap();

    // Get the FLASH memory structure
    let mut flash = dp.FLASH.constrain();

    // Get the Reset and Clocl Configuration (RCC)
    let mut rcc = dp.RCC.constrain();

    // Set the Clock Configuration Register and assign it a name
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Get the Alternate function I/O
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    // Get the General Purpose I/Os
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    /*  End Hardware Initialization
        -------------------------------------------------------------------------------------------
    */

    /*  Start Program Initialization
        -------------------------------------------------------------------------------------------
        This part Initializes all the library classes and variables and constants required by the
        program including the pin-modes, port speeds, and library interfaces
    */

    // Configure Delay
    let mut delay = delay::Delay::new(cp.SYST, clocks);

    // Configure PA2 as Relay Control
    let mut pa2 = gpioa.pa2.into_push_pull_output(&mut gpioa.crl);

    // Initialize the ADC1 at PA1
    let mut adc1 = adc::Adc::adc1(dp.ADC1, &mut rcc.apb2, clocks);

    //  Initialse PB8 and PB9 as I2C Pins.
    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        // Alternate Functions I/O remap Register
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000.hz(), // I2C Port Speed
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        // Reference to the Advanced Peripheral Bus 1, which is built into the MCU to access the different ports. In this case the I2C.
        &mut rcc.apb1,
        1000, // Connection Timeout in us
        10,   // Max Retries
        1000, // Address Line Timeout
        1000, // Data Line Timeout
    );

    // Init the Driver Interface
    let interface = I2CDIBuilder::new().init(i2c);

    // Init the OLED Display Graphics Builder with the I2C interface
    let mut disp: GraphicsMode<_> = Builder::new().connect(interface).into();

    // Init the Display
    disp.init().unwrap();

    let mut pa4 = gpioa.pa4.into_open_drain_output(&mut gpioa.crl);
    let mut pa3 = gpioa.pa3.into_analog(&mut gpioa.crl);

    let mut moisture_string: String<heapless::consts::U32> = String::new();
    let mut relay_string: String<heapless::consts::U32> = String::new();
    let mut temperature_string: String<heapless::consts::U32> = String::new();
    let mut humidity_string: String<heapless::consts::U32> = String::new();

    #[cfg(debug_assertions)]
    hprintln!("Waiting on the sensor...").unwrap();
    delay.delay_ms(1000_u16);

    /*  End Program Initialization
        -------------------------------------------------------------------------------------------
    */

    /*  Start Program Loop
        -------------------------------------------------------------------------------------------
    */
    loop {
        moisture_string.clear();
        relay_string.clear();
        temperature_string.clear();
        humidity_string.clear();

        let readings = dht_read(DhtType::DHT11, &mut pa4, &mut |d| delay.delay_us(d));

        match readings {
            Ok(res) => {
                temperature_string.clear();
                humidity_string.clear();
                write!(temperature_string, "Temperature {}C", res.temperature()).unwrap();
                write!(humidity_string, "Humidity {}%", res.humidity()).unwrap();
                // hprintln!("DHT readins {}C {}%", res.temperature(), res.humidity());
            }
            Err(err) => {
                write!(humidity_string, "DHT Timing Error").unwrap();
                // hprintln!("DHT ERROR {:?}", err);
            }
        };

        // Read Analog Data from PA3 and convert to digital via ADC1 and store it
        let data: u16 = adc1.read(&mut pa3).unwrap();

        if libraries::moisture_sensor::get_reading(data) {
            #[cfg(debug_assertions)]
            hprintln!("Low Moisture Turning on Pump, Analog {}", data).unwrap();

            // Set Relay to HIGH to turn on Pump
            pa2.set_high().unwrap();

            // Write the Relay status to a string to write to display
            write!(relay_string, "Pump On").unwrap();
        } else {
            #[cfg(debug_assertions)]
            hprintln!("High Moisture Turning off Pump, Analog {}", data).unwrap();

            // Set Relay to HIGH to turn on Pump
            pa2.set_low().unwrap();

            // Write the Relay status to a string to write to display
            write!(relay_string, "Pump Off").unwrap();
        }

        // Write the moisture sensor's analog value to a string to write to display
        write!(moisture_string, "Moisture Analog {}", data).unwrap();

        // Wait for a second.
        delay.delay_ms(2000u16);

        // Wipe Display
        disp.clear();

        // Print all strings to the OLED Display
        Text::new(moisture_string.as_str(), Point::new(5, 5))
            .into_styled(text_style)
            .draw(&mut disp)
            .unwrap();
        Text::new(relay_string.as_str(), Point::new(5, 15))
            .into_styled(text_style)
            .draw(&mut disp)
            .unwrap();
        Text::new(temperature_string.as_str(), Point::new(5, 25))
            .into_styled(text_style)
            .draw(&mut disp)
            .unwrap();
        Text::new(humidity_string.as_str(), Point::new(5, 35))
            .into_styled(text_style)
            .draw(&mut disp)
            .unwrap();

        // Update the Display
        disp.flush().unwrap();
    }

    /*  End Program Loop
        -------------------------------------------------------------------------------------------
    */
}

// Handle Hardware Exceptions
#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
