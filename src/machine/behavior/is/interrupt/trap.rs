use machine::state::State;

pub fn trap(_state: &mut State, _x: u8, _y: u8, _z: u8) {
    // FIXME For the moment, TRAP halts the machine
    println!("Goodbye!");
    ::std::process::exit(0);
}
