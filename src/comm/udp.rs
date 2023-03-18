use crate::spl::c::RoboCupGameControlData;
use crate::spl::c::RoboCupGameControlReturnData;
use crate::spl::gated::GCData;
use core::net::IpAddr::V4;
use core::net::Ipv4Addr;
use core::net::SocketAddr;
use std::net::UdpSocket;

pub struct GCLiaison {
    socket: UdpSocket,
    gc_target: SocketAddr,
    gcdata: GCData,
}

const _: () = assert!(crate::spl::c::GAMECONTROLLER_DATA_PORT <= u16::MAX as u32);
const _: () = assert!(crate::spl::c::GAMECONTROLLER_RETURN_PORT <= u16::MAX as u32);
const GC_DATA_PORT: u16 = crate::spl::c::GAMECONTROLLER_DATA_PORT as u16;
const GC_RETURN_PORT: u16 = crate::spl::c::GAMECONTROLLER_RETURN_PORT as u16;

impl GCLiaison {
    pub fn init_blocking() -> Self {
        let addr = SocketAddr::new(V4(Ipv4Addr::UNSPECIFIED), GC_DATA_PORT);
        let s = UdpSocket::bind(addr).unwrap();
        // before we set nonblocking, wait for a valid message then connect wherever we got it
        debug_println!("Waiting to hear from the GameController...");
        let (init_msg, addr) = recv(&s, true).unwrap();
        let gc_recv_addr = SocketAddr::new(addr.ip(), GC_RETURN_PORT);
        debug_println!("Valid data from {:#?}; assuming it's the GC & responding on :{:#?}", addr.ip(), GC_RETURN_PORT);
        s.set_nonblocking(true).unwrap();
        // SO_REUSEPORT???
        Self { socket: s, gc_target: gc_recv_addr, gcdata: GCData::new(init_msg) }
    }
    pub fn get(&mut self) {
        if let Some((data, _)) = recv(&self.socket, false) {
            self.gcdata.update(data);
            let send_struct = RoboCupGameControlReturnData {
                header: crate::spl::GC_RETURN_HEADER,
                version: crate::spl::GC_RETURN_VERSION,
                playerNum: 1,
                teamNum: crate::spl::TEAM_NUMBER,
                fallen: 0,
                pose: [0., 0., 0.],
                ballAge: 0.,
                ball: [0., 0.],
            };
            let sashimi = unsafe {
                std::slice::from_raw_parts(
                    &send_struct as *const _ as *const u8,
                    std::mem::size_of::<RoboCupGameControlReturnData>(),
                )
            };
            self.socket.send_to(sashimi, self.gc_target).unwrap();
        }
    }
}

#[inline(always)] // used exactly twice with different `block` arguments
pub fn recv(socket: &UdpSocket, block: bool) -> Option<(RoboCupGameControlData, SocketAddr)> {
    // https://stackoverflow.com/questions/25410028
    #[allow(clippy::uninit_assumed_init)]
    #[allow(invalid_value)]
    let mut recv_struct: RoboCupGameControlData = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
    let sashimi = unsafe {
        std::slice::from_raw_parts_mut(&mut recv_struct as *mut _ as *mut u8, std::mem::size_of::<RoboCupGameControlData>())
    };

    loop {
        let (_, srcaddr) = match socket.recv_from(sashimi) {
            Ok(val) => val,
            Err(why) => {
                if why.kind() == std::io::ErrorKind::WouldBlock {
                    if block {
                        continue;
                    } else {
                        return None;
                    }
                } else {
                    panic!("Couldn't receive UDP")
                }
            }
        };
        // TODO: make sure we actually received all of the data: loop while nbyte > 0 AND from same address
        // if nbyte == std::mem::size_of::<RoboCupGameControlData>() &&
        if recv_struct.header == crate::spl::GC_DATA_HEADER
            && recv_struct.teams.iter().any(|x| x.teamNumber == crate::spl::TEAM_NUMBER)
        {
            debug_assert_eq!(recv_struct.version, crate::spl::GC_DATA_VERSION);
            return Some((recv_struct, srcaddr));
        } else {
            debug_println!(
                "  Rejected message from {:#?} (header = {:#?}, teams = {:#?}); continuing...",
                srcaddr,
                recv_struct.header,
                recv_struct.teams
            );
        }
    }
}
