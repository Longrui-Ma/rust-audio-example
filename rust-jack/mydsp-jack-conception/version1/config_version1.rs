//! Configuration for input, output ports and sample rate.
// use std::clone;
use crate::import::*;

#[derive(Debug)]
pub struct Config {
    pub input_ports: Vec<jack::Port<AudioIn>>,
    pub output_ports: Vec<jack::Port<AudioOut>>,
    pub sample_rate: f32,
}

impl Config {
    pub fn new(client: &jack::Client, input_ports: Vec<jack::Port<AudioIn>>, output_ports: Vec<jack::Port<AudioOut>>) -> Config {
        let sample_rate = client.sample_rate() as f32;
        // let sample_rate = Config::get_sample_rate(client);
        Config {
            input_ports,
            output_ports,
            sample_rate,
        }
    }
    // pub fn get_sample_rate(client: &jack::Client) -> f32{
    //     client.sample_rate() as f32
    // }
}

// pub trait Init {
//     fn get_sample_rate(&self) -> f32;
// }

// impl Init for Config {
//     fn get_sample_rate(&self) -> f32 {
//         self.get_sample_rate()
//     }
// }
