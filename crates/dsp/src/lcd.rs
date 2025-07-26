pub struct HD44780<I2C, D> {
    pub i2c: I2C,
    delay: D,
    addr: u8,
}

impl<I2C, D, E> HD44780<I2C, D>
where
    I2C: embedded_hal::blocking::i2c::Write<Error = E>,
    D: embedded_hal::blocking::delay::DelayMs<u16> + embedded_hal::blocking::delay::DelayUs<u16>,
{
    pub fn new(i2c: I2C, delay: D, addr: u8) -> Self {
        Self { i2c, delay, addr }
    }

    pub fn init(&mut self) {
        self.delay.delay_us(50000); // wait > 40ms

        // init sequence (still in 8-bit mode)
        self.write_nibble(0b0011, false);
        self.delay.delay_us(4500);

        self.write_nibble(0b0011, false);
        self.delay.delay_us(4500);

        self.write_nibble(0b0011, false);
        self.delay.delay_us(150);

        self.write_nibble(0b0010, false); // switch to 4-bit mode

        // function set
        self.send(0b00101000, false); // 4-bit, 2 line, 5x8
        self.send(0b00001000, false); // display OFF
        self.send(0b00000001, false); // display clear
        self.delay.delay_us(2000);
        self.send(0b00000110, false); // entry mode: inc cursor
        self.send(0b00001100, false); // display ON, cursor off
    }

    pub fn clear(&mut self) {
        self.send_cmd(0x01);
        self.delay.delay_us(2);
    }

    pub fn send_cmd(&mut self, byte: u8) {
        self.send(byte, false);
    }

    pub fn send_data(&mut self, byte: u8) {
        self.send(byte, true);
    }

    pub fn set_cursor(&mut self, row: u8, col: u8) {
        let addr = match row {
            1 => 0x00,
            _ => 0x40,
        };

        self.send_cmd(0x80 | (addr + col));
    }

    pub fn set_row(&mut self, row: u8) {
        self.set_cursor(row, 0);
    }

    pub fn write_str(&mut self, str: &str) {
        for byte in str.bytes() {
            self.send_data(byte);
        }
    }

    fn send (&mut self, byte: u8, rs: bool) {
        self.write_nibble(byte >> 4, rs);       // high nibble
        self.write_nibble(byte & 0x0F, rs);     // low nibble
    }

    fn write_nibble(&mut self, nibble: u8, rs: bool) {
        let mut data = (nibble & 0x0F) << 4;

        if rs {
            data |= 1 << 0; // RS
        }

        data |= 1 << 2; // EN high
        data |= 1 << 3; // Backlight - P3

        self.i2c.write(self.addr, &[data]).ok();
        self.delay.delay_us(1);

        data &= !(1 << 2); // EN low
        self.i2c.write(self.addr, &[data]).ok();
        self.delay.delay_us(50);
    }
}
