use crate::connection::Connection;

/// Direction of the stepper CW = Clockwise / CCW = Counter clockwise
#[derive(PartialEq)]
pub enum Direction {
    CW,
    CCW,
}

pub trait Stepper {
    fn new(pins: (u8, u8, u8), connection: Connection) -> Self;
    fn move_to_position(&mut self, position: i32);
    fn move_steps(&mut self, steps: i32);
    fn set_steps_to_move(&mut self, steps: i32);
    fn step(&mut self) -> Result<(), &'static str>;
    fn set_direction(&mut self, direction: Direction);
}

// Trait used to activating a stepper ready for movements
pub trait Activatable {
    fn activate();
}
