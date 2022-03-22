use crate::connection::ConnectionTrait;

pub struct UART {}

impl ConnectionTrait for UART {
    fn new() -> Self {
        Self {}
    }

    fn send(&self) -> Vec<u8> {
        println!("Connection for UART fn modules works!");
        //let avec = vec![0; 4];
        //avec
        vec![0; 4]
    }

    //fn receive() {}
}
