use crate::connection::uart::UART;
use crate::connection::ConnectionTrait;
use crate::stepper::Stepper;

//const WRITE_FLAG: u8 = 0x00;
//const READ_FLAG: u8 = 0x01;

pub struct Tmc2209 {
    connection: UART,
    crc_parity: u8,
    //write_frame: Vec<u8>,
    //read_frame: Vec<u8>,
}

impl Tmc2209 {
    //const read_frame :Vec<u8> = jk
    // write_frame

    //// Addresses
    const GCONF: u8 = 0x00;
    //const GSTAT: u8 = 0x01;
    //const IFCNT: u8 = 0x02;
    //const IOIN: u8 = 0x06;
    //const IHOLD_IRUN: u8 = 0x10;
    //const TSTEP: u8 = 0x12;
    //const VACTUAL: u8 = 0x22;
    //const TCOOLTHRS: u8 = 0x14;
    //const SGTHRS: u8 = 0x40;
    //const SG_RESULT: u8 = 0x41;
    //const MSCNT: u8 = 0x6A;
    //const CHOPCONF: u8 = 0x6C;
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
    //const RESET: u8 = 1 << 0;
    //const DRV_ERR: u8 = 1 << 1;
    //const UV_CP: u8 = 1 << 2;

    //// CHOPCONF
    //const VSENSE: u32 = 1 << 17;
    //const MSRES0: u32 = 1 << 24;
    //const MSRES1: u32 = 1 << 25;
    //const MSRES2: u32 = 1 << 26;
    //const MSRES3: u32 = 1 << 27;
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

    fn read_gstat() {
        //
    }

    /// Calculates CRC parity bit
    fn calculate_crc(&self, datagram: Vec<u8>) -> u8 {
        // get all but the last of the vec
        0xFF
    }

    /// Sets a speicific bit to 1
    fn set_bit<T>(register_bits: T, setting_bits: T) -> T {
        register_bits | (setting_bits)
    }

    /// Sets a specific bit to 0
    fn clear_bit<T>(register_bits: T, setting_bits: T) -> T {
        register_bits & !(setting_bits)
    }

    /// Gets the full Vec for a write in correct format: [sync, address, register, 32bit data, CRC]
    /// [8,8,8,32,8]
    fn get_write_bytes(&self) -> Vec<u8> {
        let mut write_frame = vec![0xFF; 8];
        write_frame
        //self.rFrame[1] = self.mtr_id
        //self.rFrame[2] = reg
        //self.rFrame[3] = self.compute_crc8_atm(self.rFrame[:-1])
        //rtn = self.ser.write(self.rFrame)
    }

    /// Get the full Vec for a read in correct format: [sync, address, register, crc]
    /// [8,8,8,8]
    fn get_read_bytes(&self, reg: u8, modifier: u32) -> Vec<u8> {
        // 8,8,8,8
        let mut read_frame = vec![0xFF; 4]; // could this be using with_capacity?
        read_frame[0] = 0x55;
        read_frame[1] = 0x00;
        read_frame[2] = reg;
        read_frame[3] = self.calculate_crc(read_frame);
        read_frame
    }
}

impl Stepper for Tmc2209 {
    fn new(_pin: u32, _en: u32, _dir: u32) -> Self {
        Self {
            connection: UART::new(),
            crc_parity: 0,
        }
    }

    fn move_to_position(&self, position: i32) -> i32 {
        let some_vec: Vec<u8> = self.connection.send();
        2i32
    }

    fn step(&self) {}
    fn set_direction(&self) {}
}
#[cfg(test)]
mod tests {
    use super::*;

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
        let the_tmc = Tmc2209::new(1, 2, 3);
        assert_eq!(the_tmc.calculate_crc(vec![0x55, 0, 0, 0]), 207)
    }

    #[test]
    fn crc_parity_test_write() {
        let the_tmc = Tmc2209::new(1, 2, 3);
        assert_eq!(the_tmc.calculate_crc(vec![85, 15, 0, 0, 13, 0, 0]), 173)
    }

    #[test]
    fn test_gstat() {
        let the_tmc = Tmc2209::new(1, 2, 3);
        assert_eq!(
            the_tmc.get_read_bytes(Tmc2209::GCONF as u8, Tmc2209::EN_SPREADCYCLE as u32),
            vec![0x01; 4]
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
}
