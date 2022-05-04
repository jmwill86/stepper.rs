use stepper_rs::connection::ConnectionType;
use stepper_rs::driver::tmc2209::Tmc2209;
use stepper_rs::driver::tmc2209::{ChopConfOption, GConfOption};
use stepper_rs::stepper::Stepper;
use stepper_rs::stepper::StepperBuilder;

fn main() {
    println!("Running main...");

    let mut tmc = Tmc2209::new((13, 19, 26)).build(); // step, dir, en
    tmc.enable_gconf_option(GConfOption::IScale);
    tmc.enable_chopconf_option(ChopConfOption::Vsense);

    //tmc.setLoglevel(Loglevel.debug)
    //tmc.setMovementAbsRel(MovementAbsRel.absolute)

    //// Register changing settings
    //tmc.setDirection_reg(False)
    //tmc.setVSense(True)
    //tmc.setCurrent(300)
    //tmc.setIScaleAnalog(True)
    //tmc.setInterpolation(True)
    //tmc.setSpreadCycle(False)
    //tmc.setMicrosteppingResolution(2)
    //tmc.setInternalRSense(False)

    //// Read details
    //tmc.readIOIN()
    //tmc.readCHOPCONF()
    //tmc.readDRVSTATUS()
    //tmc.readGCONF()

    //tmc.setAcceleration(2000)
    //tmc.setMaxSpeed(500)
    //tmc.setMotorEnabled(True)

    //// actaully move the motor
    //tmc.runToPositionSteps(400)

    //println!("{:?}", stepper.get_connection());
    //panic!("end");

    //-----------------------------------------

    //let start: u32 = 0xFF;
    ////let changer: u8 = 1 << 0;
    //let changer: u32 = 128 >> 5;
    ////let end: u8 = start & !(changer);
    //let end: u32 = start | (changer);
    //println!("{:b}", start);
    //println!("{:b}", changer);
    //println!("{:b}", end);

    //tmc.read_gstat();
    ////stepper.set_movement_rel();
    ////stepper.set_direction(); // impl
    ////stepper.set_vsense();
    ////stepper.set_current();

    ////stepper.set_iscale_analog(true);
    ////stepper.set_interpolation(true);
    ////stepper.set_spreadcycle(false);
    ////stepper.set_microstepping_resolution(2);
    ////stepper.set_internal_rsense(false);

    ////stepper.read_ioin();
    ////stepper.read_chopconf();
    ////stepper.read_drv_status();
    ////stepper.read_gconf();

    ////stepper.set_acceleration(2000);
    ////stepper.set_max_speed(500);
    ////stepper.set_motor_enabled(true);

    //for i in 0..5 {
    //stepper.move_to_position(400);
    //stepper.move_to_position(0);
    //println!("Loop...{}", i);
    //thread::sleep_ms(2000);
    //}

    //println!("Complete!");
}
