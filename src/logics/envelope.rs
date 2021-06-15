#[macro_use]
use crate::parameters;
use parameters::envelope_param::EnvelopeParam;
use std::sync::Arc;

enum State {
    ATTACK,
    DECAY,
    RELEASE,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

pub struct ADSR {
    envelope_param: Arc<EnvelopeParam>,
    sample_rate: f32,
    gate: bool,
    state: i32,
    output: f32,
}

impl ADSR {
    pub fn new(envelope_param: Arc<EnvelopeParam>, sample_rate: f32) -> Self {
        Self {
            envelope_param: envelope_param,
            sample_rate: sample_rate,
            gate: false,
            state: 2,
            output: 0.0,
        }
    }

    pub fn default() -> Self {
        Self {
            envelope_param: Arc::new(EnvelopeParam::default()),
            sample_rate: 44100.0,
            gate: false,
            state: 2,
            output: 0.0,
        }
    }

    pub fn set_param(&mut self, param: Arc<EnvelopeParam>) {
        self.envelope_param = param;
    }

    pub fn gate_on(&mut self) {
        self.gate = true;
    }

    pub fn gate_off(&mut self) {
        self.gate = false;
    }

    pub fn retrigger(&mut self) {
        self.gate_off();
        self.output = 0.0;
        self.state = 2;
    }

    pub fn tick(&mut self) -> f32 {
        if self.output < 0.98 && self.gate == true && self.state != 1 {
            self.state = 0;
        } else if self.output > 0.98 && self.gate == true {
            self.state = 1;
        } else if self.gate == false {
            self.state = 2;
        }

        let mut alpha: f32 = 0.0;
        let mut beta: f32 = 0.0;

        match self.state {
            0 => {
                alpha = (-1.0 / (self.sample_rate * self.envelope_param.attack.get())).exp();
                beta = 1.0;
            }
            1 => {
                alpha = (-1.0 / (self.sample_rate * self.envelope_param.decay.get())).exp();
                beta = self.envelope_param.sustain.get();
            }
            2 => {
                alpha = (-1.0 / (self.sample_rate * self.envelope_param.release.get())).exp();
                beta = 0.0;
            }
            _ => (),
        };

        self.output = self.output * alpha + (1.0 - alpha) * beta;

        self.output
    }
}
