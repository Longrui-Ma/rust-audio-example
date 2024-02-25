//! Configuration for input, output ports and sample rate.
// TODO: verify in_buffer size = out_buffer, and same numbers of input_ports and output_ports 
use crate::import::*;

// #[derive(Debug)]
pub struct Config{
    pub input_ports: Vec<Port<AudioIn>>,
    pub output_ports: Vec<Port<AudioOut>>,
    pub app: Box<dyn AppTrait>, //trait object
}

impl Config{
    pub fn new(input_ports: Vec<Port<AudioIn>>, output_ports: Vec<Port<AudioOut>>, app: Box<dyn AppTrait>) -> Config {
        Config {
            input_ports,
            output_ports,
            app,
        }
    }
}

pub trait AppTrait: Send + Sync{
    fn tick(&mut self, in_frame: f32) -> f32;
    fn save_init(&mut self);
    fn load_init(&mut self);
}

impl ProcessHandler for Config {
    fn process(&mut self, _: &Client, ps: &ProcessScope) -> Control {
        self.app.save_init();
        for (in_port, out_port) in self.input_ports.iter().zip(&mut self.output_ports) {
            self.app.load_init();
            let in_buffer = in_port.as_slice(ps);
            let out_buffer = out_port.as_mut_slice(ps);
            for (in_frame, out_frame) in in_buffer.iter().zip(out_buffer.iter_mut()) { 
                *out_frame = self.app.tick(*in_frame);
            }
        }
        Control::Continue
    }
}
