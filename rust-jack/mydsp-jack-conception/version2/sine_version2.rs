//! Sinewave callback. Module `sine` of library crate `mydsp-jack`. 
//! 
//! ## TODO:
//! * fix phase problem in multichannel
//! * add panics
//! * add auto-test and panics-test
//! * `phase`, `freq`, `gain` in new() take integer `(i32, u32)` as well. (use enum() instead?)
//! * impact of using Option<f32> (for default values) instead of f32 remains unclear.
use crate::sine_table::SineTable;
use crate::phasor::Phasor;
use crate::config::AppTrait;

#[derive(Debug)]
pub struct SineWave {
    sine_table: SineTable,
    phasor: Phasor,
    phase: f32, //initial phase
    freq: f32, //frequency
    gain: f32, //volumn
    sample_rate: f32,
}

impl SineWave {
    pub fn new(sample_rate: f32, sine_table_size: usize, phase: impl Into<Option<f32>>, freq: impl Into<Option<f32>>, gain: impl Into<Option<f32>>) -> SineWave {
        let phase = phase.into().unwrap_or(0.0);
        let freq = freq.into().unwrap_or(440.0);
        let gain = gain.into().unwrap_or(1.0);
        let phase_increment = freq / sample_rate;
        SineWave {
            sine_table: SineTable::new(sine_table_size),
            phasor: Phasor::new(phase, phase_increment),
            phase,
            freq,
            gain,
            sample_rate, // here or in tick
        }
    }
}

impl AppTrait for SineWave {
    fn tick(&mut self) -> f32 {
        self.phase = self.phasor.tick();
        self.sine_table.get_value(self.phase) * self.gain
    }
}
