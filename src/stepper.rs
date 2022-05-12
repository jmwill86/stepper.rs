use crate::connection::ConnectionType;
use gpio_cdev::Chip;

/// Direction of the stepper CW = Clockwise / CCW = Counter clockwise
#[derive(PartialEq)]
pub enum Direction {
    CW,
    CCW,
}

/// Builder trait for any steppers to ensure no matter what type of stepper is being used that we
/// always have the correct interface
pub trait StepperBuilder {
    type Builder: StepperBuilder;
    type Stepper;

    fn build(self) -> Self::Stepper;
    fn set_connection(self, connection: ConnectionType) -> Self::Builder;
    fn set_chip(self, chip: Option<Chip>) -> Self::Builder;
}

pub trait Stepper {
    type Builder: StepperBuilder;

    fn new(pins: (u8, u8, u8)) -> Self::Builder;
    fn move_to_position(&mut self, position: i32);
    fn move_steps(&mut self, steps: i32);
    fn set_steps_to_move(&mut self, steps: i32);
    fn step(&mut self) -> Result<(), &'static str>;
    fn set_direction(&mut self, direction: Direction);
}
