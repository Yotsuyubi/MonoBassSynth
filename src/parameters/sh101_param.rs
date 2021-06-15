use crate::parameters;

use parameters::envelope_param::EnvelopeParam;
use parameters::vcf_param::VCFParam;
use parameters::vco_param::VCOParam;
use vst::plugin::PluginParameters;

use std::sync::Arc;

enum Params {
    VCO_RANGE,
    VCO_PULSE_WIDTH,
    VCO_PULSE_WIDTH_MOD,
    VCO_SAW_RATE,
    VCO_TRI_RATE,
    VCO_SQU_RATE,
    VCO_SUB_RATE,
    VCO_NOISE_RATE,

    VCA_ATTACK,
    VCA_DECAY,
    VCA_SUSTAIN,
    VCA_RELEASE,

    VCF_CUTOFF,
    VCF_CUTOFF_MOD,
    VCF_K,
    VCF_KBD,

    MOD_ATTACK,
    MOD_DECAY,
    MOD_SUSTAIN,
    MOD_RELEASE,

    UNKNOWN,
}

impl Params {
    pub fn from_i32(index: i32) -> Self {
        match index {
            0 => Self::VCO_RANGE,
            1 => Self::VCO_PULSE_WIDTH,
            2 => Self::VCO_PULSE_WIDTH_MOD,
            3 => Self::VCO_SAW_RATE,
            4 => Self::VCO_TRI_RATE,
            5 => Self::VCO_SQU_RATE,
            6 => Self::VCO_SUB_RATE,
            7 => Self::VCO_NOISE_RATE,

            8 => Self::VCA_ATTACK,
            9 => Self::VCA_DECAY,
            10 => Self::VCA_SUSTAIN,
            11 => Self::VCA_RELEASE,

            12 => Self::VCF_CUTOFF,
            13 => Self::VCF_CUTOFF_MOD,
            14 => Self::VCF_K,
            15 => Self::VCF_KBD,

            16 => Self::MOD_ATTACK,
            17 => Self::MOD_DECAY,
            18 => Self::MOD_SUSTAIN,
            19 => Self::MOD_RELEASE,

            _ => Self::UNKNOWN,
        }
    }
}

pub struct SH101Param {
    pub num_parameters: i32,
    pub vco_param: Arc<VCOParam>,
    pub vca_param: Arc<EnvelopeParam>,
    pub vcf_param: Arc<VCFParam>,
    pub mod_param: Arc<EnvelopeParam>,
}

impl SH101Param {
    pub fn new(
        vco_param: Arc<VCOParam>,
        vcf_param: Arc<VCFParam>,
        vca_param: Arc<EnvelopeParam>,
        mod_param: Arc<EnvelopeParam>,
    ) -> Self {
        Self {
            num_parameters: 8 + 4 + 4 + 4,
            vco_param: vco_param,
            vca_param: vca_param,
            vcf_param: vcf_param,
            mod_param: mod_param,
        }
    }
}

impl PluginParameters for SH101Param {
    fn get_parameter_label(&self, index: i32) -> String {
        let param = Params::from_i32(index);
        match param {
            Params::VCO_RANGE => "[-]".to_string(),
            Params::VCO_PULSE_WIDTH => "[-]".to_string(),
            Params::VCO_PULSE_WIDTH_MOD => "[-]".to_string(),
            Params::VCO_SAW_RATE => "[-]".to_string(),
            Params::VCO_TRI_RATE => "[-]".to_string(),
            Params::VCO_SQU_RATE => "[-]".to_string(),
            Params::VCO_SUB_RATE => "[-]".to_string(),
            Params::VCO_NOISE_RATE => "[-]".to_string(),

            Params::VCA_ATTACK => "[-]".to_string(),
            Params::VCA_DECAY => "[-]".to_string(),
            Params::VCA_SUSTAIN => "[-]".to_string(),
            Params::VCA_RELEASE => "[-]".to_string(),

            Params::VCF_CUTOFF => "[-]".to_string(),
            Params::VCF_CUTOFF_MOD => "[-]".to_string(),
            Params::VCF_K => "[-]".to_string(),
            Params::VCF_KBD => "[-]".to_string(),

            Params::MOD_ATTACK => "[-]".to_string(),
            Params::MOD_DECAY => "[-]".to_string(),
            Params::MOD_SUSTAIN => "[-]".to_string(),
            Params::MOD_RELEASE => "[-]".to_string(),

            Params::UNKNOWN => "".to_string(),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        let param = Params::from_i32(index);
        match param {
            Params::VCO_RANGE => "VCO Range".to_string(),
            Params::VCO_PULSE_WIDTH => "VCO Pulse Width".to_string(),
            Params::VCO_PULSE_WIDTH_MOD => "VCO Pulse Width Modulation Rate".to_string(),
            Params::VCO_SAW_RATE => "VCO Saw mix rate".to_string(),
            Params::VCO_TRI_RATE => "VCO Triangle mix rate".to_string(),
            Params::VCO_SQU_RATE => "VCO Square mix rate".to_string(),
            Params::VCO_SUB_RATE => "VCO Sub mix rate".to_string(),
            Params::VCO_NOISE_RATE => "VCO Noise mix rate".to_string(),

            Params::VCA_ATTACK => "VCA Attack".to_string(),
            Params::VCA_DECAY => "VCA Decay".to_string(),
            Params::VCA_SUSTAIN => "VCA Sustain".to_string(),
            Params::VCA_RELEASE => "VCA Release".to_string(),

            Params::VCF_CUTOFF => "VCF Cutoff".to_string(),
            Params::VCF_CUTOFF_MOD => "VCF Cutoff Modulation Rate".to_string(),
            Params::VCF_K => "VCF Resonance".to_string(),
            Params::VCF_KBD => "Keyboard Follow".to_string(),

            Params::MOD_ATTACK => "Mod Attack".to_string(),
            Params::MOD_DECAY => "Mod Decay".to_string(),
            Params::MOD_SUSTAIN => "Mod Sustain".to_string(),
            Params::MOD_RELEASE => "Mod Release".to_string(),

            Params::UNKNOWN => "".to_string(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        "".to_string()
    }

    fn set_parameter(&self, index: i32, value: f32) {
        let param = Params::from_i32(index);
        match param {
            Params::VCO_RANGE => self.vco_param.range.set(value),
            Params::VCO_PULSE_WIDTH => self.vco_param.pulse_width.set(value),
            Params::VCO_PULSE_WIDTH_MOD => self.vco_param.pulse_width_mod.set(value),
            Params::VCO_SAW_RATE => self.vco_param.saw_rate.set(value),
            Params::VCO_TRI_RATE => self.vco_param.tri_rate.set(value),
            Params::VCO_SQU_RATE => self.vco_param.squ_rate.set(value),
            Params::VCO_SUB_RATE => self.vco_param.sub_rate.set(value),
            Params::VCO_NOISE_RATE => self.vco_param.noise_rate.set(value),

            Params::VCA_ATTACK => self.vca_param.attack.set(value),
            Params::VCA_DECAY => self.vca_param.decay.set(value),
            Params::VCA_SUSTAIN => self.vca_param.sustain.set(value),
            Params::VCA_RELEASE => self.vca_param.release.set(value),

            Params::VCF_CUTOFF => self.vcf_param.cutoff.set(value),
            Params::VCF_CUTOFF_MOD => self.vcf_param.cutoff_mod.set(value),
            Params::VCF_K => self.vcf_param.k.set(value),
            Params::VCF_KBD => self.vcf_param.kbd_follow.set(value),

            Params::MOD_ATTACK => self.mod_param.attack.set(value),
            Params::MOD_DECAY => self.mod_param.decay.set(value),
            Params::MOD_SUSTAIN => self.mod_param.sustain.set(value),
            Params::MOD_RELEASE => self.mod_param.release.set(value),

            Params::UNKNOWN => (),
        }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        let param = Params::from_i32(index);
        match param {
            Params::VCO_RANGE => self.vco_param.range.get(),
            Params::VCO_PULSE_WIDTH => self.vco_param.pulse_width.get(),
            Params::VCO_PULSE_WIDTH_MOD => self.vco_param.pulse_width_mod.get(),
            Params::VCO_SAW_RATE => self.vco_param.saw_rate.get(),
            Params::VCO_TRI_RATE => self.vco_param.tri_rate.get(),
            Params::VCO_SQU_RATE => self.vco_param.squ_rate.get(),
            Params::VCO_SUB_RATE => self.vco_param.sub_rate.get(),
            Params::VCO_NOISE_RATE => self.vco_param.noise_rate.get(),

            Params::VCA_ATTACK => self.vca_param.attack.get(),
            Params::VCA_DECAY => self.vca_param.decay.get(),
            Params::VCA_SUSTAIN => self.vca_param.sustain.get(),
            Params::VCA_RELEASE => self.vca_param.release.get(),

            Params::VCF_CUTOFF => self.vcf_param.cutoff.get(),
            Params::VCF_CUTOFF_MOD => self.vcf_param.cutoff_mod.get(),
            Params::VCF_K => self.vcf_param.k.get(),
            Params::VCF_KBD => self.vcf_param.kbd_follow.get(),

            Params::MOD_ATTACK => self.mod_param.attack.get(),
            Params::MOD_DECAY => self.mod_param.decay.get(),
            Params::MOD_SUSTAIN => self.mod_param.sustain.get(),
            Params::MOD_RELEASE => self.mod_param.release.get(),

            Params::UNKNOWN => (0.0),
        }
    }
}
