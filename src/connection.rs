use crate::connection::uart::UART;

pub enum ConnectionType {
    UART,
}

pub trait ConnectionTrait {
    fn new() -> Self;
    fn send(&self) -> Vec<u8>;
}

pub struct ConnectionStruct {
    connection_type: UART,
}

pub mod uart;
