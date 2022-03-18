pub trait Stepper {
    //fn new() -> Self
    fn new(pin: u32, en: u32, dir: u32) -> Self;
    fn move_to_position(&self, position: i32) -> i32;
    //fn get_connection_type() -> ConnectionType;
    //fn get_connection() -> ConnectionStruct;
    //fn step();
    //fn set_direction();
    //fn run();
    //fn set_acceleration();
    //fn move_to_position();
}
