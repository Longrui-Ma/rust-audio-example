use crate::import::*;
use crate::sine_table::SineTable;

#[derive(Debug)]
pub struct PhasorEffect {
    input_port: jack::Port<AudioIn>,
    output_port: jack::Port<AudioOut>,
    phase: f32,
    frequency: f32,
    sample_rate: f32,
}

impl PhasorEffect {
    pub fn new(input_port: jack::Port<AudioIn>, output_port: jack::Port<AudioOut>, frequency: f32, sample_rate: f32) -> Self {
        PhasorEffect {
            input_port,
            output_port,
            phase: 0.0,
            frequency,
            sample_rate,
        }
    }
}

impl ProcessHandler for PhasorEffect {
    fn process(&mut self, _: &Client, ps: &ProcessScope) -> Control {
        let input = self.input_port.as_slice(ps);
        let output = self.output_port.as_mut_slice(ps);

        for (i, out_sample) in output.iter_mut().enumerate() {
            let in_sample = input[i];
            // let phasor_sample = (2.0 * std::f32::consts::PI * self.phase).sin();// without sineTable
            let sinetable = SineTable::new(2048); //todo sinetable problem?
            let phasor_sample = sinetable.get_value(self.phase);
            
            self.phase += self.frequency / self.sample_rate;
            if self.phase >= 1.0 {
                self.phase -= 1.0;
            }
            *out_sample = in_sample * phasor_sample;
        }
        Control::Continue
    }
}
