use artnet_protocol::{ArtCommand, Output};
use fps_clock::*;
use palette::{Hsv, IntoColor as _, Srgb};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};

const FPS: f32 = 5.0;
const ARTNET_PORT: u16 = 6454;
const ARTNET_PACKET_DATA_LENGTH: usize = 512;
const NUMBER_OF_PIXELS_IN_ARTNET_PACKET: usize = ARTNET_PACKET_DATA_LENGTH / 3;

fn main() {
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

    // let mut ribs = [rib_1];
    let mut ribs = [rib_1, rib_2, rib_3, rib_4];

    // We will tick
    let mut fps_clock = FpsClock::new(
        FPS as u32
            * ribs
                .iter()
                .fold(0usize, |acc_total_number_of_packets, rib| {
                    acc_total_number_of_packets
                        + rib
                            .strips
                            .iter()
                            .fold(0usize, |acc_number_of_packets_in_strip, strip| {
                                acc_number_of_packets_in_strip
                                    + strip.chunks(NUMBER_OF_PIXELS_IN_ARTNET_PACKET).count()
                            })
                }) as u32,
    );

    let mut socket = UdpSocket::bind(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        ARTNET_PORT,
    ))
    .unwrap();

    let mut breathe_pattern = BreathePattern::default();

    loop {
        let mut ns_since_last_tick: f32 = 0.0;

        ribs.iter().for_each(|rib| {
            rib.strips
                .iter()
                .enumerate()
                .for_each(|(strip_index, strip)| {
                    strip
                        .chunks(NUMBER_OF_PIXELS_IN_ARTNET_PACKET)
                        .enumerate()
                        .for_each(|(universe_index, pixels_in_universe)| {
                            // println!("sending");
                            ns_since_last_tick += fps_clock.tick();
                            socket
                                .send_to(
                                    &ArtCommand::Output(Output {
                                        data: pixels_in_universe
                                            .iter()
                                            .flat_map(|pixel| {
                                                pixel.rgb.iter().map(|component| {
                                                    GAMMA[(component * 255.0) as usize]
                                                })
                                            })
                                            .collect::<Vec<u8>>()
                                            .into(),
                                        port_address: ((10 + 10 * strip_index + universe_index)
                                            as u16)
                                            .try_into()
                                            .unwrap(),
                                        ..Default::default()
                                    })
                                    .write_to_buffer()
                                    .unwrap(),
                                    rib.address,
                                )
                                .unwrap();
                        })
                });
        });

        // let seconds_since_last_tick = 1.0 / FPS;
        let seconds_since_last_tick = ns_since_last_tick / 1000000000.0;
        breathe_pattern.tick(seconds_since_last_tick, &mut ribs);
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
                    24
                ], // Bronchi 1
                vec![
                    Pixel {
                        rgb: [0.5, 0.1, 0.1]
                    };
                    24
                ], // Bronchi 2
                vec![
                    Pixel {
                        rgb: [0.5, 0.1, 0.1]
                    };
                    24
                ], // Bronchi 3
                vec![
                    Pixel {
                        rgb: [0.5, 0.1, 0.1]
                    };
                    24
                ], // Bone 1
                vec![
                    Pixel {
                        rgb: [0.5, 0.1, 0.1]
                    };
                    24
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

const BREATHE_PERIOD: f32 = 3.;
const BRONCHI_COLORS_HSV: [(f32, f32, f32); 5] = [
    (30.0, 1.0, 1.0),
    (60.0, 1.0, 1.0),
    (30.0, 1.0, 1.0),
    (-30.0, 1.0, 1.0),
    (-30.0, 1.0, 1.0),
];
const BONE_COLORS_HSV: [(f32, f32, f32); 5] = [
    (270.0, 1.0, 1.0),
    (180.0, 1.0, 1.0),
    (210.0, 1.0, 1.0),
    (240.0, 1.0, 1.0),
    (300.0, 1.0, 1.0),
];
// A number of seconds which wil result in a full loop of the pattern
const SECONDS_LOOP: f32 =
    BRONCHI_COLORS_HSV.len() as f32 * BONE_COLORS_HSV.len() as f32 * BREATHE_PERIOD;
#[derive(Default)]
struct BreathePattern {
    seconds: f32,
}
impl BreathePattern {
    pub fn tick(&mut self, seconds_since_last_tick: f32, ribs: &mut [Rib]) {
        // self.seconds += ns_since_last_tick / 1000000000.;
        // let val = self.seconds.sin() / 2. + 0.5;
        // ribs.iter_mut().for_each(|rib| {
        //     rib.strips
        //         .iter_mut()
        //         .enumerate()
        //         .for_each(|(strip_index, strip)| {
        //             strip.iter_mut().for_each(|pixel| {
        //                 pixel.rgb[0] = val;
        //                 pixel.rgb[1] = val;
        //                 pixel.rgb[2] = val;
        //             });
        //         });
        // });

        println!("ns {}", seconds_since_last_tick);
        self.seconds += seconds_since_last_tick;
        self.seconds %= SECONDS_LOOP;
        let progress = ((self.seconds / BREATHE_PERIOD) + 1.) % 1.;
        println!("{}", progress);

        ribs.iter_mut().for_each(|rib| {
            rib.strips
                .iter_mut()
                .enumerate()
                .for_each(|(strip_index, strip)| {
                    let is_bronchi = strip_index < 3;
                    let period_index = (self.seconds / BREATHE_PERIOD) as usize;
                    let next_period_index = period_index + 1;
                    let (color, next_color) = if is_bronchi {
                        (
                            BRONCHI_COLORS_HSV[period_index % BRONCHI_COLORS_HSV.len()],
                            BRONCHI_COLORS_HSV[next_period_index % BRONCHI_COLORS_HSV.len()],
                        )
                    } else {
                        (
                            BONE_COLORS_HSV[period_index % BONE_COLORS_HSV.len()],
                            BONE_COLORS_HSV[next_period_index % BONE_COLORS_HSV.len()],
                        )
                    };

                    let color: Srgb = Hsv::new_srgb(
                        (1. - progress) * color.0 + progress * next_color.0,
                        (1. - progress) * color.1 + progress * next_color.1,
                        (1. - progress) * color.2 + progress * next_color.2,
                    )
                    .into_color();

                    strip.iter_mut().for_each(|pixel| {
                        pixel.rgb[0] = color.green;
                        pixel.rgb[1] = color.red;
                        pixel.rgb[2] = color.blue;
                    });
                });
        });
    }
}
