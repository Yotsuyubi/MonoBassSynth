#[macro_use]
use crate::parameters;
use parameters::vco_param::VCOParam;
use rand::distributions::Standard;
use rand::prelude::*;
use std::f32::consts::PI;
use std::sync::Arc;

pub struct VCO {
    sample_rate: f32,
    time: i32,
    param: Arc<VCOParam>,
    moded_pw: f32,
}

impl VCO {
    pub fn new(param: Arc<VCOParam>, sample_rate: f32) -> Self {
        Self {
            sample_rate: sample_rate,
            time: 0,
            moded_pw: param.pulse_width.get(),
            param: param,
        }
    }

    pub fn default() -> Self {
        Self {
            sample_rate: 44100.0,
            time: 0,
            param: Arc::new(VCOParam::default()),
            moded_pw: 0.0,
        }
    }

    fn get_time(&self, n: i32, theta: f32, freq: f32) -> f32 {
        n as f32 + theta * self.sample_rate / (2.0 * PI * freq)
    }

    fn saw_osc(&self, n: i32, theta: f32, freq: f32) -> f32 {
        let time = self.get_time(n, theta, freq);
        2.0 * ((time * freq) / self.sample_rate - (time * freq / self.sample_rate + 0.5).floor())
    }

    fn tri_osc(&self, n: i32, theta: f32, freq: f32) -> f32 {
        let time = self.get_time(n, theta - PI / 2.0, freq);
        1.0 - 2.0 / PI * ((2.0 * PI * freq / self.sample_rate * time).cos()).acos()
    }

    fn squ_osc(&self, n: i32, theta: f32, freq: f32) -> f32 {
        let time = self.get_time(n, theta, freq);
        let condition = (2.0 * PI * freq / self.sample_rate * time).sin();
        if condition > 0.0 {
            1.0
        } else if condition == 0.0 {
            0.0
        } else {
            -1.0
        }
    }

    fn mixer(&self, n: i32, freq: f32) -> f32 {
        let saw_out = (self.saw_osc(n, 0.0, freq) + self.saw_osc(n, PI * self.moded_pw, freq))
            / 2.0
            * self.param.saw_rate.get();
        let tri_out = (self.tri_osc(n, 0.0, freq) + self.tri_osc(n, PI * self.moded_pw, freq))
            / 2.0
            * self.param.tri_rate.get();
        let squ_out = (self.squ_osc(n, 0.0, freq) + self.squ_osc(n, PI * self.moded_pw, freq))
            / 2.0
            * self.param.squ_rate.get();
        let sub_out = (self.squ_osc(n, 0.0, freq * 0.5)
            + self.squ_osc(n, PI * self.moded_pw, freq * 0.5))
            / 2.0
            * self.param.sub_rate.get();
        let noise_out: f32 = (StdRng::from_entropy().sample::<f32, Standard>(Standard) * 2.0 - 1.0)
            * self.param.noise_rate.get();
        (tri_out + saw_out + squ_out + sub_out + noise_out) / 5.0
    }

    pub fn mod_pw(&mut self, cv: f32) {
        self.moded_pw = self.param.pulse_width.get()
            + (1.0 - self.param.pulse_width.get()) * cv * self.param.pulse_width_mod.get();
    }

    pub fn tick(&mut self, freq: f32) -> f32 {
        let ranged_freq = freq; // * self.param.range.get();
        self.time += 1;
        self.mixer(self.time, ranged_freq)
    }
}
