use crate::connection::{Connection, ConnectionType};
use crate::stepper::{Direction, Stepper, StepperBuilder};
use gpio_cdev::{Chip, LineRequestFlags};

pub struct Tmc2209Builder {
    pins: (u8, u8, u8), // step, dir, en
    chip: Option<Chip>,
    connection: Connection,
}

impl StepperBuilder for Tmc2209Builder {
    type Builder = Tmc2209Builder;
    type Stepper = Tmc2209;

    fn set_connection(mut self, connection: ConnectionType) -> Self::Builder {
        self.connection = Connection::new(connection);
        self
    }

    /// Should be supplied with Chip::new().ok() to transform the Chip result to Option
    fn set_chip(mut self, chip: Option<Chip>) -> Self::Builder {
        self.chip = chip;
        self
    }

    fn build(self) -> Self::Stepper {

        match self.chip {
            Some(_) => println!("Chip found!") ,
            None => {

                let iterator = gpio_cdev::chips().expect("No chips found");
                let mut counter = 0;
                for chip1 in iterator {
                    counter = counter + 1;
                    println!(
                    "Available chip {}: {}",
                    counter,
                    chip1
                    .expect("Chips iter not found")
                    .path()
                    .to_str()
                    .unwrap()
                    );
                }
                panic!(
                "Loading Chip failed. There are {} chips available.",
                counter
                );
            }
        }


        let mut stepper = Tmc2209 {
            pins: self.pins,
            chip: self.chip.expect("Chip was not found in build process"),
            connection: self.connection,
            crc_parity: 0,
            msres: 256, //?
            current_position: 0,
        };
        stepper.init();
        stepper
    }
}

impl Tmc2209Builder {
    //pub fn set_connection(mut self, connection: ConnectionType) -> Self {
    //self.connection = connection;
    //self
    //}
}

pub struct Tmc2209 {
    pins: (u8, u8, u8), // step, dir, en
    chip: Chip,
    connection: Connection,
    crc_parity: u8,
    current_position: u16,
    msres: u16,
}

impl Stepper for Tmc2209 {
    type Builder = Tmc2209Builder;

    fn new(pins: (u8, u8, u8)) -> Self::Builder {
        Self::Builder {
            pins,
            chip: Chip::new("/dev/gpiochip0").ok(),
            connection: Connection::new(ConnectionType::UART),
        }
    }

    fn move_to_position(&self, position: i32) {}

    fn step(&self) {}

    fn set_direction(&self, direction: Direction) {}

    fn run(&self) {}
}

impl Tmc2209 {
    //const read_frame :Vec<u8> = jk
    // write_frame

    //const WRITE_FLAG: u8 = 0x00;
    //const READ_FLAG: u8 = 0x01;

    //// Addresses
    const GCONF: u8 = 0x00;
    const GSTAT: u8 = 0x01;
    const IFCNT: u8 = 0x02;
    //const IOIN: u8 = 0x06;
    //const IHOLD_IRUN: u8 = 0x10;
    //const TSTEP: u8 = 0x12;
    //const VACTUAL: u8 = 0x22;
    //const TCOOLTHRS: u8 = 0x14;
    //const SGTHRS: u8 = 0x40;
    //const SG_RESULT: u8 = 0x41;
    //const MSCNT: u8 = 0x6A;
    const CHOPCONF: u8 = 0x6C;
    //const DRVSTATUS: u8 = 0x6F;

    //// GCONF
    //const I_SCALE_ANALOG: u8 = 1 << 0;
    //const INTERNAL_RSENSE: u8 = 1 << 1;
    const EN_SPREADCYCLE: u8 = 1 << 2;
    //const SHAFT: u8 = 1 << 3;
    //const INDEX_OTPW: u8 = 1 << 4;
    //const INDEX_STEP: u8 = 1 << 5;
    //const MSTEP_REG_SELECT: u8 = 1 << 7;

    //// GSTAT
    const RESET: u8 = 1 << 0;
    const DRV_ERR: u8 = 1 << 1;
    //const UV_CP: u8 = 1 << 2;

    //// CHOPCONF
    //const VSENSE: u32 = 1 << 17;
    const MSRES0: u32 = 1 << 24;
    const MSRES1: u32 = 1 << 25;
    const MSRES2: u32 = 1 << 26;
    const MSRES3: u32 = 1 << 27;
    //const INTPOL: u32 = 1 << 28;

    //// IOIN
    //const IO_ENN: u8 = 1 << 0;
    //const IO_STEP: u8 = 1 << 7;
    //const IO_SPREAD: u16 = 1 << 8;
    //const IO_DIR: u16 = 1 << 9;

    //// DRVSTATUS
    //const STST: u32 = 1 << 31;
    //const STEALTH: u32 = 1 << 30;
    //const CS_ACTUAL: u32 = 31 << 16;
    //const T157: u16 = 1 << 11;
    //const T150: u16 = 1 << 10;
    //const T143: u16 = 1 << 9;
    //const T120: u16 = 1 << 8;
    //const OLB: u8 = 1 << 7;
    //const OLA: u8 = 1 << 6;
    //const S2VSB: u8 = 1 << 5;
    //const S2VSA: u8 = 1 << 4;
    //const S2GB: u8 = 1 << 3;
    //const S2GA: u8 = 1 << 2;
    //const OT: u8 = 1 << 1;
    //const OTPW: u8 = 1 << 0;

    //// IHOLD_IRUN
    //const IHOLD: u8 = 31 << 0;
    //const IRUN: u16 = 31 << 8;
    //const IHOLDDELAY: u32 = 15 << 16;

    //// SGTHRS
    //const SGTHRS_MOD: u8 = 255 << 0;

    pub fn get_connection(&self) -> &Connection {
        &self.connection
    }

    fn init(&mut self) {
        println!("init!");
        self.reset_gpios();
        self.read_steps_per_revolution();
        self.clear_gstat();
        self.connection.clear_input_output();        
        //self.connection.flush_output_buffer();
        //self.connection.flush_input_buffer();
    }

    fn reset_gpios(&mut self) {
        self.chip
            .get_line(self.pins.0 as u32)
            .unwrap()
            .request(LineRequestFlags::OUTPUT, 0, "output_pin_step")
            .expect("En pin could not be set as output");

        self.chip
            .get_line(self.pins.1 as u32)
            .unwrap()
            .request(LineRequestFlags::OUTPUT, 0, "output_pin_dir")
            .expect("En pin could not be set as output");

        self.chip
            .get_line(self.pins.2 as u32)
            .unwrap()
            .request(LineRequestFlags::OUTPUT, 0, "output_pin_en")
            .expect("En pin could not be set as output");
    }

    fn clear_gstat(&mut self) {
        let mut gstat: u32 = self.connection.read(self.get_read_bytes(Self::GSTAT)) as u32;
        //check here for 4 bytes being returned otherwise something went wrong and we should retry?
        gstat = Self::set_bit(gstat, Self::RESET as u32);
        gstat = Self::set_bit(gstat, Self::DRV_ERR as u32);
        self.connection
            .write(self.get_write_bytes(Self::GSTAT, gstat));
    }

    /// This does the write but also checks the IFCNT to ensure the write was successful or not.
    fn write_check(&self, write_reg: Vec<u8>) -> Result<u8, &'static str> {
        let ifcnt1 = self.get_read_bytes(Self::IFCNT);
        let return_val = self.connection.write(write_reg);
        let ifcnt2 = self.get_read_bytes(Self::IFCNT);

        if ifcnt1 >= ifcnt2 {
            Err("Write check was not written to register - write count register was not increased")
        } else {
            Ok(1)
        }
    }

    fn read_steps_per_revolution(&mut self) -> u16 {
        //chopconf = self.tmc_uart.read_int(self.CHOPCONF)
        let chopconf = self.connection.read(self.get_read_bytes(Self::CHOPCONF));
        self.get_steps_per_rev(1u32)
    }

    fn get_steps_per_rev(&mut self, chopconf: u32) -> u16 {
        let mut msresdezimal =
            chopconf & (Self::MSRES0 | Self::MSRES1 | Self::MSRES2 | Self::MSRES3);
        msresdezimal = msresdezimal >> 24;
        msresdezimal = 8 - msresdezimal;
        self.msres = 2_u32.pow(msresdezimal) as u16;
        self.msres
    }

    fn read_gstat(&self) {
        //
    }

    /// Calculates CRC parity bit
    fn calculate_crc(&self, datagram: &mut Vec<u8>) -> u8 {
        let mut counter = datagram.len() - 1;
        let mut crc: u8 = 0;

        for byte in datagram.iter() {
            let mut new_byte = *byte;
            counter -= 1;

            for _ in 0..8 {
                if (crc >> 7) ^ (new_byte & 0x01) > 0 {
                    crc = ((crc << 1) ^ 0x07) & 0xFF;
                } else {
                    crc = (crc << 1) & 0xFF;
                }
                new_byte = new_byte >> 1; // This would change the inner byte, right? But I want to keep this for use in the calling function
            }

            if counter <= 0 {
                break;
            }
        }
        crc
    }

    /// Sets a speicific bit to 1
    fn set_bit<T: std::ops::BitOr<Output = T>>(register_bits: T, setting_bits: T) -> T {
        register_bits | (setting_bits)
    }

    /// Sets a specific bit to 0
    fn clear_bit<T: std::ops::Not<Output = T> + std::ops::BitAnd<Output = T>>(
        register_bits: T,
        setting_bits: T,
    ) -> T {
        register_bits & !(setting_bits)
    }

    /// Gets the full Vec for a write in correct format: [sync, address, register, 32bit data, CRC]
    /// [8,8,8,32,8]
    fn get_write_bytes(&self, reg: u8, val: u32) -> Vec<u8> {
        let mut write_frame = vec![0xFF; 8];
        write_frame
        //self.rFrame[1] = self.mtr_id
        //self.rFrame[2] = reg
        //self.rFrame[3] = self.compute_crc8_atm(self.rFrame[:-1])
        //rtn = self.ser.write(self.rFrame)
    }

    /// Get the full Vec for a read in correct format: [sync, address, register, crc]
    /// [8,8,8,8]
    fn get_read_bytes(&self, reg: u8) -> Vec<u8> {
        // 8,8,8,8
        let mut read_frame = vec![0xFF; 4]; // could this be using with_capacity?
        read_frame[0] = 0x55;
        read_frame[1] = 0x00;
        read_frame[2] = reg;
        read_frame[3] = self.calculate_crc(&mut read_frame);
        read_frame
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::path::Path;
    use std::sync::Arc;

    fn get_mock_tmc() -> Tmc2209 {
        //let mockpath = Path::new(".");
        //let mockchip = gpio_cdev::Chip {
        //inner: Arc::new(gpio_cdev::InnerChip {
        //file: File::open(mockpath.as_ref()).unwrap(),
        //path: mockpath.as_ref().to_path_buf(),
        //name: "test".to_owned(),
        //label: "test".to_owned(),
        //lines: 1,
        //}),
        //};

        let mockchip = Chip::new("/dev/gpiochip0").unwrap();
        Tmc2209 {
            pins: (1, 1, 1), // step, dir, en
            chip: mockchip,
            connection: Connection::new(ConnectionType::UART),
            crc_parity: 0,
            current_position: 0,
            msres: 0,
        }
    }

    #[test]
    fn set_bit_u8() {
        let pre_bits: u8 = 0xB4;
        let mod_bits: u8 = 0x02;
        let post_bits: u8 = 0xB6;
        assert_eq!(Tmc2209::set_bit(pre_bits, mod_bits), post_bits)
    }

    #[test]
    fn set_bit_u16() {
        let pre_bits: u16 = 0b0110_0100_1001_0011;
        let mod_bits: u16 = 0b0000_0010_0000_0000;
        let post_bits: u16 = 0b0110_0110_1001_0011;
        assert_eq!(Tmc2209::set_bit(pre_bits, mod_bits), post_bits)
    }

    #[test]
    fn set_bit_u32() {
        let pre_bits: u32 = 0x541AED38;
        let mod_bits: u32 = 0x800000;
        let post_bits: u32 = 0x549AED38;
        assert_eq!(Tmc2209::set_bit(pre_bits, mod_bits), post_bits)
    }

    #[test]
    fn clear_bit_u8() {
        let pre_bits: u8 = 0x67;
        let mod_bits: u8 = 0x20;
        let post_bits: u8 = 0x47;
        assert_eq!(Tmc2209::clear_bit(pre_bits, mod_bits), post_bits)
    }

    #[test]
    fn clear_bit_u16() {
        let pre_bits: u16 = 0xE9C8;
        let mod_bits: u16 = 0x80;
        let post_bits: u16 = 0xE948;
        assert_eq!(Tmc2209::clear_bit(pre_bits, mod_bits), post_bits)
    }

    #[test]
    fn clear_bit_u32() {
        let pre_bits: u32 = 0xCD02F9E2;
        let mod_bits: u32 = 0x02;
        let post_bits: u32 = 0xCD02F9E0;
        assert_eq!(Tmc2209::clear_bit(pre_bits, mod_bits), post_bits)
    }

    #[test]
    fn crc_parity_test_read() {
        let the_tmc = get_mock_tmc();
        assert_eq!(the_tmc.calculate_crc(&mut vec![0x55, 0, 0, 0]), 207)
    }

    #[test]
    fn crc_parity_test_write() {
        let the_tmc = get_mock_tmc();
        assert_eq!(
            the_tmc.calculate_crc(&mut vec![85, 15, 0, 0, 13, 0, 0, 0]),
            173
        )
    }

    #[test]
    fn test_gstat() {
        let the_tmc = get_mock_tmc();
        assert_eq!(
            the_tmc.get_read_bytes(Tmc2209::set_bit(
                Tmc2209::GCONF as u8,
                Tmc2209::EN_SPREADCYCLE as u8
            )),
            vec![0x55, 0x00, 0x04, 47]
        )
    }

    #[test]
    fn get_drv_status_vec() {
        //let the_tmc = Tmc2209::new(1, 2, 3);

        //assert_eq!(
        //the_tmc.get_read_bytes(Tmc2209::GCONF as u32, Tmc2209::EN_SPREADCYCLE as u32),
        //vec![0x01; 4]
        //)
    }

    #[test]
    fn get_steps_per_rev() {
        let mut the_tmc = get_mock_tmc();

        assert_eq!(
            the_tmc.get_steps_per_rev(0b0101_0101_0101_0101_0101_0101_0101_0101),
            0b1000
        )
    }

    #[test]
    fn get_steps_per_rev2() {
        let mut the_tmc = get_mock_tmc();

        assert_eq!(
            the_tmc.get_steps_per_rev(0b0000_0000_0000_0000_0000_0000_0000_0000),
            0b100000000
        )
    }
}
