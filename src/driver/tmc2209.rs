use crate::connection::{Connection, ConnectionType};
use crate::stepper::{Direction, Stepper, StepperBuilder};
use gpio_cdev::{Chip, LineRequestFlags};
use std::time::Duration;

pub enum MicrostepRes {
    One = 1,
    Two = 2,
    Four = 4,
    Eight = 8,
    Sixteen = 16,
    ThirtyTwo = 32,
    SixtyFour = 64,
    OneTwoFive = 125,
    TwoFiveSix = 256,
}

pub enum GConfOption {
    Direction = 1 << 3,
    IScaleAnalogue = 1 << 0,
    InternalRSense = 1 << 1,
    SpreadCycle = 1 << 2,
    MStepResolution = 1 << 7,
}

pub enum ChopConfOption {
    Vsense = 1 << 17,
    Intpol = 1 << 28,
}

pub enum Motor {
    Enabled,
    Disabled,
}

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
            Some(_) => println!("Chip found!"),
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

        let stepper = Tmc2209 {
            pins: self.pins,
            chip: self.chip.expect("Chip was not found in build process"),
            connection: self.connection,
            //crc_parity: 0,
            msres: 256, //?
            current_direction: Direction::CW,
            current_position: 0,
            steps_to_move: 0,
        };
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
    //crc_parity: u8,
    current_position: u16,
    current_direction: Direction,
    steps_to_move: i32,
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

    /// Calculates the signed int amount of steps that need to be moved and in what direction and passes this to the step function
    fn move_to_position(&mut self, position: i32) {
        //let target_position = self.current_position as i32 + position;
        self.set_steps_to_move(position);
        while self.step().is_ok() {
            std::thread::sleep(Duration::from_micros(2000));
        }
        println!("Stepping move_to_position complete!");
    }

    fn move_steps(&mut self, steps: i32) {
        self.set_steps_to_move(steps);
        while self.step().is_ok() {
            std::thread::sleep(Duration::from_micros(2000));
        }
        println!("Stepping move_steps completed!");
    }

    /// Amount of steps we need to move in total in sigend-int format for direction. This works along with step() to
    /// ensure all steps are made and so we can calcular remaining steps and the timings inbweteen.
    fn set_steps_to_move(&mut self, steps: i32) {
        self.steps_to_move = steps;
    }

    /// Runs throuhg the amount of steps required and reduces the count as it goes so we can run
    /// this in sync for multiple motors. Returns Err() when no more steps remain.
    fn step(&mut self) -> Result<(), &'static str> {
        match self.steps_to_move {
            n if n > 0 => {
                self.steps_to_move -= 1;
                self.current_position += 1;
                self.set_direction(Direction::CW);
            }
            n if n < 0 => {
                self.steps_to_move += 1;
                self.current_position -= 1;
                self.set_direction(Direction::CCW);
            }
            _ => return Err("No more steps to move"),
        };

        let handle = self
            .chip
            .get_line(self.pins.0 as u32)
            .unwrap()
            .request(LineRequestFlags::OUTPUT, 0, "step_request")
            .unwrap();

        handle.set_value(1).unwrap();
        std::thread::sleep(Duration::from_micros(1));
        handle.set_value(0).unwrap();
        std::thread::sleep(Duration::from_micros(1));
        println!("Step Made!");
        Ok(())
    }

    fn set_direction(&mut self, direction: Direction) {
        if direction != self.current_direction {
            let reg = 1 << 3;
            let mut gconf = self.read_int(self.get_read_bytes(Self::GCONF));

            match direction {
                Direction::CW => {
                    gconf = Self::clear_bit(gconf, reg);
                }
                Direction::CCW => {
                    gconf = Self::set_bit(gconf, reg);
                }
            };

            self.write_check(self.get_write_bytes(Self::GCONF, gconf))
                .unwrap();

            self.current_direction = direction;
        } else {
            println!("Direction was already set to the requested option");
        }
    }
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
    const IOIN: u8 = 0x06;
    const IHOLD_IRUN: u8 = 0x10;
    //const TSTEP: u8 = 0x12;
    //const VACTUAL: u8 = 0x22;
    //const TCOOLTHRS: u8 = 0x14;
    //const SGTHRS: u8 = 0x40;
    //const SG_RESULT: u8 = 0x41;
    //const MSCNT: u8 = 0x6A;
    const CHOPCONF: u8 = 0x6C;
    const DRVSTATUS: u8 = 0x6F;

    //// GCONF
    const I_SCALE_ANALOG: u8 = 1 << 0;
    const INTERNAL_RSENSE: u8 = 1 << 1;
    const EN_SPREADCYCLE: u8 = 1 << 2;
    const SHAFT: u8 = 1 << 3;
    const INDEX_OTPW: u8 = 1 << 4;
    const INDEX_STEP: u8 = 1 << 5;
    const MSTEP_REG_SELECT: u8 = 1 << 7;

    //// GSTAT
    const RESET: u8 = 1 << 0;
    const DRV_ERR: u8 = 1 << 1;
    //const UV_CP: u8 = 1 << 2;

    //// CHOPCONF
    const VSENSE: u32 = 1 << 17;
    const MSRES0: u32 = 1 << 24;
    const MSRES1: u32 = 1 << 25;
    const MSRES2: u32 = 1 << 26;
    const MSRES3: u32 = 1 << 27;
    const INTPOL: u32 = 1 << 28;

    //// IOIN
    const IO_ENN: u8 = 1 << 0;
    const IO_STEP: u8 = 1 << 7;
    const IO_SPREAD: u16 = 1 << 8;
    const IO_DIR: u16 = 1 << 9;

    // DRVSTATUS
    const STST: u32 = 1 << 31;
    const STEALTH: u32 = 1 << 30;
    //const CS_ACTUAL: u32 = 31 << 16;
    //const T157: u16 = 1 << 11;
    //const T150: u16 = 1 << 10;
    //const T143: u16 = 1 << 9;
    //const T120: u16 = 1 << 8;
    const OLB: u8 = 1 << 7;
    const OLA: u8 = 1 << 6;
    const S2VSB: u8 = 1 << 5;
    const S2VSA: u8 = 1 << 4;
    const S2GB: u8 = 1 << 3;
    const S2GA: u8 = 1 << 2;
    const OT: u8 = 1 << 1;
    const OTPW: u8 = 1 << 0;

    //// IHOLD_IRUN
    //const IHOLD: u8 = 31 << 0;
    //const IRUN: u16 = 31 << 8;
    //const IHOLDDELAY: u32 = 15 << 16;

    //// SGTHRS
    //const SGTHRS_MOD: u8 = 255 << 0;

    pub fn get_connection(&self) -> &Connection {
        &self.connection
    }

    //fn clear(&mut self) {
    //println!("init!");
    //self.reset_gpios();
    ////self.read_steps_per_revolution(); // Not currenty used
    //self.clear_gstat();
    //self.connection.clear_input_output();
    //}

    pub fn init_default_settings(&mut self) {}

    pub fn reset_gpios(&mut self) {
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

    fn read_int(&mut self, reg: Vec<u8>) -> u32 {
        println!("--- Read int: {:?}", reg);
        let reply = u32::from_be_bytes(self.connection.read(reg).unwrap());
        println!("--- Read int reply: {:?}", reply);
        reply
    }

    pub fn clear_gstat(&mut self) {
        println!("Clear GSTAT");
        let mut gstat: u32 = self.read_int(self.get_read_bytes(Self::GSTAT));
        //check here for 4 bytes being returned otherwise something went wrong and we should retry?
        gstat = Self::set_bit(gstat, Self::RESET as u32);
        gstat = Self::set_bit(gstat, Self::DRV_ERR as u32);
        self.write_check(self.get_write_bytes(Self::GSTAT, gstat))
            .unwrap();
    }

    /// This does the write but also checks the IFCNT to ensure the write was successful or not.
    fn write_check(&mut self, write_reg: Vec<u8>) -> Result<u8, &'static str> {
        let ifcnt1 = self.read_int(self.get_read_bytes(Self::IFCNT));
        self.connection.write(write_reg).unwrap();
        let ifcnt2 = self.read_int(self.get_read_bytes(Self::IFCNT));

        if ifcnt1 >= ifcnt2 {
            println!(
                "Write not successfull. IFCNT was {:?} now {:?}.",
                ifcnt1, ifcnt2
            );
            Err("Write check was not written to register - write count register was not increased")
        } else {
            println!("Write was successfull!");
            Ok(1)
        }
    }

    fn read_steps_per_revolution(&mut self) -> u16 {
        let chopconf = self.read_int(self.get_read_bytes(Self::CHOPCONF)); // Read int here.
        self.get_steps_per_rev(chopconf)
    }

    fn get_steps_per_rev(&mut self, chopconf: u32) -> u16 {
        let mut msresdezimal =
            chopconf & (Self::MSRES0 | Self::MSRES1 | Self::MSRES2 | Self::MSRES3);
        msresdezimal = msresdezimal >> 24;
        msresdezimal = 8 - msresdezimal;
        self.msres = 2_u32.pow(msresdezimal) as u16;
        self.msres
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
        let val_split = val.to_be_bytes();
        let mut write_frame = vec![0xFF; 8];
        write_frame[0] = 0x55;
        write_frame[1] = 0x00;
        write_frame[2] = reg | 0x80;
        write_frame[3] = val_split[0];
        write_frame[4] = val_split[1];
        write_frame[5] = val_split[2];
        write_frame[6] = val_split[3];
        write_frame[7] = self.calculate_crc(&mut write_frame);
        write_frame
    }

    /// Get the full Vec for a read in correct format: [sync, address, register, crc]
    /// [8,8,8,8]
    fn get_read_bytes(&self, reg: u8) -> Vec<u8> {
        let mut read_frame = vec![0xFF; 4]; // could this be using with_capacity?
        read_frame[0] = 0x55;
        read_frame[1] = 0x00;
        read_frame[2] = reg;
        read_frame[3] = self.calculate_crc(&mut read_frame);
        read_frame
    }

    pub fn enable_gconf_option(&mut self, option: GConfOption) {
        let mut gconf = self.read_int(self.get_read_bytes(Self::GCONF));
        gconf = Self::set_bit(gconf, option as u32);
        self.write_check(self.get_write_bytes(Self::GCONF, gconf))
            .unwrap();
    }

    pub fn disable_gconf_option(&mut self, option: GConfOption) {
        let mut gconf = self.read_int(self.get_read_bytes(Self::GCONF));
        gconf = Self::clear_bit(gconf, option as u32);
        self.write_check(self.get_write_bytes(Self::GCONF, gconf))
            .unwrap();
    }

    pub fn enable_chopconf_option(&mut self, option: ChopConfOption) {
        let mut chopconf = self.read_int(self.get_read_bytes(Self::CHOPCONF));
        chopconf = Self::set_bit(chopconf, option as u32);
        self.write_check(self.get_write_bytes(Self::CHOPCONF, chopconf))
            .unwrap();
    }

    pub fn disable_chopconf_option(&mut self, option: ChopConfOption) {
        let mut chopconf = self.read_int(self.get_read_bytes(Self::CHOPCONF));
        chopconf = Self::clear_bit(chopconf, option as u32);
        self.write_check(self.get_write_bytes(Self::CHOPCONF, chopconf))
            .unwrap();
    }

    pub fn get_vsense(&mut self) -> u32 {
        let chopconf = self.read_int(self.get_read_bytes(Self::CHOPCONF));
        chopconf & Self::VSENSE
    }

    pub fn set_current(&mut self, current: u16) {
        let hold_current_multiplier = 0.5;
        let hold_current_delay = 10;
        let vref = 1.2;
        let rsense = 0.11;
        let vfs;

        if self.get_vsense() > 0 {
            vfs = 0.180 * vref / 2.5;
        } else {
            vfs = 0.325 * vref / 2.5;
        }

        let mut cs_irun = 32.0 * 1.41421 * (current as f32) / 1000.0 * (rsense + 0.02) / vfs - 1.0;
        cs_irun = cs_irun.min(31.0).max(0.0);
        let cs_ihold = hold_current_multiplier * cs_irun;
        let cs_irun_u32 = cs_irun.round() as u32;
        let cs_ihold_u32 = cs_ihold.round() as u32;
        self.set_irun_ihold(cs_ihold_u32, cs_irun_u32, hold_current_delay);
    }

    fn set_irun_ihold(&mut self, ihold: u32, irun: u32, hold_current_delay: u32) {
        let mut ihold_irun = 0;
        ihold_irun = ihold_irun | ihold << 0;
        ihold_irun = ihold_irun | irun << 8;
        ihold_irun = ihold_irun | hold_current_delay << 16;

        self.write_check(self.get_write_bytes(Self::IHOLD_IRUN, ihold_irun))
            .unwrap();
    }

    pub fn set_microstepping_resolution(&mut self, resolution: MicrostepRes) {
        let mut chopconf = self.read_int(self.get_read_bytes(Self::CHOPCONF));
        let mut msresdezimal = ((resolution as u8) as f32).log2() as u32;

        chopconf = chopconf & (!Self::MSRES0 | !Self::MSRES1 | !Self::MSRES2 | !Self::MSRES3);
        msresdezimal = 8 - msresdezimal;
        chopconf = chopconf & 0xF0FFFFFF;
        chopconf = chopconf | msresdezimal << 24;

        self.write_check(self.get_write_bytes(Self::CHOPCONF, chopconf))
            .unwrap();

        self.enable_gconf_option(GConfOption::MStepResolution);
    }

    pub fn set_motor_enabled(&mut self, enabled: Motor) {
        let handle = self
            .chip
            .get_line(self.pins.2 as u32)
            .unwrap()
            .request(LineRequestFlags::OUTPUT, 1, "motor_enabled")
            .unwrap();

        match enabled {
            Motor::Enabled => {
                handle.set_value(0).unwrap();
            }
            Motor::Disabled => {
                handle.set_value(1).unwrap();
            }
        }
    }

    #[allow(non_snake_case)]
    pub fn read_IOIN(&mut self) {
        println!("Reading IOIN: ---");

        let ioin = self.read_int(self.get_read_bytes(Self::IOIN));

        if ioin as u16 & Self::IO_SPREAD > 0 {
            println!("Spread is high");
        } else {
            println!("Spread is low");
        }

        if ioin as u16 & Self::IO_DIR > 0 {
            println!("Dir is high");
        } else {
            println!("Dir is low");
        }

        if ioin as u8 & Self::IO_STEP > 0 {
            println!("Step is high");
        } else {
            println!("Step is low");
        }

        if ioin as u8 & Self::IO_ENN > 0 {
            println!("En is high");
        } else {
            println!("En is low");
        }

        println!("------")
    }

    #[allow(non_snake_case)]
    pub fn read_CHOPCONF(&mut self) {
        println!("Reading ChopConfig: ---");

        let chopconf = self.read_int(self.get_read_bytes(Self::CHOPCONF));

        println!(
            "Native {:?} microstep setting",
            self.read_steps_per_revolution()
        );

        if chopconf & Self::INTPOL > 0 {
            println!("Interpolation to 256 microsteps");
        }

        if chopconf & Self::VSENSE > 0 {
            println!("1: High sensitivity, low sense resistor voltage");
        } else {
            println!("0: Low sensitivity, high sense resistor voltage");
        }

        println!("------");
    }

    #[allow(non_snake_case)]
    pub fn read_DRVSTATUS(&mut self) {
        println!("Reading DRIVER STATUS: ---");
        let drvstatus = self.read_int(self.get_read_bytes(Self::DRVSTATUS));

        if drvstatus & Self::STST > 0 {
            println!("TMC2209: Info: motor is standing still");
        } else {
            println!("TMC2209: Info: motor is running");
        }

        if drvstatus & Self::STEALTH > 0 {
            println!("TMC2209: Info: motor is running on StealthChop");
        } else {
            println!("TMC2209: Info: motor is running on SpreadCycle");
        }

        let drvstatus = drvstatus as u8;

        if drvstatus & Self::OLB > 0 {
            println!("TMC2209: Warning: Open load detected on phase B");
        }

        if drvstatus & Self::OLA > 0 {
            println!("TMC2209: Warning: Open load detected on phase A");
        }

        if drvstatus & Self::S2VSB > 0 {
            println!("TMC2209: Error: Short on low-side MOSFET detected on phase B. The driver becomes disabled");
        }

        if drvstatus & Self::S2VSA > 0 {
            println!("TMC2209: Error: Short on low-side MOSFET detected on phase A. The driver becomes disabled");
        }

        if drvstatus & Self::S2GB > 0 {
            println!(
                "TMC2209: Error: Short to GND detected on phase B. The driver becomes disabled. "
            );
        }

        if drvstatus & Self::S2GA > 0 {
            println!(
                "TMC2209: Error: Short to GND detected on phase A. The driver becomes disabled. "
            );
        }

        if drvstatus & Self::OT > 0 {
            println!("TMC2209: Error: Driver Overheating!");
        }

        if drvstatus & Self::OTPW > 0 {
            println!("TMC2209: Warning: Driver Overheating Prewarning!");
        }

        println!("---");
    }

    #[allow(non_snake_case)]
    pub fn read_GCONF(&mut self) {
        println!("Reading GCONF: ---");
        let gconf = self.read_int(self.get_read_bytes(Self::GCONF)) as u8;

        if gconf & Self::I_SCALE_ANALOG > 0 {
            println!("TMC2209: Driver is using voltage supplied to VREF as current reference");
        } else {
            println!("TMC2209: Driver is using internal reference derived from 5VOUT");
        }
        if gconf & Self::INTERNAL_RSENSE > 0 {
            println!(
                "TMC2209: Internal sense resistors. Use current supplied into VREF as reference."
            );
            println!("TMC2209: VREF pin internally is driven to GND in this mode.");
            println!("TMC2209: This will most likely destroy your driver!!!");
            panic!("SERIOUS ERROR DETECTED");
        } else {
            println!("TMC2209: Operation with external sense resistors");
        }
        if gconf & Self::EN_SPREADCYCLE > 0 {
            println!("TMC2209: SpreadCycle mode enabled");
        } else {
            println!("TMC2209: StealthChop PWM mode enabled");
        }
        if gconf & Self::SHAFT > 0 {
            println!("TMC2209: Inverse motor direction");
        } else {
            println!("TMC2209: normal motor direction");
        }
        if gconf & Self::INDEX_OTPW > 0 {
            println!("TMC2209: INDEX pin outputs overtemperature prewarning flag");
        } else {
            println!("TMC2209: INDEX shows the first microstep position of sequencer");
        }
        if gconf & Self::INDEX_STEP > 0 {
            println!("TMC2209: INDEX output shows step pulses from internal pulse generator");
        } else {
            println!("TMC2209: INDEX output as selected by index_otpw");
        }
        if gconf & Self::MSTEP_REG_SELECT > 0 {
            println!("TMC2209: Microstep resolution selected by MSTEP register");
        } else {
            println!("TMC2209: Microstep resolution selected by pins MS1, MS2");
        }

        println!("------");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::path::Path;
    use std::sync::Arc;

    fn get_mock_tmc() -> Tmc2209 {
        let mockchip = Chip::new("/dev/gpiochip0").unwrap();
        Tmc2209 {
            pins: (1, 1, 1), // step, dir, en
            chip: mockchip,
            connection: Connection::new(ConnectionType::UART),
            //crc_parity: 0,
            current_position: 0,
            msres: 0,
            steps_to_move: 0,
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
