pub trait Stepper {
    //fn new() -> Self
    fn new(pin: u32, en: u32, dir: u32) -> Self;
    fn move_to_position(&self, position: i32) -> i32;
    fn step(&self);
    fn set_direction(&self);
    //fn run();
    //fn set_acceleration();
    //fn move_to_position();
}
