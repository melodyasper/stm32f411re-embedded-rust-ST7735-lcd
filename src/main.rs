#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Halt on panic
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt; // panic handler

use cortex_m;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;
use hal::{
    gpio::*,
    prelude::*,
    stm32,
    spi::*,
    delay::Delay
};
use st7735_lcd;
use st7735_lcd::Orientation;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::rectangle::Rectangle;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::style::PrimitiveStyleBuilder;

#[entry]
fn main() -> ! {    
    if let (Some(board_peripherals), Some(processor_peripherals)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // The below two lines just setup the system clock and the peripheral clocks. 
        // This shouldn't matter for the LCD.
        let reset_and_clock_control = board_peripherals.RCC;
        let clocks = reset_and_clock_control.constrain().
            cfgr.use_hse(8.mhz()).sysclk(72.mhz()).pclk1(36.mhz()).freeze();

        // Get the general purpose registers ready to use.
        let gpioa = board_peripherals.GPIOA.split();
        let gpiob = board_peripherals.GPIOB.split();
        let gpioc = board_peripherals.GPIOC.split();


        /* Below we use `into_alternate_af5()`, this call puts the pin
        into alternate function mode 5. This is based on the processor.
        
        Each alternate function provides a way for the processor to manage
        more common operations on-chip. We use AF5 because it changes the
        functions of GPIO A5, A6, and A7 to be `SPI1` pins. */


        // PA5 connects to SCL/SCK on the LCD
        let sck = gpioa.pa5.into_alternate_af5();

        // PA6 does not get connected to the LCD
        let miso = gpioa.pa6.into_alternate_af5();
        
        // PA7 connects to SDA/MOSI on the LCD
        let mosi = gpioa.pa7.into_alternate_af5();

        // PC9 connects to RST/RES on the LCD
        let rst = gpioc.pc9.into_push_pull_output();

        // PB0 connects to RS/DC on the LCD
        let dc = gpiob.pb0.into_push_pull_output();

        /* Notice this board is communicating over SPI_1. If it was some other SPI, 
        the pins would be different depending on the alternate functions. The alternate
        function group could also end up being some number other than 5. */
        let spi = Spi::spi1(board_peripherals.SPI1, (sck, miso, mosi), Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        }, 16000000.hz(), clocks);

        /* Remember the change the width and height to match your LCD screen.
        The RGB parameter specifies whether the LCD screen uses RGB or BGR for
        color. Your LCD might vary so if you find your blues are reds or vice
        versa change this parameter. */         
        let mut disp = st7735_lcd::ST7735::new(spi, dc, rst, false, false, 128, 128);

        // This gives the display a source to use to control pin timings.
        let mut delay = Delay::new(processor_peripherals.SYST, clocks);

        // Initialize the display.
        disp.init(&mut delay).unwrap();
        // Set the orientation of the display
        disp.set_orientation(&Orientation::Landscape).unwrap();

        /* Create a style that specifies a color of RED. This will always use
        Rgb565 regardless if your board uses RGB or BGR. */
        let style = PrimitiveStyleBuilder::new().fill_color(Rgb565::RED).build();

        /* Create a rectangle to fill the background. Make sure the second point
        has a width and height that matches your ST7735. */
        let red_backdrop = Rectangle::new(Point::new(0, 0), Point::new(128, 128)).into_styled(style);
        red_backdrop.draw(&mut disp).unwrap();

        loop {
            continue;
        }
    }

    loop {}
}