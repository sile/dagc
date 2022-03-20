#[derive(Debug)]
pub struct MonoAgc {
    desired_output_level: f32,
    distortion_factor: f32,
    gain: f32,
    freezed: bool,
}

impl MonoAgc {
    pub fn new(desired_output_level: f32, distortion_factor: f32) -> Self {
        // TODO: validate
        Self {
            desired_output_level,
            distortion_factor,
            gain: 1.0,
            freezed: false,
        }
    }

    pub fn freeze_gain(&mut self, freeze: bool) {
        self.freezed = freeze;
    }

    pub const fn is_gain_freezed(&self) -> bool {
        self.freezed
    }

    pub const fn gain(&self) -> f32 {
        self.gain
    }

    pub fn process(&mut self, samples: &mut [f32]) {
        for x in samples {
            *x *= self.gain;
            if !self.freezed {
                let y = (*x / self.desired_output_level).powi(2);
                let z = 1.0 + (self.distortion_factor * (1.0 - y));
                self.gain *= z.max(0.1); // `max(0.1)` is for preventing 0 multiplication
            }
        }
    }
}
