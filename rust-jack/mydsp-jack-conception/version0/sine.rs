//! Sinewave callback. Module `sine` of library crate `mydsp-jack`.
//! 
//! # Examples:
//! Pass `output_ports` and `sample_rate` using jack::{AudioOut, Client}.
//! 
//! `phase`(initial phase) should be between [0.0, 1), and `freq`(frequency) and `gain`(volumn) should be positive.
//! 
//! Pass `None` to `phase/freq/gain` to indicate using default values `(phase=0.0, freq=440.0, gain=1.0)`
//! ```
//! let sine_wave = SineWave::new(output_ports, sample_rate, 4096, 0.99, 1000.0, 0.5);
//! // use `None` to indicate using default values
//! let sine_wave_default = SineWave::new(output_ports, sample_rate, 4096, None, None, None);
//! ```
//! # Notes:
//! ## Default values
//! To achieve using default parameters, I have thought about other methods like using Default trait, builder pattern or simply expose 
//! fields which contain default values like `pub phase,`, but `phase: impl Into<Option<f32>>` seems to be the most elegant.
//! 
//! ## Method new()
//! Rust do not have null type, so use `Option<f32>`, which contains `Some()`, `None`. 
//! Without `phase: impl Into<Option<f32>>` and `phase.into().unwrap_or(0.0)`, this usage will not be possible:
//! ```
//! let sine_wave = SineWave::new(output_ports, sample_rate, 4096, None, 1000.0, 0.5);
//! ```
//! However, values of `phase`, `freq`, `gain` should only be `float` or `None`. Integers are not supported. (Monomorphization)
//! 
//! ## TODO:
//! * add trait
//! * add tick
//! * add panics
//! * add auto-test and panics-test
//! * `phase`, `freq`, `gain` in new() take integer `(i32, u32)` as well. (use enum() instead?)
//! * impact of using Option<f32> (for default values) instead of f32 remains unclear.
use crate::import::*;
use crate::sine_table::SineTable;
// use crate::config::Config;

#[derive(Debug)]
pub struct SineWave {
    output_ports: Vec<jack::Port<AudioOut>>, // multiple ports in one Vec
    sample_rate: f32,
    sine_table: SineTable,
    phase: f32, //initial phase
    freq: f32, //frequency
    gain: f32, //volumn
}

impl SineWave {
    pub fn new(output_ports: Vec<jack::Port<AudioOut>>, sample_rate: f32, size: usize, phase: impl Into<Option<f32>>, freq: impl Into<Option<f32>>, gain: impl Into<Option<f32>>) -> SineWave {
        let phase = phase.into().unwrap_or(0.0);
        let freq = freq.into().unwrap_or(440.0);
        let gain = gain.into().unwrap_or(1.0);
        SineWave {
            output_ports,
            sample_rate,
            sine_table: SineTable::new(size),
            phase,
            freq,
            gain,
        }
    }
}

impl ProcessHandler for SineWave {
    fn process(&mut self, _: &Client, ps: &ProcessScope) -> Control {
        // In one process, for different channels, initial phases are the same, so copy `phase` for upcoming channels.
        let phase_init = self.phase; 
        let mut end_phase= self.phase; // when last channel passes all frame, remember to update `phase`.
        let phase_increment = self.freq / self.sample_rate; // depends on frequncy of sinewave and device's sample rate.
        // println!("{} {} {} {} {}", ps.n_frames(), phase_increment, self.freq, self.gain, self.phase);
        for port in self.output_ports.iter_mut(){
            let output = port.as_mut_slice(ps);
            for i in 0..ps.n_frames() {
                let index = i as usize;
                let sample = self.sine_table.get_value(self.phase);
                output[index] = sample * self.gain;
                self.phase += phase_increment;
                if self.phase > 1.0 {
                    self.phase -= 1.0;
                }
            }
            end_phase = self.phase; 
            self.phase = phase_init;
        }
        // TODO: remove redundant assignment of `end_phase`, only keep the last one. Following method too complex.
        // self.phase += phase_increment * ps.n_frames() as f32;
        // while self.phase > 1.0 {
        //     self.phase -= 1.0;
        // }
        self.phase = end_phase;
        Control::Continue
    }
}
