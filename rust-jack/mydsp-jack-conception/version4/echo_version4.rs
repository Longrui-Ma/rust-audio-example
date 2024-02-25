// 1 echo in lib enough, encapsulate multiple echo.tick() for more echo
// TODO: 
// * verify `feedback` < 1
use crate::config::AppTrait;

pub struct Echo {
    buffers_echo: Vec<Vec<f32>>, // circular buffers to store delayed frames (from multiple inputs).
    buffer_indices: Vec<usize>, // buffers' read and write position.
    nframes_delay: usize, // nframes_delay / sample_rate = time of delay.
    feedback: f32, // echo volumn decay rate [0,1).
    in_app: Box<dyn AppTrait>,
}

impl Echo{
    pub fn new(nframes_delay: usize, feedback: f32, port_pairs:usize, in_app: Box<dyn AppTrait>) -> Self {
        Echo {
            buffers_echo: vec![vec![0.0; nframes_delay]; port_pairs], // buffer size = nframes_delay.
            buffer_indices: vec![0; port_pairs], // starting point
            nframes_delay, // e.g. nframes_delay = sample_rate / 2;  // delay of 0.5s.
            feedback,
            in_app,
        }
    }
}

impl AppTrait for Echo {
    fn tick(&mut self, in_frame: f32, port_index: usize) -> f32 {
        let buffer_echo = &mut self.buffers_echo[port_index];
        let buffer_index = self.buffer_indices[port_index];

        let in_frame = in_frame + self.in_app.tick(in_frame, port_index); // combine in_frame (e.g. mic input) with app.tick()
        let delayed_frame = buffer_echo[buffer_index]; // read delayed frame.
        // println!("{}, {}, {}, {}", self.nframes_delay, buffer_index, in_frame, delayed_frame);
        buffer_echo[buffer_index] = in_frame; // write, copy new frame from input.
        self.buffer_indices[port_index] = (buffer_index + 1) % self.nframes_delay; // update write position.
        in_frame + delayed_frame * self.feedback
    }
    fn save_init(&mut self) {
        self.in_app.save_init();
    }
    fn load_init(&mut self) {
        self.in_app.load_init();
    }
}
