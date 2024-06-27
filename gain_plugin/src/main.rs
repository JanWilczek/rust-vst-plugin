use nih_plug::prelude::*;

use gain_plugin::Gain;

fn main() {
    nih_export_standalone::<Gain>();
}
