use rppal::pwm::{Pwm, Channel, Polarity};
use std::thread::sleep;
use std::time::Duration;
use gilrs::{Axis, Button, Event, Gilrs};

const SERVO_MIN_MS: f64 = 1.0;
const SERVO_MAX_MS: f64 = 2.0;
const SERVO_FREQUENCY_HZ: f64 = 300.0;

fn main() {
    let pwm = Pwm::with_frequency(Channel::Pwm0, SERVO_FREQUENCY_HZ, 0.0, Polarity::Normal, true)
        .expect("Failed to initialize PWM");

    let mut gilrs = Gilrs::new().unwrap();
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }
    
    set_servo_position(&pwm, 0.0);
    
    loop {
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            match event {
                gilrs::EventType::AxisChanged(Axis::LeftStickY, val, _) => {
                    println!("left: {:?}", val)
                },
                gilrs::EventType::AxisChanged(Axis::RightZ, val, _) => {
                    println!("right: {:?}", val/-1.0);
                    set_servo_position(&pwm, val as f64);
                },
                _ => println!("now")
            }
        }
    }
    





//     // Move servo to the middle position
//     set_servo_position(&pwm, 0.5);
//     sleep(Duration::from_secs(1));

//     // Move servo to the maximum position
//     set_servo_position(&pwm, 1.0);
//     sleep(Duration::from_secs(1));

//     // Cleanup
//     pwm.set_duty_cycle(0.0).expect("Failed to set duty cycle");
}

fn set_servo_position(pwm: &Pwm, position: f64) {
    let duty_cycle = SERVO_MIN_MS + position * (SERVO_MAX_MS - SERVO_MIN_MS);
    let duty_cycle_fraction = duty_cycle / (1000.0 / SERVO_FREQUENCY_HZ);
    pwm.set_duty_cycle(duty_cycle_fraction).expect("Failed to set duty cycle");
}