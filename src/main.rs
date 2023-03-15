#![crate_type = "bin"]
#![deny(warnings)] // equivalent of C -Werror
#![allow(dead_code)]
#![feature(ip_in_core, const_trait_impl, const_convert)]

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
