use fps_clock::*;
use std::{
    io::Write,
    net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4},
};

const ARTNET_PORT: u16 = 6454;

fn main() {
    let mut fps_clock = FpsClock::new(60);

    let rib_1 = Rib::new(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(169, 254, 5, 51)),
        ARTNET_PORT,
    ));
    let rib_2 = Rib::new(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(169, 254, 5, 52)),
        ARTNET_PORT,
    ));
    let rib_3 = Rib::new(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(169, 254, 5, 53)),
        ARTNET_PORT,
    ));
    let rib_4 = Rib::new(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(169, 254, 5, 54)),
        ARTNET_PORT,
    ));

    loop {
        let i = fps_clock.tick();
        print!("\rTime since last tick (in secs): {:?}", i / 1000000000.);
        std::io::stdout().flush().unwrap();
    }
}

#[derive(Clone, Copy, Default)]
struct Pixel {
    r: f32,
    g: f32,
    b: f32,
}

struct Rib {
    address: SocketAddr,
    bronchi_1: [Pixel; 600],
    bronchi_2: [Pixel; 600],
    bronchi_3: [Pixel; 600],
    bone_1: [Pixel; 300],
    bone_2: [Pixel; 300],
}
impl Rib {
    pub fn new(address: SocketAddr) -> Self {
        Rib {
            address,
            bronchi_1: [Default::default(); 600],
            bronchi_2: [Default::default(); 600],
            bronchi_3: [Default::default(); 600],
            bone_1: [Default::default(); 300],
            bone_2: [Default::default(); 300],
        }
    }
}
