use stepper_rs::connection::Connection;
use stepper_rs::driver::tmc2209::Tmc2209;
use stepper_rs::driver::tmc2209::{ChopConfOption, GConfOption, MicrostepRes, Motor};
use stepper_rs::motion_controller::MotionController;
use stepper_rs::stepper::Direction;
use stepper_rs::stepper::Stepper;

#[tokio::main]
async fn main() {
    println!("Running main...");
    let connection = Connection::new();
    //let connection2 = Connection::new();
    let mut tmc = Tmc2209::new((13, 19, 26), connection); // .build(); // step, dir, en
    let tmc2 = Tmc2209::new((13, 19, 26), connection2); // .build(); // step, dir, en
                                                        //
                                                        //println!("Set dir");
                                                        //tmc.set_direction(Direction::CCW);
                                                        //println!("Enable Vsense");
                                                        //tmc.enable_chopconf_option(ChopConfOption::Vsense);
                                                        //println!("Set current");
                                                        //tmc.set_current(300);
                                                        //println!("Enable Iscale");
                                                        //tmc.enable_gconf_option(GConfOption::IScaleAnalogue);
                                                        //println!("Enable Intpol");
                                                        //tmc.enable_chopconf_option(ChopConfOption::Intpol);
                                                        //println!("Disable Spreadcycle");
                                                        //tmc.disable_gconf_option(GConfOption::SpreadCycle);
                                                        //println!("Microstep resolution");
                                                        //tmc.set_microstepping_resolution(MicrostepRes::Two);
                                                        //println!("Disable InternalRSense");
                                                        //tmc.disable_gconf_option(GConfOption::InternalRSense);

    ////// Read details
    //println!("Read IOIN");
    //tmc.read_IOIN();
    //tmc.read_CHOPCONF();
    //tmc.read_DRVSTATUS();
    //tmc.read_GCONF();

    ////tmc.set_acceleration(2000);
    ////tmc.set_max_speed(500);
    //tmc.set_motor_enabled(Motor::Enabled);

    //tmc.move_to_position(200);
    //tmc.move_steps(50);
    //tmc.move_steps(-50);

    // New way

    // Motion controller so we can controll the activation of which steper we're using inbetween
    // steps and swithc where needed
    let mut motion_controller = MotionController::new("stepper1".to_owned(), tmc);
    //let mut motion_controller2 = MotionController::new(tmc2);

    motion_controller.move_steps(50).await;
    motion_controller.move_steps(-50).await;

    //motion_controller2.move_steps(50);
    //motion_controller2.move_steps(-50);

    println!("Complete!");
}
