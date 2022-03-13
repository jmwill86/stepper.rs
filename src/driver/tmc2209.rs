use crate::connection::uart;

const WRITE_FLAG: u8 = 0x00;
const READ_FLAG: u8 = 0x01;

struct StepperConfig {
    connection_type: connection, // todo, this should probably be forced to use connection type interface in some way
}

pub fn new(dir_pin: u8, step_pin: u8, en_pin: u8) -> StepperConfig {
    println!("Creating new TMC instance");
    StepperConfig {}
}

/// This is the setting used to decide where to send the binary commands
fn get_connection_type() {}

fn get_drv_status() {
    self.connection_type.read();
}

fn get_gconf() {}

fn get_gstat() {}

fn cleat_gstat() {}

fn set_step_direction() {}

fn get_step_direction() {}

fn step() {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        todo!();
        //assert_eq!(2 + 2, 4);
    }
}
