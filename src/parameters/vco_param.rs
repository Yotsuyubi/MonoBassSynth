use vst::util::AtomicFloat;

pub struct VCOParam {
    pub range: AtomicFloat,
    pub pulse_width: AtomicFloat,
    pub pulse_width_mod: AtomicFloat,
    pub saw_rate: AtomicFloat,
    pub tri_rate: AtomicFloat,
    pub squ_rate: AtomicFloat,
    pub sub_rate: AtomicFloat,
    pub noise_rate: AtomicFloat,
}

impl VCOParam {
    pub fn new(
        range: f32,
        pulse_width: f32,
        pulse_width_mod: f32,
        saw_rate: f32,
        tri_rate: f32,
        squ_rate: f32,
        sub_rate: f32,
        noise_rate: f32,
    ) -> Self {
        Self {
            range: AtomicFloat::new(range),
            pulse_width: AtomicFloat::new(pulse_width),
            pulse_width_mod: AtomicFloat::new(pulse_width_mod),
            saw_rate: AtomicFloat::new(saw_rate),
            tri_rate: AtomicFloat::new(tri_rate),
            squ_rate: AtomicFloat::new(squ_rate),
            sub_rate: AtomicFloat::new(sub_rate),
            noise_rate: AtomicFloat::new(noise_rate),
        }
    }

    pub fn default() -> Self {
        Self {
            range: AtomicFloat::new(1.0),
            pulse_width: AtomicFloat::new(0.0),
            pulse_width_mod: AtomicFloat::new(0.0),
            saw_rate: AtomicFloat::new(1.0),
            tri_rate: AtomicFloat::new(0.0),
            squ_rate: AtomicFloat::new(0.0),
            sub_rate: AtomicFloat::new(0.0),
            noise_rate: AtomicFloat::new(0.0),
        }
    }
}
