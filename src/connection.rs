//use clap::{Arg, Command};
use serialport::{DataBits,StopBits,Parity, SerialPort, ClearBuffer};
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
    port :Box<dyn SerialPort>  
}

impl Connection {
    //const UART_PORT: &'static str = "/dev/ttyAMA0";
    const UART_PORT: &'static str = "/dev/ttyS0";
    const UART_BAUDRATE: u32 = 9600;

    pub fn new(connection: ConnectionType) -> Self {
        let ports = serialport::available_ports().expect("No ports found!");
        println!("Available ports:");
        for p in ports {
            println!("{}", p.port_name);
        }

        Self { 
            connection,
            port:Self::get_port()
        }
    }

    fn get_port() -> Box<dyn SerialPort> {
        serialport::new(Self::UART_PORT, Self::UART_BAUDRATE)
            .timeout(Duration::from_secs((20000/Self::UART_BAUDRATE).into()))
            .parity(Parity::None)
            .stop_bits(StopBits::One)
            .data_bits(DataBits::Eight)
            .open()
            .expect("Serial port could not connect")
    }

    pub fn read(&mut self, mut read_data: Vec<u8>) -> usize {
        println!("Makes read call...{:?}", read_data);
        let mut i = 0;

        while i < 10 {
            self.clear_input_output();
            let write_result = self.port.write(read_data.as_mut_slice());
            // Check write response is the same length as call.
            //let read_result = self.port.read(read_data.as_mut_slice());
            match write_result {
                Ok(result) => { 
                    println!("Return ok: {:b}",result);
                    std::thread::sleep(Duration::from_millis(100));
                    let mut buffer: Vec<u8> = vec![0; 12];
                    let read_result = self.port.read(buffer.as_mut_slice());
                    std::thread::sleep(Duration::from_millis(100));
                    println!("Read result: {:?}", read_result);
                    return result
                }
                Err(e) => {
                    println!("Error: {}",e);
                }
            }
            // we needs to check bytes are acorrect length here
            // return(rtn[7:11])
            i = i + 1;
        }
        panic!("No valid answer from stepper after 10 tries.");
    }

    pub fn write(&self, write_data: Vec<u8>) {
        println!("Makes write call...");
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

    //pub fn flush_input_buffer(&self) {
        //let chan_clear_buf = self.input_service();

        //println!("Clearing Input buffer...");
        //println!(
            //"Connected to {} at {} baud",
            //Self::UART_PORT,
            //Self::UART_BAUDRATE
        //);

        //loop {
            //println!(
                //"Bytes available to read: {}",
                //self.port.bytes_to_read().expect("Error calling bytes_to_read")
            //);

            //match chan_clear_buf.try_recv() {
                //Ok(_) => {
                    //println!(
                        //"------------------------- Discarding buffer ------------------------- "
                    //);
                    //self.port.clear(ClearBuffer::Input)
                        //.expect("Failed to discard input buffer")
                //}
                //Err(mpsc::TryRecvError::Empty) => (),
                //Err(mpsc::TryRecvError::Disconnected) => {
                    //println!("Stopping.");
                    //break;
                //}
            //}

            //thread::sleep(Duration::from_millis(100));
        //}
    //}

    pub fn clear_input_output(&self) {
                    self.port.clear(ClearBuffer::Output)
                        .expect("Failed to discard output buffer");
                    self.port.clear(ClearBuffer::Input)
                        .expect("Failed to discard input buffer");
    }

    //pub fn flush_output_buffer(&mut self) {

        //let chan_clear_buf = self.input_service();
        //println!(
            //"Connected to {} at {} baud",
            //Self::UART_PORT,
            //Self::UART_BAUDRATE
        //);

        //println!("Clearing Output buffer...");

        //let block = vec![0; 128]; // 128 may be wrong so needs to be checkled.
        ////todo!("We need to check 128 implementation");

        //// This loop writes the block repeatedly, as fast as possible, to try to saturate the
        //// output buffer. If you don't see much data queued to send, try changing the block size.
        //loop {
            //match self.port.write(&block) {
                //Ok(_) => (),
                //Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                //Err(e) => panic!("Error while writing data to the port: {}", e),
            //};

            //match chan_clear_buf.try_recv() {
                //Ok(_) => {
                    //println!(
                        //"------------------------- Discarding buffer ------------------------- "
                    //);
                    //self.port.clear(ClearBuffer::Output)
                        //.expect("Failed to discard output buffer")
                //}
                //Err(mpsc::TryRecvError::Empty) => (),
                //Err(mpsc::TryRecvError::Disconnected) => {
                    //println!("Stopping.");
                    //break;
                //}
            //}

            //println!(
                //"Bytes queued to send: {}",
                //self.port.bytes_to_write().expect("Error calling bytes_to_write")
            //);
        //}
    //}

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
