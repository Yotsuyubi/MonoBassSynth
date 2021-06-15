#[macro_use]
use crate::parameters;
use parameters::vcf_param::VCFParam;
use std::f32::consts::PI;
use std::sync::Arc;

struct LPF1 {
    sample_rate: f32,
    cutoff: f32,
    yz1: f32,
    uz1: f32,
}

impl LPF1 {
    fn new() -> Self {
        Self {
            sample_rate: 44100.0,
            cutoff: 1000.0,
            yz1: 0.0,
            uz1: 0.0,
        }
    }

    fn set_fc(&mut self, fc: f32) {
        self.cutoff = fc;
    }

    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }

    fn filter(&mut self, x: f32) -> f32 {
        let wa = 2.0 * self.sample_rate * (PI * self.cutoff / self.sample_rate).tanh();
        let g = wa / (2.0 * self.sample_rate);
        let G = g / (1.0 + g);
        let v = x - self.yz1 - self.uz1;
        let u = G * v;
        let y = u + self.uz1 + self.yz1;
        self.yz1 = y;
        self.uz1 = u;
        y
    }
}

struct DiodeLadder {
    lpf1: LPF1,
    lpf2: LPF1,
    lpf3: LPF1,
    lpf4: LPF1,
    y2z1: f32,
    y3z1: f32,
    yz1: f32,
    sample_rate: f32,
    cutoff: f32,
}

impl DiodeLadder {
    fn new() -> Self {
        Self {
            lpf1: LPF1::new(),
            lpf2: LPF1::new(),
            lpf3: LPF1::new(),
            lpf4: LPF1::new(),
            cutoff: 1000.0,
            sample_rate: 44100.0,
            y2z1: 0.0,
            y3z1: 0.0,
            yz1: 0.0,
        }
    }

    fn set_fc(&mut self, fc: f32) {
        self.cutoff = fc;
        self.lpf1.set_fc(fc);
        self.lpf2.set_fc(fc);
        self.lpf3.set_fc(fc);
        self.lpf4.set_fc(fc);
    }

    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.lpf1.set_sample_rate(sample_rate);
        self.lpf2.set_sample_rate(sample_rate);
        self.lpf3.set_sample_rate(sample_rate);
        self.lpf4.set_sample_rate(sample_rate);
    }

    fn filter(&mut self, x: f32) -> f32 {
        let y1 = self.lpf1.filter(self.y2z1 + x);
        let y2 = self.lpf2.filter((self.y3z1 + y1) / 2.0);
        let y3 = self.lpf3.filter((self.yz1 + y2) / 2.0);
        let y = self.lpf4.filter(y3 / 2.0);
        self.y2z1 = y2;
        self.y3z1 = y3;
        self.yz1 = y;
        self.yz1
    }
}

struct DiodeLadderLPF {
    diode_ladder: DiodeLadder,
    yz1: f32,
    sample_rate: f32,
    cutoff: f32,
    k: f32,
    A: f32,
}

impl DiodeLadderLPF {
    fn new() -> Self {
        Self {
            diode_ladder: DiodeLadder::new(),
            cutoff: 1000.0,
            k: 0.0,
            sample_rate: 44100.0,
            yz1: 0.0,
            A: 1.0,
        }
    }

    fn set_fc(&mut self, fc: f32) {
        self.cutoff = fc;
        self.diode_ladder.set_fc(fc);
    }

    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.diode_ladder.set_sample_rate(sample_rate);
    }

    fn set_k(&mut self, k: f32) {
        self.k = k;
    }

    fn clip(&self, x: f32) -> f32 {
        1.0 / (1.0 as f32).tanh() * x.tanh()
    }

    fn filter(&mut self, x: f32) -> f32 {
        self.A = 1.0 + 0.5 * self.k * 20.0;
        let clipped_input = self.clip(x);
        let feedback = self.k * 14.0 * self.yz1;
        self.yz1 = self.diode_ladder.filter(clipped_input - feedback);
        self.A * self.yz1
    }
}

pub struct VCF {
    sample_rate: f32,
    param: Arc<VCFParam>,
    moded_fc: f32,
    lpf: DiodeLadderLPF,
}

impl VCF {
    pub fn new(param: Arc<VCFParam>, sample_rate: f32) -> Self {
        Self {
            sample_rate: sample_rate,
            lpf: DiodeLadderLPF::new(),
            moded_fc: param.cutoff.get(),
            param: param,
        }
    }

    pub fn default() -> Self {
        let param = Arc::new(VCFParam::default());
        Self {
            sample_rate: 44100.0,
            lpf: DiodeLadderLPF::new(),
            moded_fc: param.cutoff.get(),
            param: param,
        }
    }

    pub fn mod_fc(&mut self, cv: f32, note: f32) {
        self.moded_fc = self.param.cutoff.get() * self.sample_rate / 2.0
            + (self.sample_rate / 2.0 - self.param.cutoff.get() * self.sample_rate / 2.0)
                * cv
                * self.param.cutoff_mod.get()
            + note * 10.0 * self.param.kbd_follow.get()
    }

    pub fn filter(&mut self, x: f32) -> f32 {
        self.lpf.set_sample_rate(self.sample_rate);
        self.lpf.set_fc(self.moded_fc);
        self.lpf.set_k(self.param.k.get());

        self.lpf.filter(x)
    }
}
