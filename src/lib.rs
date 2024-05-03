use nih_plug::prelude::*;
use std::sync::Arc;

struct MidiInverter {
    params: Arc<MidiInverterParams>,
}

#[derive(Params)]
struct MidiInverterParams {
    #[id = "split"]
    pub split: IntParam,
}

impl Default for MidiInverter {
    fn default() -> Self {
        Self {
            params: Arc::new(MidiInverterParams::default()),
        }
    }
}

impl Default for MidiInverterParams {
    fn default() -> Self {
        Self {
            // This gain is stored as linear gain. NIH-plug comes with useful conversion functions
            // to treat these kinds of parameters as if we were dealing with decibels. Storing this
            // as decibels is easier to work with, but requires a conversion for every sample.
            split: IntParam::new("Split Point", 64, IntRange::Linear { min: 0, max: 127 }),
        }
    }
}

impl Plugin for MidiInverter {
    const NAME: &'static str = "Keyboard Splitter";
    const VENDOR: &'static str = "maybehelloworld";
    const URL: &'static str = "";
    const EMAIL: &'static str = "";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // This plugin doesn't have any audio IO
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[];

    const MIDI_INPUT: MidiConfig = MidiConfig::MidiCCs;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::MidiCCs;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // We'll invert the channel, note index, velocity, pressure, CC value, pitch bend, and
        // anything else that is invertable for all events we receive
        while let Some(event) = context.next_event() {
            let split = self.params.split.value() as u8;

            match event {
                NoteEvent::NoteOn {
                    timing,
                    voice_id,
                    channel: _,
                    note,
                    velocity,
                } => context.send_event(NoteEvent::NoteOn {
                    timing,
                    voice_id,
                    channel: if note <= split { 0 } else { 1 },
                    note,
                    velocity,
                }),
                NoteEvent::NoteOff {
                    timing,
                    voice_id,
                    channel: _,
                    note,
                    velocity,
                } => context.send_event(NoteEvent::NoteOff {
                    timing,
                    voice_id,
                    channel: if note <= split { 0 } else { 1 },
                    note,
                    velocity,
                }),
                NoteEvent::Choke {
                    timing,
                    voice_id,
                    channel: _,
                    note,
                } => context.send_event(NoteEvent::Choke {
                    timing,
                    voice_id,
                    channel: if note <= split { 0 } else { 1 },
                    note,
                }),
                NoteEvent::PolyPressure {
                    timing,
                    voice_id,
                    channel: _,
                    note,
                    pressure,
                } => context.send_event(NoteEvent::PolyPressure {
                    timing,
                    voice_id,
                    channel: if note <= split { 0 } else { 1 },
                    note,
                    pressure,
                }),
                NoteEvent::PolyVolume {
                    timing,
                    voice_id,
                    channel: _,
                    note,
                    gain,
                } => context.send_event(NoteEvent::PolyVolume {
                    timing,
                    voice_id,
                    channel: if note <= split { 0 } else { 1 },
                    note,
                    gain,
                }),
                NoteEvent::PolyPan {
                    timing,
                    voice_id,
                    channel: _,
                    note,
                    pan,
                } => context.send_event(NoteEvent::PolyPan {
                    timing,
                    voice_id,
                    channel: if note <= split { 0 } else { 1 },
                    note,
                    pan,
                }),
                NoteEvent::PolyTuning {
                    timing,
                    voice_id,
                    channel: _,
                    note,
                    tuning,
                } => context.send_event(NoteEvent::PolyTuning {
                    timing,
                    voice_id,
                    channel: if note <= split { 0 } else { 1 },
                    note,
                    tuning,
                }),
                NoteEvent::PolyVibrato {
                    timing,
                    voice_id,
                    channel: _,
                    note,
                    vibrato,
                } => context.send_event(NoteEvent::PolyVibrato {
                    timing,
                    voice_id,
                    channel: if note <= split { 0 } else { 1 },
                    note,
                    vibrato,
                }),
                NoteEvent::PolyExpression {
                    timing,
                    voice_id,
                    channel: _,
                    note,
                    expression,
                } => context.send_event(NoteEvent::PolyExpression {
                    timing,
                    voice_id,
                    channel: if note <= split { 0 } else { 1 },
                    note,
                    expression,
                }),
                NoteEvent::PolyBrightness {
                    timing,
                    voice_id,
                    channel: _,
                    note,
                    brightness,
                } => context.send_event(NoteEvent::PolyBrightness {
                    timing,
                    voice_id,
                    channel: if note <= split { 0 } else { 1 },
                    note,
                    brightness,
                }),
                _ => (),
            }
        }
        ProcessStatus::Normal
    }
}

impl Vst3Plugin for MidiInverter {
    const VST3_CLASS_ID: [u8; 16] = *b"KeyboardSplitter";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Instrument, Vst3SubCategory::Tools];
}

nih_export_vst3!(MidiInverter);
