#![crate_type = "bin"]
#![deny(warnings)]
#![feature(ip_in_core)]

#[macro_use]
extern crate debug_print;

mod comm;
mod spl;
mod state;

fn main() {
    debug_println!("We're team #{:#?}", spl::TEAM_NUMBER);

    let mut comm = crate::comm::udp::GCLiaison::init_blocking();
    loop {
        comm.get();
    }
    // println!("Robocup executable finished");
}
