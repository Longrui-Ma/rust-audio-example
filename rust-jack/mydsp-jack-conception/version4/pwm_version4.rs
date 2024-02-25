use crate::config::AppTrait;

pub struct Pwm {
    // duty_cycle: f32, // range: [0,1]
    current_frame: usize, 
    current_frame_save: usize,
    on_frame: usize,
    period: usize, // e.g. =sample_rate
    in_app: Box<dyn AppTrait>,
}

impl Pwm {
    pub fn new(duty_cycle: f32, period: usize, in_app: Box<dyn AppTrait>) -> Self {
        Pwm {
            // duty_cycle,
            current_frame: 0,
            current_frame_save: 0,
            on_frame: (period as f32 * duty_cycle) as usize,
            period: period,
            in_app,
        }
    }
}

impl AppTrait for Pwm {
    fn tick(&mut self, in_frame: f32, port_index: usize) -> f32 {
        let output = if self.current_frame < self.on_frame {
            // println!("on");
            self.in_app.tick(in_frame, port_index)
        } else {
            // println!("off");
            0.0
        };
        self.current_frame = (self.current_frame + 1) % self.period;  
        output
    }
    fn save_init(&mut self) {
        self.current_frame_save = self.current_frame;
        self.in_app.save_init();
    }
    fn load_init(&mut self) {
        self.current_frame = self.current_frame_save;
        self.in_app.load_init();
    }
}
