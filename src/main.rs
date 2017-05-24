mod chipate;

fn main() {
    println!("Starting Chipate!");

    let mut chip: chipate::Chipate = chipate::newChipate();
    chip.init();
    chip.load_program("pong");
}
