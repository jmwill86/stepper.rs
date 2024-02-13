use crate::driver::tmc2209::Tmc2209;
use crate::stepper::Stepper;

pub struct MotionController<T> {
    stepper_motor: T, // @TODO - make this generic
    name: String,
}

impl<T> MotionController<T>
where
    T: Stepper,
{
    pub fn new(name: String, stepper: T) -> Self {
        Self {
            stepper_motor: stepper,
            name,
        }
    }

    pub async fn move_steps(&mut self, steps: i32) {
        println!("moving stepper {}", self.name);

        for i in 0..steps {
            println!("Moving step {}", i);
            //let _ = self.stepper_motor.step(); //.await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{connection::Connection, motion_controller, stepper::MockStepper};

    #[test]
    fn new() {
        panic!("failll");
    }

    #[test]
    fn move_steps() {
        let mock_connection = MockConnection;
        let mock_stepper = MockStepper::new((1, 2, 3), Connection::Chip);

        //let motion_controller = MotionController::new("test_stepper".to_owned(), mock_stepper);

        panic!("failll");
    }
}
