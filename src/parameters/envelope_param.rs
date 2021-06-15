use vst::util::AtomicFloat;

pub struct EnvelopeParam {
    pub attack: AtomicFloat,
    pub decay: AtomicFloat,
    pub sustain: AtomicFloat,
    pub release: AtomicFloat,
}

impl EnvelopeParam {
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        Self {
            attack: AtomicFloat::new(attack),
            decay: AtomicFloat::new(decay),
            sustain: AtomicFloat::new(sustain),
            release: AtomicFloat::new(release),
        }
    }

    pub fn default() -> Self {
        Self {
            attack: AtomicFloat::new(0.01),
            decay: AtomicFloat::new(0.1),
            sustain: AtomicFloat::new(0.5),
            release: AtomicFloat::new(0.3),
        }
    }
}
