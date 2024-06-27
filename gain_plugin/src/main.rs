use nih_plug::prelude::*;

use rust_vst_plugin::Gain;

fn main() {
    nih_export_standalone::<Gain>();
}
