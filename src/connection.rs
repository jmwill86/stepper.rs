//use clap::{Arg, Command};
use serialport::ClearBuffer;
use serialport::SerialPort;
use std::error::Error;
use std::io::{self, Read};
use std::panic::panic_any;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub enum ConnectionType {
    UART,
    SPI,
}

pub struct Connection {
    connection: ConnectionType,
}

impl Connection {
    const UART_PORT: &'static str = "/dev/ttyAMA0";
    const UART_BAUDRATE: u32 = 9600;

    pub fn new(connection: ConnectionType) -> Self {
        Self { connection }
    }

    fn get_port(&self) -> Box<dyn SerialPort> {
        serialport::new(Self::UART_PORT, Self::UART_BAUDRATE)
            .timeout(Duration::from_millis(10))
            .open()
            .expect("Serial port could not connect")
    }

    pub fn read(&self, mut read_data: Vec<u8>) -> usize {
        let mut port = self.get_port();
        port.write(read_data.as_mut_slice()).expect("Write failed!")
    }

    pub fn write(&self, write_data: Vec<u8>) {
        println!("Makes write call");
        //IFCNT           =   0x02

        //ifcnt1 = self.read_int(IFCNT)
        //self.write_reg(reg, val)
        //ifcnt2 = self.read_int(IFCNT)

        //if(ifcnt1 >= ifcnt2):
        //print("TMC2209: writing not successful!")
        //print("ifcnt:",ifcnt1,ifcnt2)
        //return False
        //else:
        //return True
    }

    pub fn flush_input_buffer(&self) {
        let chan_clear_buf = self.input_service();
        let port = self.get_port();

        println!(
            "Connected to {} at {} baud",
            Self::UART_PORT,
            Self::UART_BAUDRATE
        );

        loop {
            println!(
                "Bytes available to read: {}",
                port.bytes_to_read().expect("Error calling bytes_to_read")
            );

            match chan_clear_buf.try_recv() {
                Ok(_) => {
                    println!(
                        "------------------------- Discarding buffer ------------------------- "
                    );
                    port.clear(ClearBuffer::Input)
                        .expect("Failed to discard input buffer")
                }
                Err(mpsc::TryRecvError::Empty) => (),
                Err(mpsc::TryRecvError::Disconnected) => {
                    println!("Stopping.");
                    break;
                }
            }

            thread::sleep(Duration::from_millis(100));
        }
    }

    pub fn flush_output_buffer(&self) {
        let mut port = self.get_port();

        let chan_clear_buf = self.input_service();

        println!(
            "Connected to {} at {} baud",
            Self::UART_PORT,
            Self::UART_BAUDRATE
        );
        println!("Ctrl+D (Unix) or Ctrl+Z (Win) to stop. Press Return to clear the buffer.");

        let block = vec![0; 128]; // 128 may be wrong so needs to be checkled.
        todo!("We need to check 128 implementation");

        // This loop writes the block repeatedly, as fast as possible, to try to saturate the
        // output buffer. If you don't see much data queued to send, try changing the block size.
        loop {
            match port.write(&block) {
                Ok(_) => (),
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => panic!("Error while writing data to the port: {}", e),
            };

            match chan_clear_buf.try_recv() {
                Ok(_) => {
                    println!(
                        "------------------------- Discarding buffer ------------------------- "
                    );
                    port.clear(ClearBuffer::Output)
                        .expect("Failed to discard output buffer")
                }
                Err(mpsc::TryRecvError::Empty) => (),
                Err(mpsc::TryRecvError::Disconnected) => {
                    println!("Stopping.");
                    break;
                }
            }

            println!(
                "Bytes queued to send: {}",
                port.bytes_to_write().expect("Error calling bytes_to_write")
            );
        }

        todo!("Clear output not implemented");
    }

    fn input_service(&self) -> mpsc::Receiver<()> {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let mut buffer = [0; 32];
            loop {
                // Block awaiting any user input
                match io::stdin().read(&mut buffer) {
                    Ok(0) => {
                        drop(tx); // EOF, drop the channel and stop the thread
                        break;
                    }
                    Ok(_) => tx.send(()).unwrap(), // Signal main to clear the buffer
                    Err(e) => panic_any(e),
                }
            }
        });

        rx
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
