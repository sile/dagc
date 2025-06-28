//! An implementation of digital AGC based on the following paper:
//! - [Design and implementation of a new digital automatic gain control](https://hal.univ-lorraine.fr/hal-01397371/document)
//!
//! Demo: <https://sile.github.io/dagc/examples/agc.html>
#![warn(missing_docs)]

/// Possible errors.
#[derive(Debug)]
pub enum Error {
    /// The desired output RMS value is invalid (must be finite and positive).
    InvalidDesiredOutputRms {
        /// The invalid value that was provided.
        value: f32,
    },
    /// The distortion factor is invalid (must be between 0.0 and 1.0 inclusive).
    InvalidDistortionFactor {
        /// The invalid value that was provided.
        value: f32,
    },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidDesiredOutputRms { value } => {
                write!(
                    f,
                    "`desired_output_rms` must be a finite positive number, but got {value}",
                )
            }
            Error::InvalidDistortionFactor { value } => {
                write!(
                    f,
                    "`distortion_factor` must be a number within `0.0 ..= 1.0`, but got {value}",
                )
            }
        }
    }
}

impl std::error::Error for Error {}

/// AGC for monaural channel.
#[derive(Debug)]
pub struct MonoAgc {
    desired_output_rms: f32,
    distortion_factor: f32,
    gain: f32,
    frozen: bool,
}

impl MonoAgc {
    /// Makes a new [`MonoAgc`] instance.
    ///
    /// `desired_output_rms` specifies the target volume in terms of RMS.
    ///
    /// `distortion factor` specifies a factor that determines how quickly or radically changes the gain value.
    /// If this value is too large, the AGC processing could introduce distortion to the output signals
    /// (usucally values such as `0.001` or `0.0001` are appropriate).
    pub const fn new(desired_output_rms: f32, distortion_factor: f32) -> Result<Self, Error> {
        if !(desired_output_rms > 0.0 && desired_output_rms.is_finite()) {
            return Err(Error::InvalidDesiredOutputRms {
                value: desired_output_rms,
            });
        }
        if distortion_factor < 0.0 || distortion_factor > 1.0 {
            return Err(Error::InvalidDistortionFactor {
                value: distortion_factor,
            });
        }

        Ok(Self {
            desired_output_rms,
            distortion_factor,
            gain: 1.0,
            frozen: false,
        })
    }

    /// If `freeze` is `true`, [`MonoAgc::process()`] becomes not to update the gain value.
    ///
    /// For example, if you apply AGC to an audio stream including speech, it would be preferable to freeze the gain update during the non-speech part to prevent amplifying background noises.
    ///
    pub fn freeze_gain(&mut self, freeze: bool) {
        self.frozen = freeze;
    }

    /// Returns whether the gain update is frozen or not.
    pub const fn is_gain_frozen(&self) -> bool {
        self.frozen
    }

    /// Returns the current gain value.
    pub const fn gain(&self) -> f32 {
        self.gain
    }

    /// Applies AGC to an input frame.
    ///
    /// If [`MonoAgc::is_gain_frozen()`] is `false`, the gain value is also updated during the processing.
    pub fn process(&mut self, samples: &mut [f32]) {
        for x in samples {
            *x *= self.gain;
            if !self.frozen {
                let y = x.powi(2) / self.desired_output_rms;
                let z = 1.0 + (self.distortion_factor * (1.0 - y));
                self.gain *= z;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut agc = MonoAgc::new(0.001, 0.0001).expect("unreachable");
        assert_eq!(agc.gain(), 1.0);
        assert!(!agc.is_gain_frozen());
        agc.freeze_gain(true);
        assert!(agc.is_gain_frozen());

        let mut samples = [0.5, 1.0, -0.2];
        agc.process(&mut samples);
        assert_eq!(agc.gain(), 1.0);

        agc.freeze_gain(false);
        agc.process(&mut samples);
        assert_ne!(agc.gain(), 1.0);
    }
}
