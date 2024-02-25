//! Sinewave callback. Module `sine` of library crate `mydsp-jack`. 
//! ## TODO:
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
    phase: f32,
    gain: f32, //volumn
}

impl SineWave {
    pub fn new(sample_rate: usize, sine_table_size: usize, phase_init: impl Into<Option<f32>>, freq: impl Into<Option<f32>>, gain: impl Into<Option<f32>>) -> SineWave {
        let phase_init = phase_init.into().unwrap_or(0.0);
        let freq = freq.into().unwrap_or(440.0);
        let gain = gain.into().unwrap_or(1.0);
        let phase_increment = freq / sample_rate as f32;
        SineWave {
            sine_table: SineTable::new(sine_table_size),
            phasor: Phasor::new(phase_init, phase_increment),
            phase: phase_init,
            gain,
        }
    }
}

// note that update phasor.phase NOT sine.phase
impl AppTrait for SineWave {
    fn tick(&mut self, in_frame: f32) -> f32 {
        self.phase = self.phasor.tick(in_frame);
        self.sine_table.get_value(self.phase) * self.gain
    }
    fn save_init(&mut self) { 
        self.phasor.save_init();
    }
    fn load_init(&mut self) {
        self.phasor.load_init();
    }
}
