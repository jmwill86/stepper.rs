use gpio_cdev::Chip;

#[derive(Copy, Clone, Debug)]
pub enum ConnectionType {
    UART,
    SPI,
}

pub trait StepperBuilder {
    type Builder: StepperBuilder;
    type Stepper;

    fn build(self) -> Self::Stepper;
    fn set_connection(self, connection: ConnectionType) -> Self::Builder;
}

pub trait Stepper {
    type Builder: StepperBuilder;

    fn new(pins: (u8, u8, u8)) -> Self::Builder;
    //fn move_to_position(&self, position: i32) -> i32;
    //fn step(&self);
    //fn set_direction(&self);
    //fn run();
    //fn move_to_position();
}
