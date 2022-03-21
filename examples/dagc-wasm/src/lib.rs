use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct MonoAgc(dagc::MonoAgc);

#[wasm_bindgen]
impl MonoAgc {
    #[wasm_bindgen(constructor)]
    pub fn new(desired_output_rms: f32, distortion_factor: f32) -> Result<MonoAgc, JsError> {
        let agc = dagc::MonoAgc::new(desired_output_rms, distortion_factor)?;
        Ok(MonoAgc(agc))
    }

    pub fn freeze_gain(&mut self, freeze: bool) {
        self.0.freeze_gain(freeze);
    }

    pub fn is_gain_freezed(&self) -> bool {
        self.0.is_gain_freezed()
    }

    pub fn gain(&self) -> f32 {
        self.0.gain()
    }

    pub fn process(&mut self, samples: &mut [f32]) {
        self.0.process(samples);
    }
}
