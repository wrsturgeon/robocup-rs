#![crate_type = "bin"]
#![deny(warnings)] // equivalent of C -Werror

/*
#![no_main]
#![no_std]

#[panic_handler]
#[allow(E0152)]
fn panic(_panic: &core::panic::PanicInfo) -> ! {
    reset_handler()
}

#[no_mangle]
pub extern "C" fn reset_handler() -> ! {
    loop {}
}

// Reset vector: a pointer into the reset handler
#[link_section = ".vector_table,reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: extern "C" fn() -> ! = reset_handler;
*/

mod spl;

fn main() {
    tokio::task::spawn(async {
        let socket = tokio::net::UdpSocket::bind("0.0.0.0:8080")
            .await
            .expect("Couldn't bind a UDP socket");
        let gcip = std::env::var("GAMECONTROLLER_IP").expect("`GAMECONTROLLER_IP` undefined");
        socket
            .connect(&gcip)
            .await
            .expect("Couldn't connect a UDP socket to GAMECONTROLLER_IP");
        let mut buf = [0; 1024];
        loop {
            let len = socket
                .recv(&mut buf)
                .await
                .expect("UDP socket couldn't receive data");
            println!("{:?} bytes received from {:?}", len, &gcip);

            let len = socket
                .send(&buf[..len])
                .await
                .expect("UDP socket couldn't send data");
            println!("{:?} bytes sent", len);
        }
    });
    println!(
        "UPennalizers are team number {:#?}",
        spl::config::Team::UPennalizers as u8
    );
}
