use enigo::{
    Axis, Button, Coordinate,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};
use rand::prelude::*;
use std::fmt::Debug;
use std::ops::{Range, RangeInclusive};
use std::thread::sleep;
use std::time::Duration;

trait Operation: Debug {
    fn exec(&self, emulator: &mut Enigo, rand_generator: &mut ThreadRng) -> ();
}

#[derive(Debug)]
struct ClickOperation {
    init_x: i32,
    init_y: i32,
}

#[derive(Debug)]
struct SwitchWindowOperation;

#[derive(Debug)]
struct ScrollOperation;

#[derive(Debug)]
struct SwitchTabOperation;

#[derive(Debug)]
struct TypeCodeOperation;

impl ClickOperation {
    fn new(x: i32, y: i32) -> Self {
        Self {
            init_x: x,
            init_y: y,
        }
    }
}

impl Operation for ClickOperation {
    fn exec(&self, emulator: &mut Enigo, rand_generator: &mut ThreadRng) {
        let x_position = rand_generator.gen_range((self.init_x - 100)..=(self.init_x + 100));
        let y_position = rand_generator.gen_range((self.init_y - 100)..=(self.init_y + 100));

        println!("Move mouse to ({}; {}) and click", x_position, y_position);

        let _ = emulator.move_mouse(x_position, y_position, Coordinate::Abs);
        let _ = emulator.button(Button::Left, Click);
    }
}

impl Operation for SwitchWindowOperation {
    fn exec(&self, emulator: &mut Enigo, _: &mut ThreadRng) {
        println!("Switch window");

        let _ = emulator.key(Key::Alt, Press);
        let _ = emulator.key(Key::Tab, Press);

        let _ = emulator.key(Key::Alt, Release);
        let _ = emulator.key(Key::Tab, Release);
    }
}

impl Operation for ScrollOperation {
    fn exec(&self, emulator: &mut Enigo, rand_generator: &mut ThreadRng) {
        let scroll_len = rand_generator.gen_range(-100..=100);

        println!("Scroll window vertical length: {}", scroll_len);

        let mut range: RangeInclusive<i32> = 0..=0;

        if scroll_len > 0 {
            range = 1..=scroll_len;
        } else if scroll_len < 0 {
            range = scroll_len..=-1;
        }

        for n in range {
            let _ = emulator.scroll(n, Axis::Vertical);
        }
    }
}

impl Operation for SwitchTabOperation {
    fn exec(&self, emulator: &mut Enigo, _rand_generator: &mut ThreadRng) {
        println!("Switch tab");
        let _ = emulator.key(Key::Control, Press);
        let _ = emulator.key(Key::Tab, Click);
        let _ = emulator.key(Key::Tab, Click);
        let _ = emulator.key(Key::Control, Release);
    }
}

impl Operation for TypeCodeOperation {
    fn exec(&self, emulator: &mut Enigo, _rand_generator: &mut ThreadRng) {
        println!("Type code");
        let code = "const result = 5;";
        let log = "console.log('Debug => result is => ', result);";
        for n in code.chars() {
            let _ = emulator.text(&n.to_string());
        }
        let _ = emulator.key(Key::Return, Click);
        let _ = emulator.key(Key::Return, Click);

        for n in log.chars() {
            let _ = emulator.text(&n.to_string());
        }
    }
}

#[derive(PartialEq)]
enum WindowName {
    VsCode,
    Browser,
}

const INIT_TIME: u8 = 10;
const RANGE_BETWEEN_OPERATIONS: Range<u8> = 1..6;

fn main() {
    sleep(Duration::from_secs(INIT_TIME as u64));

    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let cursor_location: (i32, i32) = enigo.location().unwrap();

    let mut current_window = WindowName::VsCode;

    println!("x is: {}; y is: {}", cursor_location.0, cursor_location.1);

    let operations: Vec<Box<dyn Operation>> = vec![
        Box::new(ClickOperation::new(cursor_location.0, cursor_location.1)),
        Box::new(ClickOperation::new(cursor_location.0, cursor_location.1)),
        Box::new(ScrollOperation),
        Box::new(SwitchWindowOperation), // index 3
        Box::new(ClickOperation::new(cursor_location.0, cursor_location.1)),
        Box::new(ClickOperation::new(cursor_location.0, cursor_location.1)),
        Box::new(ScrollOperation),
        Box::new(ClickOperation::new(cursor_location.0, cursor_location.1)),
        Box::new(ScrollOperation),
        Box::new(ClickOperation::new(cursor_location.0, cursor_location.1)),
        Box::new(ClickOperation::new(cursor_location.0, cursor_location.1)),
        Box::new(SwitchTabOperation), 
        Box::new(TypeCodeOperation), // index 12
    ];

    let mut rng = thread_rng();

    loop {
        let operation_index = rng.gen_range(0..operations.len());
        println!("index {}", operation_index);
        let operation = operations.get(operation_index).unwrap();

        if operation_index != 12 || current_window == WindowName::VsCode {
            operation.exec(&mut enigo, &mut rng);
        }

        if operation_index == 3 {
            current_window  = match current_window {
                WindowName::VsCode => WindowName::Browser,
                WindowName::Browser => WindowName::VsCode,
            } 
        }

        let delay_after_operation: u8 = rng.gen_range(RANGE_BETWEEN_OPERATIONS);
        println!("delay_after_operation: {}", delay_after_operation);

        sleep(Duration::from_secs(delay_after_operation as u64));
    }
}
