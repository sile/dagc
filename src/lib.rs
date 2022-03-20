#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("`desired_output_rms` must be a finite positive number, but got {value}")]
    InvalidDesiredOutputRms { value: f32 },

    #[error("`distortion_factor` must be a number within `0.0 ..= 1.0`, but got {value}")]
    InvalidDistortionFactor { value: f32 },
}

#[derive(Debug)]
pub struct MonoAgc {
    desired_output_rms: f32,
    distortion_factor: f32,
    gain: f32,
    freezed: bool,
}

impl MonoAgc {
    pub fn new(desired_output_rms: f32, distortion_factor: f32) -> Result<Self, Error> {
        if !(desired_output_rms > 0.0 && desired_output_rms.is_finite()) {
            return Err(Error::InvalidDesiredOutputRms {
                value: desired_output_rms,
            });
        }
        if !(0.0..=1.0).contains(&distortion_factor) {
            return Err(Error::InvalidDistortionFactor {
                value: distortion_factor,
            });
        }

        Ok(Self {
            desired_output_rms,
            distortion_factor,
            gain: 1.0,
            freezed: false,
        })
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
                let y = x.powi(2) / self.desired_output_rms;
                let z = 1.0 + (self.distortion_factor * (1.0 - y));
                self.gain *= z.max(0.1); // `max(0.1)` is for preventing 0 multiplication
            }
        }
    }
}
