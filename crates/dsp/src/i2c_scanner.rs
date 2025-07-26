pub fn scan<I2C, E>(i2c: &mut I2C)
where
    I2C: embedded_hal::blocking::i2c::Write<Error = E>,
{
    let dummy = [0u8];
    for addr in 0x03u8..0x80 {
        if i2c.write(addr, &dummy).is_ok() {
            defmt::info!("Found device at 0x{:02X}", addr);
        }
    }
}
