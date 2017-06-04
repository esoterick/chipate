extern crate sdl;

//use sdl::event::Event;

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate clap;

mod chipate;
mod display;

use chipate::Chipate;

fn main() {
    env_logger::init().unwrap();
    let matches = clap_app!(myapp =>
                            (version: "1.0")
                            (author: "Robert J. Lambert III <rlambert85@gmail.com>")
                            (about: "Chip8 Emulator written in rust")
                            (@arg program: -p --program +required +takes_value "Program to run")
                            (@arg clock: -c --clock +required +takes_value "Clock speed in ms")
    )
            .get_matches();

    let program = matches.value_of("program").unwrap();
    println!("Value for program: {}", program);

    let clock = matches.value_of("clock").unwrap();
    println!("Value for clock: {}", clock);

    let mut chip = Chipate::new();
    chip.init();
    // chip.load_program("PONG");
    chip.load_program(program);
    chip.set_clock_speed(clock.parse::<u64>().unwrap());

    loop {
        chip.emulate_cycle();
        chip.display.draw_screen();
        chip.set_keys();
    }
}
