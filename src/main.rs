mod chipate;

#[macro_use]
extern crate log;
extern crate env_logger;

fn main() {
    env_logger::init().unwrap();

    let mut chip = chipate::Chipate::new();
    chip.init();
    chip.load_program("pong");

    chip.setup_testing_memory();
    chip.emulate_cycle();
    chip.decode_opcode();

    loop {
        chip.emulate_cycle();
        if false {
            chip.draw_screen();
        }

        chip.set_keys();
    }
}
