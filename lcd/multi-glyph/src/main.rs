#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use hal::block::ImageDef;
use panic_halt as _;
use rp235x_hal as hal;

use liquid_crystal::prelude::*;
use liquid_crystal::Parallel;

/// Tell the Boot ROM about our application
#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

/// External high-speed crystal on the Raspberry Pi Pico 2 board is 12 MHz.
/// Adjust if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[hal::entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = hal::pac::Peripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut timer = hal::Timer::new_timer0(pac.TIMER0, &mut pac.RESETS, &clocks);

    // Read Select Pin
    let rs = pins.gpio16.into_push_pull_output();

    // Enable Pin
    let en = pins.gpio17.into_push_pull_output();

    // Data Pins
    let d4 = pins.gpio18.into_push_pull_output();
    let d5 = pins.gpio19.into_push_pull_output();
    let d6 = pins.gpio20.into_push_pull_output();
    let d7 = pins.gpio21.into_push_pull_output();

    let mut lcd_interface = Parallel::new(d4, d5, d6, d7, rs, en, lcd_dummy);
    let mut lcd = LiquidCrystal::new(&mut lcd_interface, Bus4Bits, LCD16X2);

    const SYMBOL1: [u8; 8] = [
        0b00110, 0b01000, 0b01110, 0b01000, 0b00100, 0b00011, 0b00100, 0b01000,
    ];

    const SYMBOL2: [u8; 8] = [
        0b00000, 0b00000, 0b00000, 0b10001, 0b10001, 0b11111, 0b00000, 0b00000,
    ];

    const SYMBOL3: [u8; 8] = [
        0b01100, 0b00010, 0b01110, 0b00010, 0b00100, 0b11000, 0b00100, 0b00010,
    ];

    const SYMBOL4: [u8; 8] = [
        0b01000, 0b01000, 0b00100, 0b00011, 0b00001, 0b00010, 0b00101, 0b01000,
    ];

    const SYMBOL5: [u8; 8] = [
        0b00000, 0b00000, 0b00000, 0b11111, 0b01010, 0b10001, 0b00000, 0b00000,
    ];

    const SYMBOL6: [u8; 8] = [
        0b00010, 0b00010, 0b00100, 0b11000, 0b10000, 0b01000, 0b10100, 0b00010,
    ];

    lcd.begin(&mut timer);
    lcd.custom_char(&mut timer, &SYMBOL1, 0);
    lcd.custom_char(&mut timer, &SYMBOL2, 1);
    lcd.custom_char(&mut timer, &SYMBOL3, 2);
    lcd.custom_char(&mut timer, &SYMBOL4, 3);
    lcd.custom_char(&mut timer, &SYMBOL5, 4);
    lcd.custom_char(&mut timer, &SYMBOL6, 5);

    lcd.set_cursor(&mut timer, 0, 4)
        .write(&mut timer, CustomChar(0))
        .write(&mut timer, CustomChar(1))
        .write(&mut timer, CustomChar(2));

    lcd.set_cursor(&mut timer, 1, 4)
        .write(&mut timer, CustomChar(3))
        .write(&mut timer, CustomChar(4))
        .write(&mut timer, CustomChar(5));
    loop {
        timer.delay_ms(500);
    }
}

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[link_section = ".bi_entries"]
#[used]
pub static PICOTOOL_ENTRIES: [hal::binary_info::EntryAddr; 5] = [
    hal::binary_info::rp_cargo_bin_name!(),
    hal::binary_info::rp_cargo_version!(),
    hal::binary_info::rp_program_description!(c"PWM Blinky Example"),
    hal::binary_info::rp_cargo_homepage_url!(),
    hal::binary_info::rp_program_build_attribute!(),
];

// End of file
