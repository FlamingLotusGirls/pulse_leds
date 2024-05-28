use artnet_protocol::{ArtCommand, Output};
use fps_clock::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};

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

    let ribs = [rib_1, rib_2, rib_3, rib_4];

    let socket = UdpSocket::bind(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        ARTNET_PORT,
    ))
    .unwrap();

    loop {
        let _ns_since_last_tick = fps_clock.tick();

        ribs.iter().for_each(|r| {
            r.strips.iter().for_each(|strip| {
                socket
                    .send_to(
                        &ArtCommand::Output(Output {
                            data: strip
                                .iter()
                                .take(170)
                                .flat_map(|pixel| {
                                    pixel
                                        .rgb
                                        .iter()
                                        .map(|component| GAMMA[(component * 255.0) as usize])
                                })
                                .collect::<Vec<u8>>()
                                .into(),
                            port_address: 10.into(),
                            ..Default::default()
                        })
                        .write_to_buffer()
                        .unwrap(),
                        r.address,
                    )
                    .unwrap();
            });
        });
    }
}

#[derive(Clone, Copy, Default)]
struct Pixel {
    rgb: [f32; 3],
}

struct Rib {
    address: SocketAddr,
    strips: Vec<Vec<Pixel>>,
}
impl Rib {
    pub fn new(address: SocketAddr) -> Self {
        Rib {
            address,
            strips: vec![
                vec![
                    Pixel {
                        rgb: [0.5, 0.1, 0.1]
                    };
                    600
                ], // Bronchi 1
                vec![
                    Pixel {
                        rgb: [0.5, 0.1, 0.1]
                    };
                    600
                ], // Bronchi 2
                vec![
                    Pixel {
                        rgb: [0.5, 0.1, 0.1]
                    };
                    600
                ], // Bronchi 3
                vec![
                    Pixel {
                        rgb: [0.5, 0.1, 0.1]
                    };
                    300
                ], // Bone 1
                vec![
                    Pixel {
                        rgb: [0.5, 0.1, 0.1]
                    };
                    300
                ], // Bone 2
            ],
        }
    }
}

const GAMMA: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5,
    5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 9, 9, 9, 10, 10, 10, 11, 11, 11, 12, 12, 13, 13, 13, 14,
    14, 15, 15, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22, 23, 24, 24, 25, 25, 26, 27,
    27, 28, 29, 29, 30, 31, 32, 32, 33, 34, 35, 35, 36, 37, 38, 39, 39, 40, 41, 42, 43, 44, 45, 46,
    47, 48, 49, 50, 50, 51, 52, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 66, 67, 68, 69, 70, 72,
    73, 74, 75, 77, 78, 79, 81, 82, 83, 85, 86, 87, 89, 90, 92, 93, 95, 96, 98, 99, 101, 102, 104,
    105, 107, 109, 110, 112, 114, 115, 117, 119, 120, 122, 124, 126, 127, 129, 131, 133, 135, 137,
    138, 140, 142, 144, 146, 148, 150, 152, 154, 156, 158, 160, 162, 164, 167, 169, 171, 173, 175,
    177, 180, 182, 184, 186, 189, 191, 193, 196, 198, 200, 203, 205, 208, 210, 213, 215, 218, 220,
    223, 225, 228, 231, 233, 236, 239, 241, 244, 247, 249, 252, 255,
];
