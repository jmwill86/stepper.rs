use stepper_rs::driver::tmc2209::Tmc2209;
use stepper_rs::stepper::Stepper;

fn main() {
    println!("Running main...");

    //uart::connect();
    //let tmc2209 = tmc2209::new2();
    let motor_1 = Tmc2209::new(1, 2, 3);
    motor_1.clear_gstat();
    //motor_1::clear_gstat();

    //tmc = TMC_2209(16, 20, 21) # use your pins for pin_step, pin_dir, pin_en here

    //tmc.setMovementAbsRel(MovementAbsRel.absolute)
    //tmc.setDirection_reg(False)
    //tmc.setVSense(True)
    //tmc.setCurrent(300)
    //tmc.setIScaleAnalog(True)
    //tmc.setInterpolation(True)
    //tmc.setSpreadCycle(False)
    //tmc.setMicrosteppingResolution(2)
    //tmc.setInternalRSense(False)

    ////tmc.readIOIN()
    ////tmc.readCHOPCONF()
    ////tmc.readDRVSTATUS()
    ////tmc.readGCONF()

    //tmc.setAcceleration(2000)
    //tmc.setMaxSpeed(500)

    //tmc.setMotorEnabled(True)
    //tmc.runToPositionSteps(400)
}
