use crate::connection::uart::UART;
//use crate::connection::Connection;
use crate::connection::ConnectionTrait;
use crate::stepper::Stepper;

const WRITE_FLAG: u8 = 0x00;
const READ_FLAG: u8 = 0x01;

pub struct Tmc2209 {
    connection: UART,
}

impl Tmc2209 {
    pub fn clear_gstat(self) -> i32 {
        println!("yay!");
        12
    }
}

impl Stepper for Tmc2209 {
    fn new(_pin: u32, _en: u32, _dir: u32) -> Self {
        Self {
            connection: UART::new(),
        }
    }

    fn move_to_position(&self, position: i32) -> i32 {
        let some_vec: Vec<u8> = self.connection.send();
        2i32
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        todo!();
    }
}
