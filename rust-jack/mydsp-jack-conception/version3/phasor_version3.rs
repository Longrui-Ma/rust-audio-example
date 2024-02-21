//! Phase management, make phase increase between [0, 1).
use crate::config::AppTrait;

#[derive(Debug)]
pub struct Phasor {
    phase: f32, // current phase
    phase_save: f32, // saved phase used as initial phase for all ports
    phase_increment: f32, // per tick
}

impl Phasor {
    pub fn new(initial_phase: f32, phase_increment: f32) -> Self {
        Phasor { 
            phase: initial_phase, 
            phase_save: 0.0,
            phase_increment 
        }
    }
    // for later use.
    pub fn set_phase_increment(&mut self, phase_increment: f32) {
        self.phase_increment = phase_increment;
    }
}

impl AppTrait for Phasor {
    fn tick(&mut self) -> f32 {
        self.phase += self.phase_increment;
        self.phase -= self.phase.floor(); // phase wraps around 0 to 1
        self.phase
    }
    fn save_init(&mut self) { // note that update phasor.phase not sine.phase
        self.phase_save = self.phase;
        // println!("save_init current_phase={}", self.phase);
    }
    fn load_init(&mut self) {
        self.phase = self.phase_save;
        // println!("load_init current_phase={}", self.phase);
    }
    // fn print_phase(&mut self) {
    //     println!("current_phase={}", self.phase);
    // }
}
