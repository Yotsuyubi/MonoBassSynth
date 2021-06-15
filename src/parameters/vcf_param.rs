use vst::util::AtomicFloat;

pub struct VCFParam {
    pub cutoff: AtomicFloat,
    pub cutoff_mod: AtomicFloat,
    pub k: AtomicFloat,
    pub kbd_follow: AtomicFloat,
}

impl VCFParam {
    pub fn new(cutoff: f32, cutoff_mod: f32, k: f32, kbd: f32) -> Self {
        Self {
            cutoff: AtomicFloat::new(cutoff),
            cutoff_mod: AtomicFloat::new(cutoff_mod),
            k: AtomicFloat::new(k),
            kbd_follow: AtomicFloat::new(kbd),
        }
    }

    pub fn default() -> Self {
        Self {
            cutoff: AtomicFloat::new(0.5), // 5000kHz
            cutoff_mod: AtomicFloat::new(0.0),
            k: AtomicFloat::new(0.0),
            kbd_follow: AtomicFloat::new(0.0),
        }
    }
}
