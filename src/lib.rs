#![no_std]

extern crate alloc;

use alloc::{vec::Vec, boxed::Box};

/// Common Sample Rates
pub enum SampleRate {
    R44100 = 44100,
}

/// Common Bit Depths
pub enum BitDepth {
    B16 = 16,
}

/// Format of audio data
pub struct Format {
    pub sample_rate: SampleRate,
    pub bit_depth: BitDepth,
}

/// A power of 2 sized block of memory containing sampled audio data
pub struct Block {
    pub channels: u8,
    pub format: Format,
    pub buf: Vec<u8>,
}

/// A source of audio in a pipeline
pub trait Source {
    fn source(&mut self, out_buf: &mut Block);
}

/// A sink of audio in a pipeline
pub trait Sink {
    fn sink(&mut self, in_buf: &Block);
}

/// A process in a pipeline of musical processes
pub trait Process {
    /// Process the given input buffer and put results into
    /// the given output. Must be infallible.
    fn process(&mut self, in_buf: &Block, out_buf: &mut Block);
}

/// Audio processing pipeline
pub struct Pipeline {
    pub source: Box<dyn Source>,
    pub sink: Box<dyn Sink>,
    pub processes: Vec<Box<dyn Process>>,
}

impl Pipeline {
    pub fn new(
        source: Box<dyn Source>,
        sink: Box<dyn Sink>,
        processes: Vec<Box<dyn Process>>,
    ) -> Pipeline {
        Pipeline {
            source,
            sink,
            processes,
        }
    }
}
