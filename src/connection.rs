use serialport::{ClearBuffer, DataBits, Parity, SerialPort, StopBits};
use std::io::Read;
use std::time::Duration;

pub enum ConnectionType {
    UART,
    SPI,
}

pub struct Connection {
    connection: ConnectionType,
    port: Box<dyn SerialPort>,
}

impl Connection {
    //const UART_PORT: &'static str = "/dev/ttyAMA0";
    const UART_PORT: &'static str = "/dev/ttyS0";
    const UART_BAUDRATE: u32 = 9600;
    const CALLING_PAUSE: u64 = 100;

    pub fn new(connection: ConnectionType) -> Self {
        let ports = serialport::available_ports().expect("No ports found!");
        println!("Available ports:");
        for p in ports {
            println!("{}", p.port_name);
        }

        Self {
            connection,
            port: Self::get_port(),
        }
    }

    fn get_port() -> Box<dyn SerialPort> {
        serialport::new(Self::UART_PORT, Self::UART_BAUDRATE)
            .timeout(Duration::from_secs((20000 / Self::UART_BAUDRATE).into()))
            .parity(Parity::None)
            .stop_bits(StopBits::One)
            .data_bits(DataBits::Eight)
            .open()
            .expect("Serial port could not connect")
    }

    /// Reads data via X retry's to ensure maximum success
    pub fn read(&mut self, mut read_data: Vec<u8>) -> Result<[u8; 4], &'static str> {
        println!("--- Read Reg: {:?}", read_data);
        let mut i = 0;

        while i < 10 {
            self.clear_input_output();
            let write_result = self.port.write(read_data.as_mut_slice());
            match write_result {
                Ok(result) => {
                    if result != read_data.len() {
                        println!("Error");
                        return Err("Missmatch in receive/response counts for reading.");
                    }
                    std::thread::sleep(Duration::from_millis(Self::CALLING_PAUSE));
                    let mut buffer: Vec<u8> = vec![0; 12];
                    self.port.read(buffer.as_mut_slice()).unwrap();
                    println!("Full reply...{:?}", buffer);
                    let return_read = buffer[7..11].try_into().unwrap();
                    std::thread::sleep(Duration::from_millis(Self::CALLING_PAUSE));
                    println!("--- Read Reg reply: {:?}", return_read);
                    return Ok(return_read);
                }
                Err(_) => {
                    println!("Failed to read data, retrying...")
                }
            }
            i = i + 1;
        }
        panic!("No valid answer from stepper after 10 tries.");
    }

    /// Writes to the register but does not check if write was successfull, that should be done in
    /// the calling file.
    pub fn write(&mut self, mut write_data: Vec<u8>) -> Result<(), &'static str> {
        println!("--- Write Reg: {:?}", write_data);

        self.clear_input_output();
        let write_result = self.port.write(write_data.as_mut_slice());
        std::thread::sleep(Duration::from_millis(Self::CALLING_PAUSE));

        match write_result {
            Ok(result) => {
                if result != write_data.len() {
                    return Err("Mismatch in receive/response counts for writing.");
                }
                Ok(())
            }
            Err(_) => Err("Error writing to register"),
        }
    }

    pub fn clear_input_output(&self) {
        self.port
            .clear(ClearBuffer::Output)
            .expect("Failed to discard output buffer");
        self.port
            .clear(ClearBuffer::Input)
            .expect("Failed to discard input buffer");
    }
}

#[cfg(test)]
mod tests {

    #[test]
    #[ignore]
    fn read() {}

    #[test]
    #[ignore]
    fn write() {}
}
