//! An implementation of digital AGC based on the following paper:
//! - [Design and implementation of a new digital automatic gain control](https://hal.univ-lorraine.fr/hal-01397371/document)
//!
//! Demo: <https://sile.github.io/dagc/examples/agc.html>
#![warn(missing_docs)]

/// Possible errors.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("`desired_output_rms` must be a finite positive number, but got {value}")]
    InvalidDesiredOutputRms { value: f32 },

    #[error("`distortion_factor` must be a number within `0.0 ..= 1.0`, but got {value}")]
    InvalidDistortionFactor { value: f32 },
}

/// AGC for monaural channel.
#[derive(Debug)]
pub struct MonoAgc {
    desired_output_rms: f32,
    distortion_factor: f32,
    gain: f32,
    freezed: bool,
}

impl MonoAgc {
    /// Makes a new [`MonoAgc`] instance.
    ///
    /// `desired_output_rms` specifies the target volume in terms of RMS.
    ///
    /// `distortion factor` specifies a factor that determines how quickly or radically changes the gain value.
    /// If this value is too large, the AGC processing could introduce distortion to the output signals
    /// (usucally values such as `0.001` or `0.0001` are appropriate).
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

    /// If `freeze` is `true`, [`MonoAgc::process()`] becomes not to update the gain value.
    ///
    /// For example, if you apply AGC to an audio stream including speech, it would be preferable to freeze the gain update during the non-speech part to prevent amplifying background noises.
    ///
    pub fn freeze_gain(&mut self, freeze: bool) {
        self.freezed = freeze;
    }

    /// Returns whether the gain update is frozen or not.
    pub const fn is_gain_frozen(&self) -> bool {
        self.freezed
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
            if !self.freezed {
                let y = x.powi(2) / self.desired_output_rms;
                let z = 1.0 + (self.distortion_factor * (1.0 - y));
                self.gain *= z;
            }
        }
    }
}
