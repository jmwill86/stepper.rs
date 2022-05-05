use stepper_rs::driver::tmc2209::Tmc2209;
use stepper_rs::driver::tmc2209::{ChopConfOption, GConfOption, Motor};
use stepper_rs::stepper::Direction;
use stepper_rs::stepper::Stepper;
use stepper_rs::stepper::StepperBuilder;

fn main() {
    println!("Running main...");

    let mut tmc = Tmc2209::new((13, 19, 26)).build(); // step, dir, en

    tmc.set_direction(Direction::CW);
    tmc.enable_chopconf_option(ChopConfOption::Vsense);
    //tmc.set_current(300);
    tmc.enable_gconf_option(GConfOption::IScaleAnalogue);
    tmc.enable_chopconf_option(ChopConfOption::Intpol);
    tmc.disable_gconf_option(GConfOption::SpreadCycle);
    //tmc.setMicrosteppingResolution(2)
    tmc.disable_gconf_option(GConfOption::InternalRSense);

    //// Read details
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
