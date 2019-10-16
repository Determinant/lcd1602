//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

extern crate panic_halt;
extern crate stm32f1;
#[macro_export] extern crate lcd1602;
use stm32f1::stm32f103::{Interrupt, Peripherals, CorePeripherals, gpioa};
use lcd1602::replace;
use lcd1602::driver;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};


#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let lcd = driver::LCD1602::new(&p);
    lcd.init(driver::LCD16X2_DISPLAY_ON_CURSOR_OFF_BLINK_OFF);
    lcd.set_backlight(true);
    lcd.puts("Hello, world!");
    //hprintln!("Hello world!").unwrap();
    let custom_char: [u8; 8] = [0x0e, 0x1b, 0x11, 0x11, 0x11, 0x11, 0x1f, 0x1f];
    let mut cnt = 0;
    let mut sym = 0;
    lcd.create_custom_char(0, &custom_char);
    loop {
        driver::delay_us(10000);
        if cnt == 40 {
            cnt = 0;
            sym = 1 - sym;
        }
        if cnt & 1 == 0 {
            lcd.putc(if sym == 0 {'.'} else {'+'})
        } else {
            lcd.putc(0 as char)
        }
        cnt += 1;
    }
}
