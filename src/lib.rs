#[macro_use]
extern crate vst;
extern crate vst_gui;

mod logics;
mod parameters;

use crate::vst::host::Host;
use vst::api::{Events, Supported};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::plugin::HostCallback;
use vst::plugin::{Category, Info, Plugin, PluginParameters};

use logics::envelope::ADSR;
use logics::vcf::VCF;
use logics::vco::VCO;
use parameters::envelope_param::EnvelopeParam;
use parameters::sh101_param::SH101Param;
use parameters::vcf_param::VCFParam;
use parameters::vco_param::VCOParam;

use std::sync::Arc;

fn midi_pitch_to_freq(pitch: u8) -> f32 {
    const A4_PITCH: i8 = 69;
    const A4_FREQ: f32 = 440.0;

    ((f32::from(pitch as i8 - A4_PITCH)) / 12.).exp2() * A4_FREQ
}

struct Sh101 {
    params: Arc<SH101Param>,
    vca: ADSR,
    vco: VCO,
    vcf: VCF,
    modulation: ADSR,
    note: u8,
}

impl Sh101 {
    fn process_midi_event(&mut self, data: [u8; 3]) {
        match data[0] {
            128 => self.note_off(data[1]),
            144 => self.note_on(data[1]),
            _ => (),
        }
    }

    fn note_on(&mut self, note: u8) {
        if note != self.note {
            self.vca.retrigger();
            self.modulation.retrigger();
        }
        self.note = note;
        self.vca.gate_on();
        self.modulation.gate_on();
    }

    fn note_off(&mut self, note: u8) {
        if note == self.note {
            self.vca.gate_off();
            self.modulation.gate_off();
        }
    }
}

impl Plugin for Sh101 {
    fn new(_host: HostCallback) -> Self {
        let vca_param = Arc::new(EnvelopeParam::default());
        let vco_param = Arc::new(VCOParam::default());
        let vcf_param = Arc::new(VCFParam::default());
        let mod_param = Arc::new(EnvelopeParam::default());
        let param = Arc::new(SH101Param::new(
            vco_param.clone(),
            vcf_param.clone(),
            vca_param.clone(),
            mod_param.clone(),
        ));
        Self {
            params: param,
            vca: ADSR::new(vca_param.clone(), 44100.0),
            vco: VCO::new(vco_param.clone(), 44100.0),
            vcf: VCF::new(vcf_param.clone(), 44100.0),
            modulation: ADSR::new(mod_param.clone(), 44100.0),
            note: 69,
        }
    }

    fn get_info(&self) -> Info {
        Info {
            name: "SH101".to_string(),
            vendor: "Psykhedelic Mandala".to_string(),
            unique_id: 1101,
            category: Category::Synth,
            inputs: 2,
            outputs: 2,
            parameters: self.params.num_parameters,
            ..Info::default()
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.params) as Arc<dyn PluginParameters>
    }

    #[allow(unused_variables)]
    #[allow(clippy::single_match)]
    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            match event {
                Event::Midi(ev) => self.process_midi_event(ev.data),
                _ => (),
            }
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let (inputs, outputs) = buffer.split();

        let (l, r) = inputs.split_at(1);
        let stereo_in = l[0].iter().zip(r[0].iter());

        let (mut l, mut r) = outputs.split_at_mut(1);
        let stereo_out = l[0].iter_mut().zip(r[0].iter_mut());

        for ((_left_in, _right_in), (left_out, right_out)) in stereo_in.zip(stereo_out) {
            let pitch = midi_pitch_to_freq(self.note);
            let env = self.vca.tick();
            let moduletion = self.modulation.tick();

            self.vco.mod_pw(moduletion);
            self.vcf.mod_fc(moduletion, pitch);

            let signal = self.vco.tick(pitch);
            let filtered_signal = self.vcf.filter(signal);
            let amplitude_controlled = filtered_signal * env;
            let out_sample = amplitude_controlled;

            *left_out = out_sample;
            *right_out = out_sample;
        }
    }
}

plugin_main!(Sh101);
