pub enum ConnectionType {
    UART,
}

pub trait ConnectionTrait {
    fn new() -> Self;
    fn send(&self) -> Vec<u8>;
}

pub mod uart;
