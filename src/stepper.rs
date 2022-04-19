use gpio_cdev::Chip;

#[derive(Copy, Clone, Debug)]
pub enum ConnectionType {
    UART,
    SPI,
}

pub trait Stepper {
    fn new(pins: (u32, u32, u32)) -> Self;
    fn move_to_position(&self, position: i32) -> i32;
    fn step(&self);
    fn set_direction(&self);
    fn run();
    fn move_to_position();
}
