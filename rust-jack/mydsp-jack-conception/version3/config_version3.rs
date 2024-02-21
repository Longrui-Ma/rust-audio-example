//! Configuration for input, output ports and sample rate.
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
    fn tick(&mut self) -> f32;
    fn save_init(&mut self);
    fn load_init(&mut self);
    // fn print_phase(&mut self);
}

impl ProcessHandler for Config {
    fn process(&mut self, _: &Client, ps: &ProcessScope) -> Control {
        self.app.save_init();
        // println!("----------------{}", ps.n_frames());
        for port in &mut self.output_ports {
            self.app.load_init();
            let buffer = port.as_mut_slice(ps);
            for frame in buffer.iter_mut() {
                *frame = self.app.tick();
            }
            // self.app.print_phase();
        }
        // self.app.print_phase();
        Control::Continue
    }
}
