use pipeline::{Block, Format, Source};

/// Sine wave audio source
pub struct Sine {
    // Amplitude of the sine wave
    amplitude: u32,
    // DC or integer offset for amplitude
    offset: u32,
    // Phase in radians
    phase: u32,
    // Frequency in microhertz
    freq: u32,
    // Current sample (tau)
    t: u64,
}

impl Source for Sine {
    fn source(&mut self, out_buf: &mut Block) {
        out_buf.write(|_idx: u32| -> u32 {

        });
    }
}
