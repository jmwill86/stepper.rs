use stepper_rs::connection::uart;

fn main() {
    println!("Test println...");
    uart::connect();
    //this should error
    //let something = String::from("soasdasdas");

    //let v: Vec<_> = vec![1, 2, 3];
}
