use stepper_rs::driver::tmc2209::Tmc2209;
use stepper_rs::driver::tmc2209::{MicrostepRes, ChopConfOption, GConfOption, Motor};
use stepper_rs::stepper::Direction;
use stepper_rs::stepper::Stepper;
use stepper_rs::stepper::StepperBuilder;

fn main() {
    println!("Running main...");

    let mut tmc = Tmc2209::new((13, 19, 26)).build(); // step, dir, en
    println!("Set dir");
    tmc.set_direction(Direction::CCW);
    println!("Enable Vsense");
    tmc.enable_chopconf_option(ChopConfOption::Vsense);
    println!("Set current");
    tmc.set_current(300);
    println!("Enable Iscale");
    tmc.enable_gconf_option(GConfOption::IScaleAnalogue);
    println!("Enable Intpol");
    tmc.enable_chopconf_option(ChopConfOption::Intpol);
    println!("Disable Spreadcycle");
    tmc.disable_gconf_option(GConfOption::SpreadCycle);
    println!("Microstep resolution");
    tmc.set_microstepping_resolution(MicrostepRes::TWO);
    println!("Disable InternalRSense");
    tmc.disable_gconf_option(GConfOption::InternalRSense);

    //// Read details
    println!("Read IOIN");
    tmc.read_IOIN();
    tmc.read_CHOPCONF();
    tmc.read_DRVSTATUS();
    tmc.read_GCONF();

    //tmc.set_acceleration(2000);
    //tmc.set_max_speed(500);
    tmc.set_motor_enabled(Motor::Enabled);

    tmc.move_to_position(200);
    //tmc.move_steps(200);
    //tmc.move_steps(-200);

    println!("Complete!");
}
