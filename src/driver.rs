#![no_std]

extern crate stm32f1;
use stm32f1::stm32f103::{Interrupt, Peripherals, CorePeripherals, gpioa};

const SYSTICK_CYCLE: u32 = 8_000_000;
const LCD16X2_DELAY_ENABLE_PULSE: u32 = 2;
const LCD16X2_DELAY_INIT: u32 = 5000;
const LCD16X2_DELAY_INIT_REP: u32 = 64;
const LCD16X2_DELAY_INIT_4BIT: u32 = 64;
const LCD16X2_DELAY_POWER_ON: u32 = 16000;
const LCD16X2_BUSY_FLAG: u8 = 0x80;
const LCD16X2_DELAY_BUSY_FLAG: u32 = 4;
const LCD16X2_4BIT_INTERFACE: u8 = 0x00;
const LCD16X2_2LINE_MODE: u8 = 0x08;
const LCD16X2_5X7DOT_FORMAT: u8 = 0x00;

const LCD16X2_CLEAR_DISPLAY: u8 = 0x01;
const LCD16X2_CURSOR_HOME: u8 = 0x02;
const LCD16X2_CHARACTER_ENTRY_MODE: u8 = 0x04;
const LCD16X2_DISPLAY_CURSOR_ON_OFF: u8 = 0x08;
const LCD16X2_DISPLAY_CURSOR_SHIFT: u8 = 0x10;
const LCD16X2_FUNCTION_SET: u8 = 0x20;
const LCD16X2_SET_CGRAM_ADDRESS: u8 = 0x40;
const LCD16X2_SET_DDRAM_ADDRESS: u8 = 0x80;
/* Character entry mode instructions */
const LCD16X2_INCREMENT: u8 = 0x02;
const LCD16X2_DECREMENT: u8 = 0x00;
const LCD16X2_DISPLAY_SHIFT_ON: u8 = 0x01;
const LCD16X2_DISPLAY_SHIFT_OFF: u8 = 0x00;	
/* Display cursor on off instructions */
pub const LCD16X2_DISPLAY_ON: u8 = 0x04;
pub const LCD16X2_DISPLAY_OFF: u8 = 0x00;
pub const LCD16X2_CURSOR_UNDERLINE_ON: u8 = 0x02;
pub const LCD16X2_CURSOR_UNDERLINE_OFF: u8 = 0x00;
pub const LCD16X2_CURSOR_BLINK_ON: u8 = 0x01;
pub const LCD16X2_CURSOR_BLINK_OFF: u8 = 0x00;
/* Display cursor shift instructions */
const LCD16X2_DISPLAY_SHIFT: u8 = 0x08;
const LCD16X2_CURSOR_MOVE: u8 = 0x00;
const LCD16X2_RIGHT_SHIFT: u8 = 0x04;
const LCD16X2_LEFT_SHIFT: u8 = 0x00;

pub const LCD16X2_DISPLAY_ON_CURSOR_OFF_BLINK_OFF: u8 = LCD16X2_DISPLAY_ON | LCD16X2_CURSOR_UNDERLINE_OFF | LCD16X2_CURSOR_BLINK_OFF;

const LCD16X2_LINES: u8 = 2; // visible characters per line of the display
const LCD16X2_DISP_LENGTH: u8 = 16; // DDRAM address of first char of line 1
const LCD16X2_START_LINE_1: u8 = 0x00; // DDRAM address of first char of line 2
const LCD16X2_START_LINE_2: u8 = 0x40;

pub fn delay_us(us: u32) {
    cortex_m::asm::delay(8 * us);
}

macro_rules! make_crh_func {
    ($f: ident, $r: ident) => {
        #[inline(always)]
        fn $f(crh: &mut gpioa::crh::W, v: u8) -> &mut gpioa::crh::W {
            crh.$r().bits(v)
        }
    }
}

macro_rules! make_bsrr_func {
    ($f: ident, $r: ident) => {
        #[inline(always)]
        fn $f(crh: &mut gpioa::bsrr::W) -> &mut gpioa::bsrr::W {
            crh.$r().set_bit()
        }
    }
}

macro_rules! make_brr_func {
    ($f: ident, $r: ident) => {
        #[inline(always)]
        fn $f(crh: &mut gpioa::brr::W) -> &mut gpioa::brr::W {
            crh.$r().set_bit()
        }
    }
}

macro_rules! make_idr_func {
    ($f: ident, $r: ident) => {
        #[inline(always)]
        fn $f(idr: &gpioa::idr::R) -> gpioa::idr::IDR0R {
            idr.$r()
        }
    }
}

make_crh_func!(d7_mode_bits, mode9);
make_crh_func!(d7_cnf_bits, cnf9);
make_bsrr_func!(d7_bsrr_set, bs9);
make_brr_func!(d7_brr_set, br9);
make_idr_func!(d7_idr_get, idr9);

make_crh_func!(d6_mode_bits, mode10);
make_crh_func!(d6_cnf_bits, cnf10);
make_bsrr_func!(d6_bsrr_set, bs10);
make_brr_func!(d6_brr_set, br10);
make_idr_func!(d6_idr_get, idr10);

make_crh_func!(d5_mode_bits, mode11);
make_crh_func!(d5_cnf_bits, cnf11);
make_bsrr_func!(d5_bsrr_set, bs11);
make_brr_func!(d5_brr_set, br11);
make_idr_func!(d5_idr_get, idr11);

make_crh_func!(d4_mode_bits, mode12);
make_crh_func!(d4_cnf_bits, cnf12);
make_bsrr_func!(d4_bsrr_set, bs12);
make_brr_func!(d4_brr_set, br12);
make_idr_func!(d4_idr_get, idr12);


make_crh_func!(rs_mode_bits, mode12);
make_crh_func!(rs_cnf_bits, cnf12);
make_bsrr_func!(rs_bsrr_set, bs12);
make_brr_func!(rs_brr_set, br12);

make_crh_func!(rw_mode_bits, mode13);
make_crh_func!(rw_cnf_bits, cnf13);
make_bsrr_func!(rw_bsrr_set, bs13);
make_brr_func!(rw_brr_set, br13);

make_crh_func!(en_mode_bits, mode14);
make_crh_func!(en_cnf_bits, cnf14);
make_bsrr_func!(en_bsrr_set, bs14);
make_brr_func!(en_brr_set, br14);

make_crh_func!(bl_mode_bits, mode15);
make_crh_func!(bl_cnf_bits, cnf15);
make_bsrr_func!(bl_bsrr_set, bs15);
make_brr_func!(bl_brr_set, br15);

pub struct LCD1602<'a> {
    p: &'a Peripherals
}

impl<'a> LCD1602<'a> {
    pub fn new(p: &'a Peripherals) -> Self { LCD1602{p} }
    fn toggle_en(&self) {
        let gpiob = &self.p.GPIOB;
        // EN pin = HIGH
        gpiob.bsrr.write(|w| en_bsrr_set(w));
        // pulse length in us
        delay_us(LCD16X2_DELAY_ENABLE_PULSE);
        // EN pin = LOW
        gpiob.brr.write(|w| en_brr_set(w));
    }

    fn read(&self, rs: bool) -> u8 {
        let gpioa = &self.p.GPIOA;
        let gpiob = &self.p.GPIOB;
        let mut data: u8 = 0;
        // read mode (RW = 1)
        gpiob.bsrr.write(|w| rw_bsrr_set(w));
        if rs {
            // read data (RS = 1)
            gpiob.bsrr.write(|w| rs_bsrr_set(w));
        } else {
            // read busy flag and DDRAM address (RS = 0)
            gpiob.brr.write(|w| rs_brr_set(w));
        }

        gpioa.crh.write(|w| {
            d7_cnf_bits(d7_mode_bits(w, 0b00), 0b10);
            d6_cnf_bits(d6_mode_bits(w, 0b00), 0b10);
            d5_cnf_bits(d5_mode_bits(w, 0b00), 0b10);
            d4_cnf_bits(d4_mode_bits(w, 0b00), 0b10);
            w
        });

        // EN pin = HIGH
        gpiob.bsrr.write(|w| en_bsrr_set(w));
        delay_us(LCD16X2_DELAY_ENABLE_PULSE);
        /* read high nibble first */
        {
            let r = gpioa.idr.read();
            if d4_idr_get(&r).bit_is_set() { data |= 0x10 };
            if d5_idr_get(&r).bit_is_set() { data |= 0x20 };
            if d6_idr_get(&r).bit_is_set() { data |= 0x40 };
            if d7_idr_get(&r).bit_is_set() { data |= 0x80 };
        }
        // EN pin = LOW
        gpiob.brr.write(|w| en_brr_set(w));
        delay_us(LCD16X2_DELAY_ENABLE_PULSE);
        // EN pin = HIGH
        gpiob.bsrr.write(|w| en_bsrr_set(w));
        delay_us(LCD16X2_DELAY_ENABLE_PULSE);
        /* read low nibble */
        {
            let r = gpioa.idr.read();
            if d4_idr_get(&r).bit_is_set() { data |= 0x01 };
            if d5_idr_get(&r).bit_is_set() { data |= 0x02 };
            if d6_idr_get(&r).bit_is_set() { data |= 0x04 };
            if d7_idr_get(&r).bit_is_set() { data |= 0x08 };
        }
        // EN pin = LOW
        gpiob.brr.write(|w| en_brr_set(w));
        data
    }

    fn wait_busy(&self) -> u8 {
        // wait until busy flag is cleared
        while (self.read(false) & LCD16X2_BUSY_FLAG) != 0 {}
        // delay needed for address counter is updated after busy flag is cleared
        delay_us(LCD16X2_DELAY_BUSY_FLAG);
        // read and return address counter
        self.read(false)
    }

    fn write(&self, data: u8, rs: bool) {
        let gpioa = &self.p.GPIOA;
        let gpiob = &self.p.GPIOB;
        // write mode (RW = 0)
        gpiob.brr.write(|w| rw_brr_set(w));

        if rs {
            // write data (RS = 1)
            gpiob.bsrr.write(|w| rs_bsrr_set(w));
        } else {
            // write instruction (RS = 0)
            gpiob.brr.write(|w| rs_brr_set(w));
        }
        // configure all data pins as output
        gpioa.crh.write(|w| {
            d7_cnf_bits(d7_mode_bits(w, 0b10), 0b00);
            d6_cnf_bits(d6_mode_bits(w, 0b10), 0b00);
            d5_cnf_bits(d5_mode_bits(w, 0b10), 0b00);
            d4_cnf_bits(d4_mode_bits(w, 0b10), 0b00);
            w
        });
        // output high nibble first
        gpioa.brr.write(|w| {
            d7_brr_set(w);
            d6_brr_set(w);
            d5_brr_set(w);
            d4_brr_set(w);
            w
        });
        gpioa.bsrr.write(|w| {
            if data & 0x80 != 0 { d7_bsrr_set(w); }
            if data & 0x40 != 0 { d6_bsrr_set(w); }
            if data & 0x20 != 0 { d5_bsrr_set(w); }
            if data & 0x10 != 0 { d4_bsrr_set(w); }
            w
        });
        self.toggle_en();

        // output low nibble
        gpioa.brr.write(|w| {
            d7_brr_set(w);
            d6_brr_set(w);
            d5_brr_set(w);
            d4_brr_set(w);
            w
        });
        gpioa.bsrr.write(|w| {
            if data & 0x08 != 0 { d7_bsrr_set(w); }
            if data & 0x04 != 0 { d6_bsrr_set(w); }
            if data & 0x02 != 0 { d5_bsrr_set(w); }
            if data & 0x01 != 0 { d4_bsrr_set(w); }
            w
        });
        self.toggle_en();

        // all data pins high (inactive)
        gpioa.bsrr.write(|w| {
            d7_bsrr_set(w);
            d6_bsrr_set(w);
            d5_bsrr_set(w);
            d4_bsrr_set(w);
            w
        });
    }

    fn write_command(&self, cmd: u8) {
        self.wait_busy();
        self.write(cmd, false);
    }

    fn clrscr(&self) {
        self.write_command(LCD16X2_CLEAR_DISPLAY)
    }

    fn entry_inc(&self) {
        self.write_command(LCD16X2_CHARACTER_ENTRY_MODE | LCD16X2_INCREMENT |
                            LCD16X2_DISPLAY_SHIFT_OFF)
    }

    fn getxy(&self) -> u8 {
        self.wait_busy()
    }

    fn new_line(&self, pos: u8) {
        let mut address_counter: u8 = 0;
        if pos < LCD16X2_START_LINE_2 {
            address_counter = LCD16X2_START_LINE_2;
        }
        else {
            address_counter = LCD16X2_START_LINE_1;
        }
        self.write_command(LCD16X2_SET_DDRAM_ADDRESS | address_counter)
    }

    fn write_data(&self, data: u8) {
        self.wait_busy();
        self.write(data, true)
    }

    pub fn putc(&self, c: char) {
        let pos = self.getxy();
        if c == '\n' {
            self.new_line(pos);
        }
        else
        {
            if pos == (LCD16X2_START_LINE_1 + LCD16X2_DISP_LENGTH) {
                self.write(LCD16X2_SET_DDRAM_ADDRESS | LCD16X2_START_LINE_2, false)
            }
            else if pos == (LCD16X2_START_LINE_2 + LCD16X2_DISP_LENGTH) {
                self.write(LCD16X2_SET_DDRAM_ADDRESS | LCD16X2_START_LINE_1, false)
            }
            self.write_data(c as u8)
        }
    }

    pub fn puts(&self, s: &str) {
        for c in s.chars() {
            self.putc(c)
        }
    }

    pub fn set_backlight(&self, on: bool) {
        let gpiob = &self.p.GPIOB;
        if on {
            gpiob.brr.write(|w| bl_brr_set(w));
        } else {
            gpiob.bsrr.write(|w| bl_bsrr_set(w));
        }
    }

    pub fn gotoxy(&self, x: u8, y: u8) {
        if y == 0 {
            self.write_command(LCD16X2_SET_DDRAM_ADDRESS | (LCD16X2_START_LINE_1 + x))
        } else {
            self.write_command(LCD16X2_SET_DDRAM_ADDRESS | (LCD16X2_START_LINE_2 + x))
        }
    }

    pub fn create_custom_char(&self, mut loc: u8, bitmap: &[u8]) {
        // we only have 8 locations 0-7 for custom chars
        loc &= 0x07; 
        // set CGRAM address
        self.write_command(LCD16X2_SET_CGRAM_ADDRESS | (loc << 3));
        // Write 8 bytes custom char pattern
        for i in 0..8 {
            self.write_data(bitmap[i]);
        }
    }

    pub fn put_custom_char(&self, x: u8, y: u8, loc: u8) {
        self.gotoxy(x, y);
        self.write_data(loc)
    }

    pub fn init(&self, disp_attr: u8) {
        let rcc = &self.p.RCC;
        let gpioa = &self.p.GPIOA;
        let gpiob = &self.p.GPIOB;

        cortex_m::asm::delay(1000000);
        rcc.apb2enr.modify(|_, w|
            w.iopaen().enabled()    /* GPIOA */
             .iopben().enabled());  /* GPIOB */
        gpioa.crh.write(|w| {
            d7_cnf_bits(d7_mode_bits(w, 0b10), 0b00);
            d6_cnf_bits(d6_mode_bits(w, 0b10), 0b00);
            d5_cnf_bits(d5_mode_bits(w, 0b10), 0b00);
            d4_cnf_bits(d4_mode_bits(w, 0b10), 0b00);
            w
        });

        gpiob.crh.write(|w| {
            rs_cnf_bits(rs_mode_bits(w, 0b10), 0b00);
            rw_cnf_bits(rw_mode_bits(w, 0b10), 0b00);
            en_cnf_bits(en_mode_bits(w, 0b10), 0b00);
            bl_cnf_bits(bl_mode_bits(w, 0b10), 0b00);
            w
        });

        // delay power on
        delay_us(LCD16X2_DELAY_POWER_ON);


        // initialize 8-bit mode
        gpioa.bsrr.write(|w| d5_bsrr_set(w)); // function set
        gpioa.bsrr.write(|w| d4_bsrr_set(w)); // 8-bit mode

        self.toggle_en();
        // delay, busy flag can't be checked here
        delay_us(LCD16X2_DELAY_INIT);

        // repeat last command
        self.toggle_en();
        delay_us(LCD16X2_DELAY_INIT_REP);

        // repeat
        self.toggle_en();
        delay_us(LCD16X2_DELAY_INIT_REP);

        gpioa.bsrr.write(|w| d5_bsrr_set(w)); // function set
        gpioa.brr.write(|w| d4_brr_set(w)); // 4-bit mode
        self.toggle_en();
        delay_us(LCD16X2_DELAY_INIT_4BIT);

        /* from now the LCD only accepts 4 bit I/O */
        // 4-bit interface, 2 lines, 5x7 dot format font
        self.write_command(LCD16X2_FUNCTION_SET | LCD16X2_4BIT_INTERFACE |
                            LCD16X2_2LINE_MODE | LCD16X2_5X7DOT_FORMAT);
        // display off
        self.write_command(LCD16X2_DISPLAY_CURSOR_ON_OFF | LCD16X2_DISPLAY_OFF);
        // clear screen
        self.clrscr();
        // entry mode
        self.entry_inc();
        // display cursor on off
        let display_cursor_on_off_control = disp_attr;
        self.write_command(LCD16X2_DISPLAY_CURSOR_ON_OFF | display_cursor_on_off_control);
    }
}
